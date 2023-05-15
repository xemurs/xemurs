mod error;
mod window;

pub mod emulator;

pub use error::{Error, Result};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;

use self::emulator::Emulator;
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

#[derive(Debug, PartialEq, Eq)]
pub enum State {
    Running,
    Stopped,
}

pub struct System {
    emulator: Box<dyn Emulator>,
    events: EventPump,
    state: State,
    window: Window,
}

impl System {
    pub fn new(config: SystemConfig, emulator: Box<dyn Emulator>) -> Result<Self> {
        let sdl = sdl2::init().map_err(|err| Error::SdlInit(err))?;
        let event_pump = sdl.event_pump().map_err(|err| Error::EventPumpInit(err))?;
        let window = Window::new(&sdl, config)?;

        Ok(Self {
            emulator,
            events: event_pump,
            state: State::Stopped,
            window,
        })
    }

    pub fn start(&mut self) -> Result<()> {
        self.state = State::Running;

        while self.state == State::Running {
            for event in self.events.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => {
                        self.state = State::Stopped;
                        println!("Detected ESC/Quit, Stopping");
                    }
                    Event::KeyDown {
                        keycode: Some(keycode),
                        ..
                    } => {
                        self.emulator.keyboard_press(keycode)?;
                    }
                    _ => {}
                }

                self.window.render()?;
            }
        }

        Ok(())
    }
}
