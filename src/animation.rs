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

pub struct SpriteStrip {
    texture: Texture,
    dims: glm::UVec2,
    wrapping_coords: Option<glm::UVec2>,
}

impl SpriteStrip {
    pub fn new(texture: Texture, num_frames: u32, wrapping_coords: Option<glm::UVec2>) -> Self {
        let query = texture.query();
        let dims = glm::uvec2(query.width / num_frames, query.height);

        SpriteStrip {
            texture: texture,
            dims: dims,
            wrapping_coords: wrapping_coords,
        }
    }

    pub fn draw(&self,
                renderer: &mut Renderer,
                center: glm::IVec2,
                frame_num: u32)
                -> Result<(), String> {

        let source_rect = rect::Rect::new((self.dims.x * frame_num) as i32,
                                          0,
                                          self.dims.x,
                                          self.dims.y);

        self.drawing_rectangles(center)
            .iter()
            .filter(|r| r.is_some())
            .map(|&r| renderer.copy(&self.texture, Some(source_rect), r))
            .fold(Ok(()), |res, x| { if res.is_err() { res } else { x } })
    }

    pub fn get_dims(&self) -> glm::UVec2 {
        self.dims
    }

    fn drawing_rectangles(&self, center: glm::IVec2) -> [Option<rect::Rect>; 4] {
        let left = center.x - self.dims.x as i32 / 2;
        let top = center.y - self.dims.y as i32 / 2;

        let rect = rect::Rect::new(left, top, self.dims.x, self.dims.y);
        let (side_rect, vert_rect) = if let None = self.wrapping_coords {
            (None, None)
        } else {
            (self.side_rect(&rect, center), self.vert_rect(&rect, center))
        };

        let side_vert_rect = match (vert_rect, side_rect) {
            (Some(vert_unwrapped), Some(side_unwrapped)) => {
                Some(rect::Rect::new(side_unwrapped.x(),
                                     vert_unwrapped.y(),
                                     self.dims.x,
                                     self.dims.y))
            }
            _ => None,
        };

        [Some(rect), vert_rect, side_rect, side_vert_rect]
    }

    fn vert_rect(&self, original: &rect::Rect, center: glm::IVec2) -> Option<rect::Rect> {
        let bottom = center.y + self.dims.y as i32 / 2;
        let top = center.y - self.dims.y as i32 / 2;
        let max_height = self.wrapping_coords.unwrap().y as i32;

        let mut copy = original.clone();

        if top < 0 {
            copy.set_y(top + max_height);
            Some(copy)
        } else if bottom > max_height {
            copy.set_bottom(bottom % max_height);
            Some(copy)
        } else {
            None
        }
    }

    fn side_rect(&self, original: &rect::Rect, center: glm::IVec2) -> Option<rect::Rect> {
        let left = center.x - self.dims.x as i32 / 2;
        let right = center.x + self.dims.x as i32 / 2;
        let max_width = self.wrapping_coords.unwrap().x as i32;

        let mut copy = original.clone();

        if left < 0 {
            copy.set_x(left + max_width);
            Some(copy)
        } else if right > max_width {
            copy.set_right(right % max_width);
            Some(copy)
        } else {
            None
        }
    }
}
