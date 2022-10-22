use chip8::args::Args;
use chip8::Chip8;
use clap::Parser;

pub fn main() {
    env_logger::init();

    let args = Args::parse();
    let mut chip8 = Chip8::new(args.scale);
    chip8.run(&args.rom);
}
