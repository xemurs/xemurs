pub use sdl2::keyboard::Keycode;

use crate::{Result, System};

pub trait Emulator {
    fn start(&mut self, system: &mut System) -> Result<()>;
}
