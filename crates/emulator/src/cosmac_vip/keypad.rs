use std::ops::{Index, IndexMut};

use system::emulator::Keycode;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Key {
    Key0,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    KeyA,
    KeyB,
    KeyC,
    KeyD,
    KeyE,
    KeyF,
}

impl Key {
    #[inline]
    pub fn from_keycode(keycode: Keycode) -> Option<Key> {
        match keycode {
            Keycode::Num0 => Some(Key::Key0),
            Keycode::Num1 => Some(Key::Key1),
            Keycode::Num2 => Some(Key::Key2),
            Keycode::Num3 => Some(Key::Key3),
            Keycode::Num4 => Some(Key::Key4),
            Keycode::Num5 => Some(Key::Key5),
            Keycode::Num6 => Some(Key::Key6),
            Keycode::Num7 => Some(Key::Key7),
            Keycode::Num8 => Some(Key::Key8),
            Keycode::Num9 => Some(Key::Key9),
            Keycode::A => Some(Key::KeyA),
            Keycode::B => Some(Key::KeyB),
            Keycode::C => Some(Key::KeyC),
            Keycode::D => Some(Key::KeyD),
            Keycode::E => Some(Key::KeyE),
            Keycode::F => Some(Key::KeyF),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Keypad([bool; 16]);

impl Keypad {
    pub fn new() -> Self {
        Keypad([false; 16])
    }

    pub fn press(&mut self, key: Key) {
        self.0 = [false; 16];

        match key {
            Key::Key0 => self.0[0] = true,
            Key::Key1 => self.0[1] = true,
            Key::Key2 => self.0[2] = true,
            Key::Key3 => self.0[3] = true,
            Key::Key4 => self.0[4] = true,
            Key::Key5 => self.0[5] = true,
            Key::Key6 => self.0[6] = true,
            Key::Key7 => self.0[7] = true,
            Key::Key8 => self.0[8] = true,
            Key::Key9 => self.0[9] = true,
            Key::KeyA => self.0[10] = true,
            Key::KeyB => self.0[11] = true,
            Key::KeyC => self.0[12] = true,
            Key::KeyD => self.0[13] = true,
            Key::KeyE => self.0[14] = true,
            Key::KeyF => self.0[15] = true,
        }
    }

    pub fn clear(&mut self) {
        self.0 = [false; 16];
    }
}

impl Index<usize> for Keypad {
    type Output = bool;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Keypad {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}
