use sdl2::event::Event;
use sdl2::keyboard::Keycode;

/// Represents available keyboards keys plus the exit one.
///
/// 1	2	3	C
/// 4	5	6	D
/// 7	8	9	E
/// A	0	B	F
///
pub enum Key {
    Exit,
}

pub struct Keyboard {
    event_pump: sdl2::EventPump,
}

impl Keyboard {
    pub fn new(sdl_context: &sdl2::Sdl) -> Keyboard {
        let event_pump = sdl_context.event_pump().unwrap();

        Keyboard { event_pump }
    }

    /// Return pressed keys since the last call
    pub fn pressed_keys(&mut self) -> Vec<Key> {
        let mut keys = vec![];

        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    // XXX log message
                    println!("Quitting screen...");
                    keys.push(Key::Exit);
                }
                _ => {}
            };
        }

        keys
    }
}
