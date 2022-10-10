extern crate sdl2;

use crate::constants::{HEIGHT, WIDTH};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

pub struct Screen {
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    event_pump: sdl2::EventPump,
    pub running: bool,
    scale: usize,
}

impl Screen {
    pub fn new(scale: usize) -> Screen {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("CHIP8", (scale * WIDTH) as u32, (scale * HEIGHT) as u32)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(Color::RGB(0, 255, 255));
        canvas.clear();
        canvas.present();
        let event_pump = sdl_context.event_pump().unwrap();

        Screen {
            canvas,
            event_pump,
            running: true,
            scale,
        }
    }

    pub fn tick(&mut self, buffer: [[bool; 32]; 64]) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    println!("Quitting...");
                    self.running = false;
                }
                _ => {}
            }
        }

        self.draw(buffer);

        self.canvas.present();
    }

    // XXX: A bit coupled with vram layout
    fn draw(&mut self, buffer: [[bool; 32]; 64]) {
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        for y in 0..HEIGHT {
            let yy = y * self.scale;
            for (x, _) in buffer.iter().enumerate().take(WIDTH) {
                let xx = x * self.scale;
                if buffer[x][y] {
                    for i in 0..self.scale {
                        for j in 0..self.scale {
                            self.canvas
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
}
