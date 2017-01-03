extern crate sdl2;
extern crate glm;
extern crate moho;

use std::time::{Instant, Duration};

pub struct Animation {
    num_frames: u32,
    frame_duration: Duration,
    dims: glm::UVec2,
    repeat: bool,
    frame_instant: Option<Instant>,
    current_frame: u32,
}

impl Animation {
    pub fn new(num_frames: u32, frame_duration: Duration, dims: glm::UVec2, repeat: bool) -> Self {
        Animation {
            num_frames: num_frames,
            frame_duration: frame_duration,
            dims: dims,
            repeat: repeat,
            frame_instant: None,
            current_frame: 0,
        }
    }

    pub fn update(&mut self) -> bool {
        self.current_frame = self.advance_frame();
        if self.current_frame >= self.num_frames {
            self.update_loop()
        } else {
            true
        }
    }

    pub fn src_rect(&self) -> glm::IVec4 {
        let texture_width = (self.dims.x / self.num_frames) as i32;
        let uv_left = texture_width * self.current_frame as i32;
        glm::ivec4(uv_left, 0, texture_width, self.dims.y as i32)
    }

    fn update_loop(&mut self) -> bool {
        if self.repeat {
            self.current_frame -= self.num_frames;
            true
        } else {
            self.frame_instant = None;
            false
        }
    }

    fn advance_frame(&mut self) -> u32 {
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
