use sdl2::event::Event;
use sdl2::keyboard::Keycode;

/// Represents available keyboards keys plus the exit one.
///
/// 1  2  3  C
/// 4  5  6  D
/// 7  8  9  E
/// A  0  B  F
///
pub enum Key {
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    A,
    B,
    C,
    D,
    E,
    F,
    Exit,
}

impl From<Key> for u8 {
    fn from(key: Key) -> u8 {
        match key {
            Key::Num0 => 0,
            Key::Num1 => 1,
            Key::Num2 => 2,
            Key::Num3 => 3,
            Key::Num4 => 4,
            Key::Num5 => 5,
            Key::Num6 => 6,
            Key::Num7 => 7,
            Key::Num8 => 8,
            Key::Num9 => 9,
            Key::A => 10,
            Key::B => 11,
            Key::C => 12,
            Key::D => 13,
            Key::E => 14,
            Key::F => 15,
            Key::Exit => 16, // XXX This should probably fail => TryInto
        }
    }
}

impl From<u8> for Key {
    fn from(value: u8) -> Self {
        match value {
            0 => Key::Num0,
            1 => Key::Num1,
            2 => Key::Num2,
            3 => Key::Num3,
            4 => Key::Num4,
            5 => Key::Num5,
            6 => Key::Num6,
            7 => Key::Num7,
            8 => Key::Num8,
            9 => Key::Num9,
            10 => Key::A,
            11 => Key::B,
            12 => Key::C,
            13 => Key::D,
            14 => Key::E,
            15 => Key::F,
            16 => Key::Exit, // XXX This should probably fail => TryInto
            _ => panic!("Invalid u8 -> Key"),
        }
    }
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
                    keys.push(Key::Exit);
                }
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    let key = match keycode {
                        Keycode::X => Some(Key::Num0),
                        Keycode::Num1 => Some(Key::Num1),
                        Keycode::Num2 => Some(Key::Num2),
                        Keycode::Num3 => Some(Key::Num3),
                        Keycode::Q => Some(Key::Num4),
                        Keycode::W => Some(Key::Num5),
                        Keycode::E => Some(Key::Num6),
                        Keycode::A => Some(Key::Num7),
                        Keycode::S => Some(Key::Num8),
                        Keycode::D => Some(Key::Num9),
                        Keycode::Z => Some(Key::A),
                        Keycode::B => Some(Key::B),
                        Keycode::Num4 => Some(Key::C),
                        Keycode::R => Some(Key::D),
                        Keycode::F => Some(Key::E),
                        Keycode::V => Some(Key::F),
                        _ => None,
                    };
                    if let Some(k) = key {
                        keys.push(k);
                    };
                }
                _ => (),
            };
        }

        keys
    }
}
