pub mod args;
mod constants;
mod cpu;
mod screen;

extern crate sdl2;

use std::fs;
use std::time::Duration;

// TODO: #[derive(Default)]
pub struct Chip8 {
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
    pub fn new(scale: u8) -> Chip8 {
        let screen = screen::Screen::new(scale as usize);
        Chip8 {
            cpu: cpu::Cpu::default(),
            screen,
        }
    }

    pub fn run(&mut self, rom: &str) {
        let rom: Vec<u8> = fs::read(rom).expect("No file found");
        self.cpu.load_rom(&rom);

        while self.screen.running {
            self.cpu.tick();
            if self.cpu.refresh_screen() {
                self.screen.tick(self.cpu.vram());
            }
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}
