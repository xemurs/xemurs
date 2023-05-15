pub(crate) mod cpu;
pub(crate) mod display;
pub(crate) mod keypad;
pub(crate) mod memory;
pub(crate) mod opcode;
pub(crate) mod registers;
pub(crate) mod rom;
pub(crate) mod stack;

use system::emulator::{Emulator, Keycode};

use self::cpu::Cpu;
use self::keypad::Key;

pub struct CosmacVip(Cpu);

impl CosmacVip {
    pub fn new() -> Self {
        Self(Cpu::new())
    }
}

impl Emulator for CosmacVip {
    fn keyboard_press(&mut self, keycode: Keycode) -> system::Result<()> {
        if let Some(key) = Key::from_keycode(keycode) {
            println!("Pressed {:?}", key);
            self.0.keypad.press(key);
        }

        Ok(())
    }
}
