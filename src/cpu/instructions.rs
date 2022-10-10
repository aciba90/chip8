use super::Opcode;

pub enum Instruction {
    Nop,
    Cls,
    Rts,
    Jmp { nnn: u16 },
    Call { nnn: u16 },
    Ske { x: u8, kk: u8 },
    Skne { x: u8, kk: u8 },
    Skre { x: u8, y: u8 },
    Load { x: u8, kk: u8 },
    Add { x: u8, kk: u8 },
    Move { x: u8, y: u8 },
    Or { x: u8, y: u8 },
    And { x: u8, y: u8 },
    Xor { x: u8, y: u8 },

    Skrne { x: u8, y: u8 },
    Loadi { nnn: u16 },
    Rand { x: u8, kk: u8 },
    Draw { x: u8, y: u8, n: u8 },
    Keyd { x: u8 },

    Addi { x: u8 },
    Ldspr { x: u8 },
    Bcd { x: u8 },
    // Stor{x: u8},
    // Read{x: u8},
}

impl Instruction {
    pub fn decode(opcode: Opcode) -> Instruction {
        let (x, y, n, kk, nnn) = opcode.interpret();

        match opcode.nibbles() {
            (0x0, 0x0, 0xE, 0x0) => Instruction::Cls,
            (0x0, 0x0, 0xE, 0xE) => Instruction::Rts,
            (0x0, _, _, _) => Instruction::Nop,
            (0x1, _, _, _) => Instruction::Jmp { nnn },
            (0x2, _, _, _) => Instruction::Call { nnn },
            (0x3, _, _, _) => Instruction::Ske { x, kk },
            (0x4, _, _, _) => Instruction::Skne { x, kk },
            (0x5, _, _, _) => Instruction::Skre { x, y },
            (0x6, _, _, _) => Instruction::Load { x, kk },
            (0x7, _, _, _) => Instruction::Add { x, kk },
            (0x8, _, _, 0x0) => Instruction::Move { x, y },
            (0x8, _, _, 0x1) => Instruction::Or { x, y },
            (0x8, _, _, 0x2) => Instruction::And { x, y },
            (0x8, _, _, 0x3) => Instruction::Xor { x, y },
            // // (0x8, _, _, 0x4) => self.i_8xy4(x, y),
            // // (0x8, _, _, 0x5) => self.i_8xy5(x, y),
            // // (0x8, _, _, 0x6) => self.i_8xy6(x, y),
            // // (0x8, _, _, 0x7) => self.i_8xy7(x, y),
            // // (0x8, _, _, 0xE) => self.i_8xyE(x, y),
            (0x9, _, _, _) => Instruction::Skrne { x, y },
            (0xA, _, _, _) => Instruction::Loadi { nnn },
            // // (0xB, _, _, _) => self.i_bnnn(nnn),
            (0xC, _, _, _) => Instruction::Rand { x, kk },
            (0xD, _, _, _) => Instruction::Draw { x, y, n },
            // // (0xE, _, 0x9, 0xE) => self.i_Ex9E(x),
            // // (0xE, _, 0xA, 0x1) => self.i_ExA1(x),
            // // (0xF, _, 0x0, 0x7) => self.i_Fx07(x),
            (0xF, _, 0x0, 0xA) => Instruction::Keyd { x },
            // // (0xF, _, 0x1, 0x5) => self.i_Fx15(x),
            // // (0xF, _, 0x1, 0x8) => self.i_Fx18(x),
            (0xF, _, 0x1, 0xE) => Instruction::Addi { x },
            (0xF, _, 0x2, 0x9) => Instruction::Ldspr { x },
            (0xF, _, 0x3, 0x3) => Instruction::Bcd { x },
            // (0xF, _, 0x5, 0x5) => Instruction::STOR{x},
            // (0xF, _, 0x6, 0x5) => Instruction::READ{x},
            _ => panic!("Unknown opcode: {}", opcode),
        }
    }
}
