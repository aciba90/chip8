use chip8::Chip8;

pub fn main() {
    let mut chip8 = Chip8::new(16);
    chip8.run("roms/TETRIS");
}
