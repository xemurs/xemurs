use sdl2::Sdl;
use sdl2::render::Canvas;

use crate::error::{Error, Result};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WindowConfig {
  title: String,
  height: u32,
  width: u32,
}

pub struct Window {
    pub(crate) canvas: Canvas<sdl2::video::Window>,
}

impl Window {
    pub fn new(ctx: &Sdl, config: WindowConfig) -> Result<Self> {
        let video_subsystem = ctx.video().map_err(|err| {
          Error::VideoSubsystemInitialization(err)
        })?;
        let window = video_subsystem
            .window(&config.title, config.height, config.width)
            .position_centered()
            .build()?;
        let canvas = window.into_canvas().build()?;

        Ok(Self { canvas })
    }
}
