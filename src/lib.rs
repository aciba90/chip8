mod constants;
mod cpu;
pub mod screen;
mod utils;
extern crate sdl2;

use std::env;
use std::time::Duration;

pub struct Config {
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        let filename = args[1].clone();
        Ok(Config { filename })
    }
}

// TODO: #[derive(Default)]
pub struct Chip8 {
    config: Config,
    cpu: cpu::Cpu,
    screen: screen::Screen,
    // TODO keyboard
    /*
    1	2	3	C
    4	5	6	D
    7	8	9	E
    A	0	B	F
    */
}

impl Chip8 {
    pub fn new(config: Config) -> Chip8 {
        let screen = screen::Screen::new(4);
        Chip8 {
            config,
            cpu: cpu::Cpu::default(),
            screen: screen,
        }
    }

    pub fn init(&mut self) {
        let filename = self.config.filename.clone();
        self.cpu.init(&filename);
    }

    pub fn run(&mut self) {
        while self.screen.running {
            self.cpu.tick();
            if self.cpu.vram_changed {
                self.screen.tick(self.cpu.vram);
            }
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}

pub fn exec_chip8() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap();
    let mut chip8 = Chip8::new(config);
    chip8.init();
    chip8.run();
}
