mod error;
mod window;

pub use error::{Error, Result};

use self::window::{Window, WindowConfig};

pub struct SystemConfig {
    title: String,
    height: u32,
    width: u32,
}

pub struct System {
    window: Window,
}

impl System {
    pub fn new(config: SystemConfig) -> Result<Self> {
        let window = Window::new(config)?;

        Ok(Self {
            window,
        })
    }
}