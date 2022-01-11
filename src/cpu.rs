use crate::utils;
const WIDTH: usize = 64;
const HEIGHT: usize = 32;

const FONTS: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct Cpu {
    // Memory
    ram: [u8; 4096],   // 4k RAM
    _stack: [u16; 16], // 16 16-bit values.

    // VRAM
    pub vram: [[bool; HEIGHT]; WIDTH],
    pub vram_changed: bool,

    // Registers
    v: [u8; 16], // 16 8-bit registers
    i: u16,      // 16-bit I register.

    _delay_timer: u8, // 8-bit delay register.
    _sound_timer: u8, // 8-bit sound register.

    program_counter: u16, // 16-bit program counter.
    update_pc: bool,

    _stack_pointer: u8, // 8-bit program counter.

                        // I/O
                        // keyboard: Keyboard,
}

impl Default for Cpu {
    fn default() -> Cpu {
        Cpu {
            ram: [0; 4096],
            _stack: [0; 16],
            vram: [[false; HEIGHT]; WIDTH],
            vram_changed: false,
            v: [0; 16],
            i: 0,
            _delay_timer: 0,
            _sound_timer: 0,
            program_counter: 0x200,
            update_pc: true,
            _stack_pointer: 0,
        }
    }
}

impl Cpu {
    pub fn init(&mut self) {
        self.load_fonts();
    }

    /// Load fonts in RAM (from 0x000 to 0x1FF)
    fn load_fonts(&mut self) {
        for (i, byte) in FONTS.iter().enumerate() {
            self.ram[i] = *byte;
        }
    }

    pub fn load_program(&mut self, bytecode: Vec<u8>) {
        for (index, val) in bytecode.iter().enumerate() {
            self.ram[0x200 + index] = *val;
        }
    }

    pub fn tick(&mut self) {
        self.update_pc = true;
        let inst: Vec<u8> =
            self.ram[self.program_counter as usize..(self.program_counter + 2) as usize].to_vec();
        let instruction: [u8; 2] = inst.try_into().unwrap();
        println!("Instruction {}", utils::format_instruction(instruction)); // TODO print hex
        self.run_instruction(instruction);
        if self.update_pc {
            self.program_counter += 2
        }
    }

    fn run_instruction(&mut self, instruction: [u8; 2]) {
        let nibbles = (
            instruction[0] >> 4,
            instruction[0] & 0x0F,
            instruction[1] >> 4,
            instruction[1] & 0x0F,
        );
        let x = nibbles.1; // A 4-bit value, the lower 4 bits of the high byte of the instruction
        let y = nibbles.2; // A 4-bit value, the upper 4 bits of the low byte of the instruction
        let n = nibbles.3; // A 4-bit value, the lowest 4 bits of the instruction
        let kk = instruction[1]; // An 8-bit value, the lowest 8 bits of the instruction
        let nnn = utils::inst_array_to_u16(instruction) & 0x0FFF; // A 12-bit value, the lowest 12 bits of the instruction
        match nibbles {
            (0x0, 0x0, 0xE, 0x0) => self.i_00e0(),
            (0x0, 0x0, 0xE, 0xE) => self.i_00ee(),
            (0x0, _, _, _) => self.i_0nnn(nnn),
            (0x1, _, _, _) => self.i_1nnn(nnn),
            // (0x2, _, _, _) => self.i_2nnn(nnn),
            (0x3, _, _, _) => self.i_3xkk(x, kk),
            // (0x4, _, _, _) => self.i_4xkk(x, kk),
            // (0x5, _, _, _) => self.i_5xy0(x, y),
            (0x6, _, _, _) => self.i_6xkk(x, kk),
            (0x7, _, _, _) => self.i_7xkk(x, kk),
            // (0x8, _, _, 0x0) => self.i_8xy0(x, y),
            // (0x8, _, _, 0x1) => self.i_8xy1(x, y),
            // (0x8, _, _, 0x2) => self.i_8xy2(x, y),
            // (0x8, _, _, 0x3) => self.i_8xy3(x, y),
            // (0x8, _, _, 0x4) => self.i_8xy4(x, y),
            // (0x8, _, _, 0x5) => self.i_8xy5(x, y),
            // (0x8, _, _, 0x6) => self.i_8xy6(x, y),
            // (0x8, _, _, 0x7) => self.i_8xy7(x, y),
            // (0x8, _, _, 0xE) => self.i_8xyE(x, y),
            // (0x9, _, _, _) => self.i_9xy0(x, y),
            (0xA, _, _, _) => self.i_annn(nnn),
            // (0xB, _, _, _) => self.i_bnnn(nnn),
            // (0xC, _, _, _) => self.i_cxkk(x, kk),
            (0xD, _, _, _) => self.i_dxyn(x, y, n),
            // (0xE, _, 0x9, 0xE) => self.i_Ex9E(x),
            // (0xE, _, 0xA, 0x1) => self.i_ExA1(x),
            // (0xF, _, 0x0, 0x7) => self.i_Fx07(x),
            (0xF, _, 0x0, 0xA) => self.i_fx0a(x),
            // (0xF, _, 0x1, 0x5) => self.i_Fx15(x),
            // (0xF, _, 0x1, 0x8) => self.i_Fx18(x),
            (0xF, _, 0x1, 0xE) => self.i_fx1e(x),
            (0xF, _, 0x2, 0x9) => self.i_fx29(x),
            // (0xF, _, 0x3, 0x3) => self.i_Fx33(x),
            // (0xF, _, 0x5, 0x5) => self.i_Fx55(x),
            // (0xF, _, 0x6, 0x5) => self.i_Fx65(x),
            _ => panic!("Skipping unknown instruction: {:x?}", instruction),
        }
    }
}

