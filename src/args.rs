use clap::Parser;

/// CHIP8 emulator
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// ROM to play
    #[arg()]
    pub rom: String,

    /// Screen scale multiplier
    #[arg(long, default_value_t = 16)]
    pub scale: u8,
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Args::command().debug_assert()
}
