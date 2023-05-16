mod error;
mod window;

pub mod emulator;

pub use error::{Error, Result};
pub use sdl2::event::Event;
pub use sdl2::keyboard::KeyboardState;

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

#[derive(Debug, PartialEq, Eq)]
pub enum State {
    Running,
    Stopped,
}

pub struct System {
    events: EventPump,
    state: State,
    window: Window,
}

impl System {
    pub fn new(config: SystemConfig) -> Result<Self> {
        let sdl = sdl2::init().map_err(|err| Error::SdlInit(err))?;
        let event_pump = sdl.event_pump().map_err(|err| Error::EventPumpInit(err))?;
        let window = Window::new(&sdl, config)?;

        Ok(Self {
            events: event_pump,
            state: State::Stopped,
            window,
        })
    }

    pub fn start(&mut self) {
        self.state = State::Running;
    }

    #[inline]
    pub fn is_running(&self) -> bool {
        matches!(self.state, State::Running)
    }

    pub fn poll_event(&mut self) -> Option<Event> {
        if let Some(event) = self.events.poll_event() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    self.state = State::Stopped;
                    println!("Detected ESC/Quit, Stopping");
                }
                _ => {
                    return Some(event);
                }
            }
        }

        None
    }

    /// Retrieves the keyboard state
    pub fn keyboard_state(&self) -> KeyboardState {
        self.events.keyboard_state()
    }

    pub fn render(&mut self) -> Result<()> {
        self.window.render()
    }
}
