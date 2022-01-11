pub fn inst_array_to_u16(inst: [u8; 2]) -> u16 {
    ((inst[0] as u16) << 8) | inst[1] as u16
}

pub fn format_instruction(inst: [u8; 2]) -> String {
    format!("{:02X}{:02X}", inst[0], inst[1])
}
