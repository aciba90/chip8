mod cpu;
mod utils;
extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::env;
use std::fs;
use std::process;
use std::time::Duration;

const WIDTH: usize = 64;
const HEIGHT: usize = 32;
const SCALE: usize = 4;

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

#[derive(Default)]
pub struct Chip8 {
    cpu: cpu::Cpu,
    // TODO keyboard
    /*
    1	2	3	C
    4	5	6	D
    7	8	9	E
    A	0	B	F
    */
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 {
            ..Default::default()
        }
    }

    pub fn init(&mut self) {
        self.cpu.init();
    }

    pub fn load_rom(&mut self, filename: String) {
        let rom: Vec<u8> = fs::read(filename).expect("No file found");
        self.cpu.load_program(rom);
    }

    pub fn load_program(&mut self, bytecode: Vec<u8>) {
        self.cpu.load_program(bytecode);
    }

    pub fn run(&mut self) {
        loop {
            self.cpu.tick();
            if self.cpu.vram_changed {
                // self.display.into_inner().draw(self.cpu.vram);
            }
        }
    }
}

pub fn exec_sdl() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let mut chip8 = Chip8::new();
    chip8.init();
    chip8.load_rom(config.filename);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window(
            "rust-sdl2 demo",
            (SCALE * WIDTH) as u32,
            (SCALE * HEIGHT) as u32,
        )
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        chip8.cpu.tick();
        if chip8.cpu.vram_changed {
            canvas.set_draw_color(Color::RGB(255, 255, 255));
            for y in 0..HEIGHT {
                let yy = y * SCALE;
                for x in 0..WIDTH {
                    let xx = x * SCALE;
                    if chip8.cpu.vram[x][y] {
                        for i in 0..SCALE {
                            for j in 0..SCALE {
                                canvas
                                    .draw_point(sdl2::rect::Point::new(
                                        (xx + i) as i32,
                                        (yy + j) as i32,
                                    ))
                                    .unwrap();
                            }
                        }
                    }
                }
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
