extern crate sdl2;
extern crate glm;

use std::time::{Instant, Duration};

use self::sdl2::render::{Renderer, Texture};
use sprite_strip::SpriteStrip;

pub struct Animation {
    sprite: SpriteStrip,
    num_frames: u32,
    current_frame: u32,
    frame_duration: Duration,
    frame_instant: Option<Instant>,
}

impl Animation {
    pub fn new(texture: Texture,
               num_frames: u32,
               frame_duration_ms: u16,
               wrapping_coords: Option<glm::UVec2>)
               -> Self {

        let sprite = SpriteStrip::new(texture, 1, wrapping_coords);

        Animation {
            sprite: sprite,
            num_frames: num_frames,
            current_frame: 0,
            frame_duration: Duration::from_millis(frame_duration_ms as u64),
            frame_instant: None,
        }
    }

    pub fn draw(&mut self, renderer: &mut Renderer, center: glm::IVec2) -> Result<(), String> {
        let frame_rect = match self.frame_instant {
            None => {}
            Some(_) => {}
        };

        self.sprite.draw(renderer, center, self.current_frame)
    }
}
