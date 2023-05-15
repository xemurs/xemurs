mod error;
mod window;

pub use error::{Error, Result};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;

use self::window::Window;

pub struct SystemConfig {
    title: String,
    screen_height: u32,
    screen_width: u32,
}

impl Default for SystemConfig {
    fn default() -> Self {
        Self {
            title: String::from("Xemurs"),
            screen_height: 720,
            screen_width: 1280,
        }
    }
}

pub struct System {
    events: EventPump,
    window: Window,
}

impl System {
    pub fn new(config: SystemConfig) -> Result<Self> {
        let sdl = sdl2::init().map_err(|err| Error::SdlInit(err))?;
        let event_pump = sdl.event_pump().map_err(|err| Error::EventPumpInit(err))?;
        let window = Window::new(&sdl, config)?;

        Ok(Self {
            events: event_pump,
            window,
        })
    }

    pub fn start(&mut self) -> Result<()> {
        while let Ok(()) = self.poll() {
            self.window.render()?;
        }

        Ok(())
    }

    fn poll(&mut self) -> Result<()> {
        for event in self.events.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    return Err(Error::ProgramQuit);
                }
                _ => {}
            }
        }

        Ok(())
    }
}
