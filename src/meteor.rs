extern crate glm;
extern crate sdl2;

use std::cmp;

use self::sdl2::rect;
use self::sdl2::render::{Renderer, Texture};

use circle::Circle;

pub struct Meteor {
    texture: Texture,
    center: glm::Vector2<f64>,
    dims: glm::Vector2<u32>,
    max_coords: glm::Vector2<u32>,
    velocity: glm::Vector2<f64>,
    launched: bool,
}

impl Meteor {
    pub fn new(texture: Texture, center: glm::IVec2, max_coords: glm::Vector2<u32>) -> Self {
        let query = texture.query();
        let center = glm::dvec2(center.x as f64, center.y as f64);
        let dims = glm::uvec2(query.width, query.height);

        Meteor {
            texture: texture,
            center: center,
            dims: dims,
            max_coords: max_coords,
            velocity: glm::dvec2(0., 0.),
            launched: false,
        }
    }

    pub fn restart_at(&mut self, center: glm::IVec2) {
        self.center = glm::dvec2(center.x as f64, center.y as f64);
        self.launched = false;
        self.velocity = glm::dvec2(0., 0.);
    }

    pub fn launch(&mut self, target: glm::Vector2<i32>) {
        const FACTOR: f64 = 85.;
        let offset = glm::ivec2(target.x - self.center.x as i32,
                                target.y - self.center.y as i32);
        self.velocity = glm::dvec2(offset.x as f64 / FACTOR, offset.y as f64 / FACTOR);
        self.launched = true;
    }

    pub fn update(&mut self) {
        self.center.y += self.velocity.y;
        self.center.x += self.velocity.x;

        let max_height = self.max_coords.y as f64;
        let max_width = self.max_coords.x as f64;

        self.center.y = (self.center.y + max_height) % max_height;
        self.center.x = (self.center.x + max_width) % max_width;
    }

    pub fn is_launched(&self) -> bool {
        self.launched
    }

    pub fn draw(&self, renderer: &mut Renderer) -> Result<(), String> {
        self.drawing_rectangles()
            .iter()
            .filter(|r| r.is_some())
            .map(|&r| renderer.copy(&self.texture, None, r))
            .fold(Ok(()), |res, x| { if res.is_err() { res } else { x } })
    }

    pub fn collision_body(&self) -> Circle {
        let diameter = cmp::min(self.dims.x, self.dims.x) as f64;

        Circle {
            center: self.center,
            radius: diameter / 2.,
        }
    }

    fn drawing_rectangles(&self) -> [Option<rect::Rect>; 4] {
        let left = self.center.x as i32 - self.dims.x as i32 / 2;
        let top = self.center.y as i32 - self.dims.y as i32 / 2;

        let rect = rect::Rect::new(left, top, self.dims.x, self.dims.y);
        let side_rect = self.side_rect(&rect);
        let vert_rect = self.vert_rect(&rect);

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

    fn vert_rect(&self, original: &rect::Rect) -> Option<rect::Rect> {
        let bottom = self.center.y as i32 + self.dims.y as i32 / 2;
        let top = self.center.y as i32 - self.dims.y as i32 / 2;
        let max_height = self.max_coords.y as i32;

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

    fn side_rect(&self, original: &rect::Rect) -> Option<rect::Rect> {
        let left = self.center.x as i32 - self.dims.x as i32 / 2;
        let right = self.center.x as i32 + self.dims.x as i32 / 2;
        let max_width = self.max_coords.x as i32;

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
