pub use sdl2::keyboard::Keycode;

use crate::Result;

pub trait Emulator {
    fn keyboard_press(&mut self, keycode: Keycode) -> Result<()>;
}
