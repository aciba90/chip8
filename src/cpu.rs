mod instructions;

use instructions::Instruction;

use crate::constants::{HEIGHT, WIDTH};
use std::fmt;

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

const RAM_SIZE: usize = 4096;

pub struct Cpu {
    ram: [u8; RAM_SIZE],
    stack: [u16; 16],

    vram: [[bool; HEIGHT]; WIDTH],
    vram_changed: bool,

    // Registers
    v: [u8; 16],
    i: u16,

    _delay_timer: u8,
    _sound_timer: u8,

    pc: u16,
    sp: u8,
    // I/O
    // keyboard: Keyboard
}

impl Default for Cpu {
    fn default() -> Cpu {
        Cpu {
            ram: [0; RAM_SIZE],
            stack: [0; 16],
            vram: [[false; HEIGHT]; WIDTH],
            vram_changed: false,
            v: [0; 16],
            i: 0,
            _delay_timer: 0,
            _sound_timer: 0,
            pc: 0x200,
            sp: 0,
        }
    }
}

impl Cpu {
    /// Load fonts in RAM (from 0x000 to 0x1FF)
    fn load_fonts(&mut self) {
        for (i, byte) in FONTS.iter().enumerate() {
            self.ram[i] = *byte;
        }
    }

    pub fn load_rom(&mut self, bytecode: &[u8]) {
        self.load_fonts();
        // TODO check out of bounds
        for (index, val) in bytecode.iter().enumerate() {
            self.ram[0x200 + index] = *val;
        }
    }

    pub fn refresh_screen(&self) -> bool {
        self.vram_changed
    }

    pub fn vram(&self) -> &[[bool; HEIGHT]; WIDTH] {
        &self.vram
    }

    pub fn tick(&mut self) {
        let opcode: Opcode = self.ram[self.pc as usize..(self.pc + 2) as usize]
            .try_into()
            .unwrap();
        println!("Opcode {}", &opcode);
        let instruction = Instruction::decode(opcode);
        self.run_instruction(&instruction);
    }

