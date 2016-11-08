extern crate sdl2;
extern crate glm;

use std::error::Error;
use std::time::{Instant, Duration};
use self::sdl2::rect;

use self::sdl2::render::{Renderer, Texture};

pub struct Animation {
    texture: Texture,
    num_frames: u16,
    current_frame: u16,
    frame_duration: Duration,
    frame_instant: Option<Instant>,
}

impl Animation {
    pub fn new(texture: Texture, num_frames: u16, frame_duration_ms: u16) -> Self {
        Animation {
            texture: texture,
            num_frames: num_frames,
            current_frame: 0,
            frame_duration: Duration::from_millis(frame_duration_ms as u64),
            frame_instant: None,
        }
    }

    pub fn draw(&mut self, renderer: &mut Renderer) -> Result<(), Box<Error>> {
        let frame_rect = match self.frame_instant {
            None => {}
            Some(_) => {}
        };

        Ok(())
    }
}
