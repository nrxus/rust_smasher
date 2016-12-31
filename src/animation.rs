extern crate sdl2;
extern crate glm;
extern crate moho;

use std::time::{Instant, Duration};

use self::moho::resource_manager::ResourceManager;
use self::moho::resource_manager::Renderer;
use self::moho::resource_manager::TextureData;

pub struct Animation<R: Renderer> {
    texture: TextureData<R::Texture>,
    num_frames: u32,
    current_frame: u32,
    frame_duration: Duration,
    frame_instant: Option<Instant>,
    repeat: bool,
}

impl<R: Renderer> Animation<R> {
    pub fn new(texture: TextureData<R::Texture>,
               num_frames: u32,
               repeat: bool,
               frame_duration_ms: u16)
               -> Self {
        Animation {
            texture: texture,
            num_frames: num_frames,
            repeat: repeat,
            frame_duration: Duration::from_millis(frame_duration_ms as u64),
            current_frame: 0,
            frame_instant: None,
        }
    }

    pub fn update(&mut self) -> bool {
        self.current_frame = self.calculate_frame();
        self.is_active()
    }

    pub fn draw(&self,
                renderer: &mut ResourceManager<R>,
                center: glm::IVec2,
                dims: glm::UVec2)
                -> Result<(), String> {
        let texture_width = (self.texture.width / self.num_frames) as i32;
        let texture_height = self.texture.height as i32;
        let uv_left = texture_width * self.current_frame as i32;
        let src = glm::ivec4(uv_left, 0, texture_width, texture_height);
        renderer.draw_from_center(&*self.texture.texture, Some(src), center, dims, None)
    }

    fn is_active(&mut self) -> bool {
        if self.current_frame >= self.num_frames {
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

    fn calculate_frame(&mut self) -> u32 {
        match self.frame_instant {
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
        }
    }
}
