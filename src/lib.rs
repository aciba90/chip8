pub mod args;
mod constants;
mod cpu;
mod keyboard;
mod screen;

extern crate sdl2;

use std::fs;
use std::time::Duration;

// TODO: #[derive(Default)]
pub struct Chip8 {
    cpu: cpu::Cpu,
    screen: screen::Screen,
    keyboard: keyboard::Keyboard,
}

impl Chip8 {
    pub fn new(scale: u8) -> Chip8 {
        let sdl_context = sdl2::init().unwrap();
        let screen = screen::Screen::new(&sdl_context, scale as usize);
        let keyboard = keyboard::Keyboard::new(&sdl_context);

        Chip8 {
            cpu: cpu::Cpu::default(),
            screen,
            keyboard,
        }
    }

    pub fn run(&mut self, rom: &str) {
        let rom: Vec<u8> = fs::read(rom).expect("No file found");
        self.cpu.load_rom(&rom);

        'main: loop {
            let pressed_keys = self.keyboard.pressed_keys();

            for key in pressed_keys.iter() {
                if let keyboard::Key::Exit = key {
                    log::info!("Exit key pressed...");
                    break 'main;
                };
            }

            self.cpu.decrease_timers();
            self.cpu.tick(pressed_keys);
            if self.cpu.refresh_screen() {
                self.screen.tick(self.cpu.vram());
            }
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}