// instructions
impl Cpu {
    #[allow(dead_code)]
    fn i_sys(self) {
        unimplemented!();
    }

    /// 0x00E0 - CLS
    /// Clear the display.
    fn i_00e0(&mut self) {
        self.vram = [[false; HEIGHT]; WIDTH];
        self.vram_changed = true
    }

    /// 00EE - RET
    /// Return from a subroutine.
    ///
    /// The interpreter sets the program counter to the address at the top of the stack,
    /// then subtracts 1 from the stack pointer.
    fn i_00ee(&mut self) {
        unimplemented!();
    }

    /// 0nnn - SYS addr
    /// Jump to a machine code routine at nnn.
    ///
    /// This instruction is only used on the old computers on which Chip-8 was
    /// originally implemented. It is ignored by modern interpreters.
    fn i_0nnn(&mut self, _nnn: u16) {}

    /// 1nnn - JP addr
    /// Jump to location nnn.
    ///
    /// The interpreter sets the program counter to nnn.
    fn i_1nnn(&mut self, nnn: u16) {
        self.program_counter = nnn;
        self.update_pc = false;
    }

    /// 3xkk - SE Vx, byte
    /// Skip next instruction if Vx = kk.
    ///
    /// The interpreter compares register Vx to kk, and if they are equal, increments
    /// the program counter by 2.
    fn i_3xkk(&mut self, x: u8, kk: u8) {
        if self.v[x as usize] == self.v[kk as usize] {
            self.program_counter += 4;
            self.update_pc = false;
        }
    }

    /// 6xkk - LD Vx, byte
    /// Set Vx = kk.
    ///
    /// The interpreter puts the value kk into register Vx.
    fn i_6xkk(&mut self, x: u8, kk: u8) {
        self.v[x as usize] = kk;
    }

    /// Set Vx = Vx + kk.
    ///
    /// Adds the value kk to the value of register Vx, then stores the result in Vx.
    fn i_7xkk(&mut self, x: u8, kk: u8) {
        self.v[x as usize] += kk;
    }

    /// Annn - LD I, addr
    /// Set I = nnn.
    ///
    /// The value of register I is set to nnn.
    fn i_annn(&mut self, nnn: u16) {
        self.i = nnn;
    }

    /// Display n-byte sprite starting at memory location I at (Vx, Vy),
    /// set VF = collision.
    ///
    /// The interpreter reads n bytes from memory, starting at the address stored in I.
    /// These bytes are then displayed as sprites on screen at coordinates (Vx, Vy).
    /// Sprites are XORed onto the existing screen.
    /// If this causes any pixels to be erased, VF is set to 1, otherwise it is set to 0.
    /// If the sprite is positioned so part of it is outside the coordinates of the
    /// display, it wraps around to the opposite side of the screen.
    /// See instruction 8xy3 for more information on XOR, and section 2.4, Display
    /// for more information on the Chip-8 screen and sprites.
    fn i_dxyn(&mut self, x: u8, y: u8, n: u8) {
        let vx = ((self.v[x as usize] as usize) % WIDTH) as usize;
        let vy = ((self.v[y as usize] as usize) % HEIGHT) as usize;

        self.v[0xF] = 0;
        for jj in 0..n {
            let yy = (vy + jj as usize) % HEIGHT;
            let byte_ii = self.ram[(self.i + (jj as u16)) as usize];
            for ii in 0..8 {
                let xx = (vx + ii as usize) % WIDTH;
                let pixel_new = ((byte_ii >> (7 - ii)) & 0x01) != 0;
                let pixel = self.vram[xx][yy];

                self.v[0xF] |= (pixel_new & pixel) as u8;
                self.vram[xx][yy] ^= pixel_new;
            }
        }
    }

    /// Fx0A - LD Vx, K
    /// Wait for a key press, store the value of the key in Vx.
    ///
    /// All execution stops until a key is pressed, then the value of that key
    /// is stored in Vx.
    fn i_fx0a(&mut self, _x: u8) {
        // TODO get keys
        self.update_pc = false;
    }

    /// Fx1E - ADD I, Vx
    /// Set I = I + Vx.
    ///
    /// The values of I and Vx are added, and the results are stored in I.
    fn i_fx1e(&mut self, x: u8) {
        self.i += self.v[x as usize] as u16;
    }

    /// Fx29 - LD F, Vx
    /// Set I = location of sprite for digit Vx.
    ///
    /// The value of I is set to the location for the hexadecimal sprite
    /// corresponding to the value of Vx.
    /// See section 2.4, Display, for more information on the Chip-8
    /// hexadecimal font.
    fn i_fx29(&mut self, x: u8) {
        self.i = (self.v[x as usize] * 5) as u16 // 5 is the len of a digit
    }
}

impl Cpu {
    #[allow(dead_code)]
    fn print_vram(&self) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if self.vram[x][y] {
                    print!("*");
                } else {
                    print!("_");
                }
            }
            println!("|");
        }
    }
}