    fn run_instruction(&mut self, instruction: &Instruction) {
        let jump = match *instruction {
            Instruction::Nop => None,
            Instruction::Cls => self.i_00e0(),
            Instruction::Rts => self.i_00ee(),
            Instruction::Jmp { nnn } => self.i_1nnn(nnn),
            Instruction::Call { nnn } => self.i_2nnn(nnn),
            Instruction::Ske { x, kk } => self.i_3xkk(&x, &kk),
            Instruction::Skne { x, kk } => self.i_4xkk(&x, &kk),
            Instruction::Skre { x, y } => self.i_5xy0(&x, &y),
            Instruction::Load { x, kk } => self.i_6xkk(&x, kk),
            Instruction::Add { x, kk } => self.i_7xkk(&x, &kk),
            Instruction::Move { x, y } => self.i_8xy0(&x, &y),
            Instruction::Or { x, y } => self.i_8xy1(&x, &y),
            Instruction::And { x, y } => self.i_8xy2(&x, &y),
            Instruction::Xor { x, y } => self.i_8xy3(&x, &y),
            Instruction::Addr { x, y } => self.i_8xy4(&x, &y),
            Instruction::Sub { x, y } => self.i_8xy5(&x, &y),
            Instruction::Shr { x, y } => self.i_8xy6(&x, &y),
            Instruction::Shl { x, y } => self.i_8xyE(&x, &y),
            Instruction::Skrne { x, y } => self.i_9xy0(&x, &y),
            Instruction::Loadi { nnn } => self.i_annn(nnn),
            Instruction::Jumpi { nnn } => self.i_bnnn(nnn),
            Instruction::Rand { x, kk } => self.i_cxkk(&x, &kk),
            Instruction::Draw { x, y, n } => self.i_dxyn(&x, &y, &n),
            Instruction::Skpr { x } => self.i_ex9e(x),
            Instruction::Skup { x } => self.i_exa1(x),
            Instruction::Moved { x } => self.i_fx07(x),
            Instruction::Keyd { x } => self.i_fx0a(x),
            Instruction::LOADD { x } => self.i_fx15(&x),
            Instruction::LOADS { x } => self.i_fx18(&x),
            Instruction::Addi { x } => self.i_fx1e(&x),
            Instruction::Ldspr { x } => self.i_fx29(&x),
            Instruction::Bcd { x } => self.i_fx33(&x),
            Instruction::Stor { x } => self.i_fx55(&x),
            Instruction::Read { x } => self.i_fx65(&x),
        };

        match jump.unwrap_or(PC::Advance(1)) {
            PC::Advance(i) => self.pc += 2 * i,
            PC::Jump(nnn) => self.pc = nnn,
        };
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
    fn i_00e0(&mut self) -> Option<PC> {
        self.vram = [[false; HEIGHT]; WIDTH];
        self.vram_changed = true;

        None
    }

    /// 00EE - RET
    /// Return from a subroutine.
    ///
    /// The interpreter sets the program counter to the address at the top of the stack,
    /// then subtracts 1 from the stack pointer.
    fn i_00ee(&mut self) -> Option<PC> {
        self.pc = self.stack[self.sp as usize - 1];
        self.sp -= 1;

        None
    }

    /// 1nnn - JP addr
    /// Jump to location nnn.
    ///
    /// The interpreter sets the program counter to nnn.
    fn i_1nnn(&mut self, nnn: u16) -> Option<PC> {
        Some(PC::Jump(nnn))
    }

    /// 2NNN
    ///
    /// Execute subroutine starting at address NNN
    fn i_2nnn(&mut self, nnn: u16) -> Option<PC> {
        // push PC to the stack to return later
        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;

        // call the subroutine
        Some(PC::Jump(nnn))
    }

    /// 3xkk - SE Vx, byte
    /// Skip next instruction if Vx = kk.
    ///
    /// The interpreter compares register Vx to kk, and if they are equal, increments
    /// the program counter by 2.
    fn i_3xkk(&mut self, x: &u8, kk: &u8) -> Option<PC> {
        if self.v[*x as usize] == *kk {
            return Some(PC::Advance(2));
        }
        None
    }

    /// Skip the following instruction if the value of register VX is not equal to NN
    fn i_4xkk(&mut self, x: &u8, kk: &u8) -> Option<PC> {
        if self.v[*x as usize] == *kk {
            return None;
        }
        Some(PC::Advance(2))
    }

    /// Skip the following instruction if the value of register VX is equal to the
    /// value of register VY
    fn i_5xy0(&mut self, x: &u8, y: &u8) -> Option<PC> {
        if self.v[*x as usize] != self.v[*y as usize] {
            return None;
        }
        Some(PC::Advance(2))
    }

    /// 6xkk - LD Vx, byte
    /// Set Vx = kk.
    ///
    /// The interpreter puts the value kk into register Vx.
    fn i_6xkk(&mut self, x: &u8, kk: u8) -> Option<PC> {
        self.v[*x as usize] = kk;
        None
    }

    /// Set Vx = Vx + kk.
    ///
    /// Adds the value kk to the value of register Vx, then stores the result in Vx.
    fn i_7xkk(&mut self, x: &u8, kk: &u8) -> Option<PC> {
        self.v[*x as usize] = self.v[*x as usize].wrapping_add(*kk);
        None
    }

    /// Store the value of register VY in register VX
    fn i_8xy0(&mut self, x: &u8, y: &u8) -> Option<PC> {
        self.v[*x as usize] = self.v[*y as usize];
        None
    }

    /// Set VX to VX OR VY
    fn i_8xy1(&mut self, x: &u8, y: &u8) -> Option<PC> {
        self.v[*x as usize] |= self.v[*y as usize];
        None
    }

    /// Set VX to VX AND VY
    fn i_8xy2(&mut self, x: &u8, y: &u8) -> Option<PC> {
        self.v[*x as usize] &= self.v[*y as usize];
        None
    }

    /// Set VX to VX XOR VY
    fn i_8xy3(&mut self, x: &u8, y: &u8) -> Option<PC> {
        self.v[*x as usize] ^= self.v[*y as usize];
        None
    }

    /// ADD VX, VY
    ///
    /// Set VX equal to VX plus VY. In the case of an overflow VF is set to 1.
    /// Otherwise 0.
    fn i_8xy4(&mut self, x: &u8, y: &u8) -> Option<PC> {
        match self.v[*x as usize].checked_add(self.v[*y as usize]) {
            None => self.v[0xF] = 1,
            Some(_) => self.v[0xF] = 0,
        }

        self.v[*x as usize] = self.v[*x as usize].wrapping_add(self.v[*y as usize]);

        None
    }

    /// SUB VX, VY
    ///
    /// Set VX equal to VX minus VY. In the case of an underflow VF is set 0.
    /// Otherwise 1. (VF = VX > VY)
    fn i_8xy5(&mut self, x: &u8, y: &u8) -> Option<PC> {
        self.v[0xF] = (self.v[*x as usize] > self.v[*y as usize]) as u8;

        self.v[*x as usize] = self.v[*x as usize].wrapping_sub(self.v[*y as usize]);

        None
    }

    fn i_8xy6(&mut self, x: &u8, y: &u8) -> Option<PC> {
        todo!()
    }
    fn i_8xyE(&mut self, x: &u8, y: &u8) -> Option<PC> {
        todo!()
    }

    /// Skip the following instruction if the value of register VX is not equal to the
    /// value of register VY
    fn i_9xy0(&mut self, x: &u8, y: &u8) -> Option<PC> {
        if self.v[*x as usize] == self.v[*y as usize] {
            return None;
        }
        Some(PC::Advance(2))
    }

    /// Annn - LD I, addr
    /// Set I = nnn.
    ///
    /// The value of register I is set to nnn.
    fn i_annn(&mut self, nnn: u16) -> Option<PC> {
        self.i = nnn;
        None
    }

    fn i_bnnn(&mut self, nnn: u16) -> Option<PC> {
        todo!()
    }

    /// Set VX to a random number with a mask of kk
    fn i_cxkk(&mut self, x: &u8, kk: &u8) -> Option<PC> {
        self.v[*x as usize] = utils::random_byte() & *kk;
        None
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
    fn i_dxyn(&mut self, x: &u8, y: &u8, n: &u8) -> Option<PC> {
        let vx = ((self.v[*x as usize] as usize) % WIDTH) as usize;
        let vy = ((self.v[*y as usize] as usize) % HEIGHT) as usize;

        self.v[0xF] = 0;
        for jj in 0..*n {
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

        None
    }

    fn i_ex9e(&mut self, _x: u8) -> Option<PC> {
        todo!()
    }
    fn i_exa1(&mut self, _x: u8) -> Option<PC> {
        todo!()
    }
    fn i_fx07(&mut self, _x: u8) -> Option<PC> {
        todo!()
    }

    /// Fx0A - LD Vx, K
    /// Wait for a key press, store the value of the key in Vx.
    ///
    /// All execution stops until a key is pressed, then the value of that key
    /// is stored in Vx.
    fn i_fx0a(&mut self, _x: u8) -> Option<PC> {
        todo!("get keys");
    }

    fn i_fx15(&mut self, x: &u8) -> Option<PC> {
        todo!()
    }
    fn i_fx18(&mut self, x: &u8) -> Option<PC> {
        todo!()
    }

    /// Fx1E - ADD I, Vx
    /// Set I = I + Vx.
    ///
    /// The values of I and Vx are added, and the results are stored in I.
    fn i_fx1e(&mut self, x: &u8) -> Option<PC> {
        self.i += self.v[*x as usize] as u16;
        None
    }

    /// Fx29 - LD F, Vx
    /// Set I = location of sprite for digit Vx.
    ///
    /// The value of I is set to the location for the hexadecimal sprite
    /// corresponding to the value of Vx.
    /// See section 2.4, Display, for more information on the Chip-8
    /// hexadecimal font.
    fn i_fx29(&mut self, x: &u8) -> Option<PC> {
        self.i = (self.v[*x as usize] * 5) as u16; // 5 is the len of a digit
        None
    }

    /// FX33
    ///
    /// Store the binary-coded decimal equivalent of the value stored in register VX at
    /// addresses I, I+1, and I+2.
    fn i_fx33(&mut self, x: &u8) -> Option<PC> {
        let mut byte = self.v[*x as usize];

        // first figure
        self.ram[self.i as usize + 2] = byte.rem_euclid(10);

        // second figure
        byte /= 10;
        self.ram[self.i as usize + 1] = byte.rem_euclid(10);

        // third figure
        byte /= 10;
        self.ram[self.i as usize] = byte.rem_euclid(10);

        None
    }

    fn i_fx55(&mut self, x: &u8) -> Option<PC> {
        todo!()
    }
    fn i_fx65(&mut self, x: &u8) -> Option<PC> {
        todo!()
    }
}

#[derive(Debug)]
pub struct Opcode([u8; 2]);

impl Opcode {
    pub fn nibbles(&self) -> (u8, u8, u8, u8) {
        (
            self.0[0] >> 4,
            self.0[0] & 0x0F,
            self.0[1] >> 4,
            self.0[1] & 0x0F,
        )
    }

    pub fn interpret(&self) -> (u8, u8, u8, u8, u16) {
        let nibbles = self.nibbles();

        // A 4-bit value, the lower 4 bits of the high byte of the instruction
        let x = nibbles.1;
        // A 4-bit value, the upper 4 bits of the low byte of the instruction
        let y = nibbles.2;
        // A 4-bit value, the lowest 4 bits of the instruction
        let n = nibbles.3;
        // An 8-bit value, the lowest 8 bits of the instruction
        let kk = self.0[1];
        // A 12-bit value, the lowest 12 bits of the instruction
        let nnn = (((self.0[0] as u16) << 8) | self.0[1] as u16) & 0x0FFF;

        (x, y, n, kk, nnn)
    }
}

impl TryFrom<&[u8]> for Opcode {
    type Error = &'static str;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let raw_opcode: [u8; 2] = value.try_into().unwrap();
        Ok(Opcode(raw_opcode))
    }
}

impl TryFrom<&[u16]> for Opcode {
    type Error = &'static str;

    fn try_from(value: &[u16]) -> Result<Self, Self::Error> {
        let x = value[0].to_be_bytes();
        Ok(Opcode(x))
    }
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02X}{:02X}", self.0[0], self.0[1])
    }
}

