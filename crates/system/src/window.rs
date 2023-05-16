use std::ops::Deref;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::Sdl;

use crate::error::{Error, Result};
use crate::SystemConfig;

pub const BACKGROUND_COLOR: Color = Color::RGB(u8::MIN, u8::MIN, u8::MIN);
pub const FOREGROUND_COLOR: Color = Color::RGB(u8::MAX, u8::MAX, u8::MAX);
pub const SCREEN_HEIGHT: u32 = 32;
pub const SCREEN_WIDTH: u32 = 64;

pub struct Bitmap(Vec<u8>);

impl Bitmap {
    pub fn new(slice: &[u8]) -> Self {
        Bitmap(slice.to_vec())
    }
}

impl Deref for Bitmap {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

pub struct Window {
    pub(crate) canvas: Canvas<sdl2::video::Window>,
    pub(crate) screen_height: u32,
    pub(crate) screen_width: u32,
    pub(crate) screen_area: u32,
    pub(crate) scale: u32,
}

impl Window {
    pub fn new(ctx: &Sdl, config: SystemConfig) -> Result<Self> {
        let video_subsystem = ctx
            .video()
            .map_err(|err| Error::VideoSubsystemInitialization(err))?;
        let screen_width = SCREEN_WIDTH * config.screen_scale;
        let screen_height =  SCREEN_HEIGHT * config.screen_scale;
        let window = video_subsystem
            .window(&config.title, screen_width, screen_height)
            .position_centered()
            .build()?;
        let canvas = window.into_canvas().build()?;

        Ok(Self {
            canvas,
            screen_height,
            screen_width,
            screen_area: screen_height * screen_width,
            scale: config.screen_scale,
        })
    }

    /// This function is thought to work on Chip8 and is not intended to be
    /// used for other emulators. The value uses the original scale and its
    /// then expanded to the current scale.
    pub fn render(&mut self, bitmap: &Bitmap) -> Result<()> {
        for col in 0..SCREEN_WIDTH {
            for row in 0..SCREEN_HEIGHT {
                if bitmap[(row * SCREEN_WIDTH + col) as usize] > 0 {
                    self.canvas.set_draw_color(FOREGROUND_COLOR);
                    self.canvas
                        .fill_rect(Rect::new(
                            (col * self.scale) as i32,
                            (row * self.scale) as i32,
                            self.scale as u32,
                            self.scale as u32,
                        ))
                        .map_err(|err| Error::WindowRenderError(err))?;
                    continue;
                }

                self.canvas.set_draw_color(BACKGROUND_COLOR);
                self.canvas
                    .fill_rect(Rect::new(
                        (col * self.scale) as i32,
                        (row * self.scale) as i32,
                        self.scale as u32,
                        self.scale as u32,
                    ))
                    .map_err(|err| Error::WindowRenderError(err))?;
            }
        }

        self.canvas.present();
        Ok(())
    }
}
