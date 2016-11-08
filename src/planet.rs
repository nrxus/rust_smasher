extern crate sdl2;
extern crate glm;

use std::cmp;

use self::sdl2::render::{Renderer, Texture};

use circle::Circle;
use animation::SpriteStrip;

pub struct Planet {
    sprite: SpriteStrip,
    center: glm::Vector2<f64>,
}

impl Planet {
    pub fn new(texture: Texture, center: glm::IVec2) -> Self {
        let center = glm::dvec2(center.x as f64, center.y as f64);
        let sprite = SpriteStrip::new(texture, 1, None);

        Planet {
            sprite: sprite,
            center: center,
        }
    }

    pub fn draw(&self, renderer: &mut Renderer) -> Result<(), String> {
        let center = glm::ivec2(self.center.x as i32, self.center.y as i32);
        self.sprite.draw(renderer, center, 0)
    }

    pub fn collision_body(&self) -> Circle {
        let dims = self.sprite.get_dims();
        let diameter = cmp::min(dims.x, dims.y) as f64;

        Circle {
            center: self.center,
            radius: diameter / 2.,
        }
    }
}