enum PC {
    Advance(u16), // number of positions
    Jump(u16),
}

mod utils {
    pub fn random_byte() -> u8 {
        rand::random()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_cpu() -> Cpu {
        let mut cpu = Cpu::default();
        cpu.load_fonts();

        cpu
    }

    #[test]
    fn clear_screen_00e0() {
        let mut cpu = create_cpu();
        cpu.vram[2][4] |= true;

        cpu.i_00e0();

        for pixel in cpu.vram.iter().flatten() {
            assert_eq!(*pixel, false, "All pixels should have been cleared");
            assert!(cpu.vram_changed, "Screen must be updated");
        }
    }

    /// if v[x] != nn => skip following instruction
    #[test]
    fn test_4xnn_skip_instruction() {
        let mut cpu = create_cpu();
        assert_eq!(cpu.pc, 0x200);

        // fifth register is not 0x2A
        cpu.v[5] = 2;
        let rom: &[u8] = &[0x45, 0x2A];
        cpu.load_rom(rom);

        cpu.tick();

        assert_eq!(cpu.pc, 0x204);
    }

    /// if v[x] == nn => don't skip following instruction
    #[test]
    fn test_4xnn_do_not_skip_instruction() {
        let mut cpu = create_cpu();
        assert_eq!(cpu.pc, 0x200);
        cpu.v[5] = 0x2A;
        let rom: &[u8] = &[0x45, 0x2A];
        cpu.load_rom(rom);

        cpu.tick();

        assert_eq!(cpu.pc, 0x202);
    }

    /// if v[x] == v[y] => skip following instruction
    #[test]
    fn test_5xy0_skip_instruction() {
        let mut cpu = create_cpu();
        assert_eq!(cpu.pc, 0x200);

        cpu.v[5] = 2;
        cpu.v[4] = 2;
        let rom: &[u8] = &[0x54, 0x50];
        cpu.load_rom(rom);

        cpu.tick();

        assert_eq!(cpu.pc, 0x204);
    }

    /// if v[x] != v[y] => do not skip following instruction
    #[test]
    fn test_5xy0_do_skip_instruction() {
        let mut cpu = create_cpu();
        assert_eq!(cpu.pc, 0x200);

        cpu.v[5] = 2;
        cpu.v[4] = 5;
        let rom: &[u8] = &[0x54, 0x50];
        cpu.load_rom(rom);

        cpu.tick();

        assert_eq!(cpu.pc, 0x202);
    }

    // Store the value of register VY in register VX
    #[test]
    fn test_8xy0() {
        let mut cpu = create_cpu();
        cpu.v[5] = 2;
        let rom: &[u8] = &[0x84, 0x50];
        cpu.load_rom(rom);
        cpu.tick();
        assert_eq!(cpu.v[4], 2);
        assert_eq!(cpu.v[5], 2);
        assert_eq!(cpu.pc, 0x202);
    }

    /// Set VX to VX OR VY
    #[test]
    fn test_8xy1() {
        let mut cpu = create_cpu();
        cpu.v[4] = 0b1001;
        cpu.v[5] = 0b1010;
        let rom: &[u8] = &[0x84, 0x51];
        cpu.load_rom(rom);

        cpu.tick();

        assert_eq!(cpu.v[4], 0b1011);
        assert_eq!(cpu.v[5], 0b1010);
        assert_eq!(cpu.pc, 0x202);
    }

    /// Set VX to VX AND VY
    #[test]
    fn test_8xy2() {
        let mut cpu = create_cpu();
        cpu.v[4] = 0b1001;
        cpu.v[5] = 0b1010;
        let rom: &[u8] = &[0x84, 0x52];
        cpu.load_rom(rom);

        cpu.tick();

        assert_eq!(cpu.v[4], 0b1000);
        assert_eq!(cpu.v[5], 0b1010);
        assert_eq!(cpu.pc, 0x202);
    }

    /// Set VX to VX XOR VY
    #[test]
    fn test_8xy3() {
        let mut cpu = create_cpu();
        cpu.v[4] = 0b1001;
        cpu.v[5] = 0b1010;
        let rom: &[u8] = &[0x84, 0x53];
        cpu.load_rom(rom);

        cpu.tick();

        assert_eq!(cpu.v[4], 0b0011);
        assert_eq!(cpu.v[5], 0b1010);
        assert_eq!(cpu.pc, 0x202);
    }

    /// if v[x] != v[y] => skip following instruction
    #[test]
    fn test_9xy0_skip_instruction() {
        let mut cpu = create_cpu();
        cpu.load_fonts();
        assert_eq!(cpu.pc, 0x200);

        cpu.v[5] = 2;
        cpu.v[4] = 5;
        let rom: &[u8] = &[0x94, 0x50];
        cpu.load_rom(rom);

        cpu.tick();

        assert_eq!(cpu.pc, 0x204);
    }

    /// if v[x] == v[y] => do not skip following instruction
    #[test]
    fn test_9xy0_do_skip_instruction() {
        let mut cpu = create_cpu();
        cpu.load_fonts();
        assert_eq!(cpu.pc, 0x200);

        cpu.v[5] = 2;
        cpu.v[4] = 2;
        let rom: &[u8] = &[0x94, 0x50];
        cpu.load_rom(rom);

        cpu.tick();

        assert_eq!(cpu.pc, 0x202);
    }
}
