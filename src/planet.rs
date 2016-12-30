extern crate sdl2;
extern crate glm;
extern crate moho;

use self::moho::resource_manager::*;

use circle::Circle;
use sprite_strip::SpriteStrip;
use shape::Intersect;

pub struct Planet<R: Renderer> {
    center: glm::DVec2,
    planet_sprite: SpriteStrip<R>,
    gravity_sprite: SpriteStrip<R>,
    planet_radius: f64,
    gravity_radius: f64,
}

impl<R: Renderer> Planet<R> {
    pub fn new(center: glm::UVec2,
               gravity_radius: f64,
               planet_radius: f64,
               planet_texture: TextureData<R::Texture>,
               gravity_texture: TextureData<R::Texture>)
               -> Self {
        let center = glm::dvec2(center.x as f64, center.y as f64);
        let planet_sprite = SpriteStrip::new(planet_texture, None);
        let gravity_sprite = SpriteStrip::new(gravity_texture, None);

        Planet {
            center: center,
            planet_radius: planet_radius,
            gravity_radius: gravity_radius,
            planet_sprite: planet_sprite,
            gravity_sprite: gravity_sprite,
        }
    }

    pub fn draw(&self, renderer: &mut ResourceManager<R>) -> Result<(), String> {
        let center = glm::ivec2(self.center.x as i32, self.center.y as i32);
        let planet_diameter = (self.planet_radius * 2.) as u32;
        let gravity_diameter = (self.gravity_radius * 2.) as u32;

        self.gravity_sprite
            .draw(center,
                  glm::uvec2(gravity_diameter, gravity_diameter),
                  0,
                  renderer)?;

        self.planet_sprite.draw(center,
                                glm::uvec2(planet_diameter, planet_diameter),
                                0,
                                renderer)
    }

    pub fn collides_with<S: Intersect<Circle>>(&self, shape: &S) -> bool {
        shape.intersects(&self.collision_body())
    }

    fn collision_body(&self) -> Circle {
        Circle {
            center: self.center,
            radius: self.planet_radius,
        }
    }
}
