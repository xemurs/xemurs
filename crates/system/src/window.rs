use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::Sdl;

use crate::error::{Error, Result};
use crate::SystemConfig;

pub const BACKGROUND_COLOR: Color = Color::RGB(u8::MIN, u8::MIN, u8::MIN);

pub struct Window {
    canvas: Canvas<sdl2::video::Window>,
    screen_height: u32,
    screen_width: u32,
}

impl Window {
    pub fn new(ctx: &Sdl, config: SystemConfig) -> Result<Self> {
        let video_subsystem = ctx
            .video()
            .map_err(|err| Error::VideoSubsystemInitialization(err))?;
        let window = video_subsystem
            .window(&config.title, config.screen_width, config.screen_height)
            .position_centered()
            .build()?;
        let canvas = window.into_canvas().build()?;

        Ok(Self {
            canvas,
            screen_height: config.screen_height,
            screen_width: config.screen_width,
        })
    }

    pub fn render(&mut self) -> Result<()> {
        for col in 0..self.screen_width {
            for row in 0..self.screen_height {
                self.canvas.set_draw_color(BACKGROUND_COLOR);
                self.canvas
                    .fill_rect(Rect::new(
                        (col * 20) as i32,
                        (row * 20) as i32,
                        20 as u32,
                        20 as u32,
                    ))
                    .map_err(|err| Error::WindowRenderError(err))?;
            }
        }

        self.canvas.present();
        Ok(())
    }
}
