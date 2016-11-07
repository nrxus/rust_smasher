extern crate sdl2;
extern crate glm;

use std::cmp;
use std::error::Error;

use self::sdl2::rect;
use self::sdl2::render::{Renderer, Texture};

use circle::Circle;

pub struct Planet {
    texture: Texture,
    center: glm::Vector2<f64>,
    dims: glm::Vector2<u32>,
}

impl Planet {
    pub fn new(texture: Texture, center: glm::IVec2) -> Self {
        let query = texture.query();
        let center = glm::dvec2(center.x as f64, center.y as f64);
        let dims = glm::uvec2(query.width, query.width);

        Planet {
            texture: texture,
            center: center,
            dims: dims,
        }
    }

    pub fn draw(&self, renderer: &mut Renderer) -> Result<(), Box<Error>> {
        let left = self.center.x as i32 - self.dims.x as i32 / 2;
        let top = self.center.y as i32 - self.dims.y as i32 / 2;
        let rect = rect::Rect::new(left, top, self.dims.x, self.dims.y);

        try!(renderer.copy(&self.texture, None, Some(rect)));
        Ok(())
    }

    pub fn collision_body(&self) -> Circle {
        let diameter = cmp::min(self.dims.x, self.dims.x) as f64;

        Circle {
            center: self.center,
            radius: diameter / 2.,
        }
    }
}
