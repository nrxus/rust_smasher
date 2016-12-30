extern crate sdl2;
extern crate glm;
extern crate moho;

use std::cmp;

use self::moho::resource_manager::*;

use circle::Circle;
use sprite_strip::SpriteStrip;
use shape::Intersect;

pub struct Planet<R: Renderer> {
    sprite: SpriteStrip<R>,
    gravity_sprite: SpriteStrip<R>,
    center: glm::Vector2<f64>,
    gravity_radius: u32,
}

impl<R: Renderer> Planet<R> {
    pub fn new(texture: TextureData<R::Texture>,
               gravity_texture: TextureData<R::Texture>,
               center: glm::IVec2,
               gravity_radius: u32)
               -> Self {
        let center = glm::dvec2(center.x as f64, center.y as f64);
        let sprite = SpriteStrip::new(texture, 1, None);
        let gravity_sprite = SpriteStrip::new(gravity_texture, 1, None);

        Planet {
            sprite: sprite,
            center: center,
            gravity_radius: gravity_radius,
            gravity_sprite: gravity_sprite,
        }
    }

    pub fn draw(&self, renderer: &mut ResourceManager<R>) -> Result<(), String> {
        let center = glm::ivec2(self.center.x as i32, self.center.y as i32);
        self.gravity_sprite.draw(renderer, center, 0)?;
        self.sprite.draw(renderer, center, 0)
    }

    pub fn collides_with<S: Intersect<Circle>>(&self, shape: &S) -> bool {
        shape.intersects(&self.collision_body())
    }

    fn collision_body(&self) -> Circle {
        let dims = self.sprite.get_dims();
        let diameter = cmp::min(dims.x, dims.y) as f64;

        Circle {
            center: self.center,
            radius: diameter / 2.,
        }
    }
}
