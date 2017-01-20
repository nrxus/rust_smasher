use std::time::{Instant, Duration};

use glm;

pub struct Animation {
    num_frames: u32,
    frame_duration: Duration,
    repeat: bool,
    frame_instant: Option<Instant>,
    current_frame: u32,
}

impl Animation {
    pub fn new(num_frames: u32, frame_duration: Duration, repeat: bool) -> Self {
        Animation {
            num_frames: num_frames,
            frame_duration: frame_duration,
            repeat: repeat,
            frame_instant: None,
            current_frame: 0,
        }
    }

    pub fn update(&mut self) {
        self.current_frame = self.advance_frame();
        if self.current_frame >= self.num_frames {
            self.loop_animation();
        }
    }

    pub fn src_rect(&self) -> glm::DVec4 {
        let width = 1. / self.num_frames as f64;
        let uv_left = width * self.current_frame as f64;
        glm::dvec4(uv_left, 0., width, 1.)
    }

    pub fn is_active(&self) -> bool {
        self.frame_instant.is_some()
    }

    fn loop_animation(&mut self) {
        if self.repeat {
            self.current_frame -= self.num_frames;
        } else {
            self.frame_instant = None;
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
