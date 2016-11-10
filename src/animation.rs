extern crate sdl2;
extern crate glm;
extern crate moho;

use std::time::{Instant, Duration};

use self::sdl2::render::Texture;
use self::moho::resource_manager::ResourceManager;
use self::moho::resource_manager::Renderer;

use sprite_strip::SpriteStrip;

pub struct Animation {
    sprite: SpriteStrip,
    num_frames: u32,
    current_frame: u32,
    frame_duration: Duration,
    frame_instant: Option<Instant>,
    repeat: bool,
}

impl Animation {
    pub fn new(sprite: SpriteStrip, num_frames: u32, repeat: bool, frame_duration_ms: u16) -> Self {
        Animation {
            sprite: sprite,
            num_frames: num_frames,
            repeat: repeat,
            frame_duration: Duration::from_millis(frame_duration_ms as u64),
            current_frame: 0,
            frame_instant: None,
        }
    }

    pub fn update(&mut self) -> bool {
        self.current_frame = match self.frame_instant {
            None => {
                self.frame_instant = Some(Instant::now());
                0
            }
            Some(instant) => {
                if instant.elapsed() >= self.frame_duration {
                    self.frame_instant = Some(instant + self.frame_duration);
                    self.current_frame + 1
                } else {
                    self.current_frame
                }
            }
        };

        if self.current_frame > self.num_frames {
            if self.repeat {
                self.current_frame -= self.num_frames;
                true
            } else {
                self.frame_instant = None;
                false
            }
        } else {
            true
        }
    }

    pub fn draw<I>(&self,
                   renderer: &mut ResourceManager<I>,
                   center: glm::IVec2)
                   -> Result<(), String>
        where I: Renderer<Texture = Texture>
    {
        self.sprite.draw(renderer, center, self.current_frame)
    }
}
