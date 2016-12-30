extern crate sdl2;
extern crate glm;
extern crate moho;

use self::moho::resource_manager::*;

use sdl2::rect;

use std::rc::Rc;

use circle::Circle;
use shape::Intersect;

pub struct Planet<R: Renderer> {
    center: glm::DVec2,
    planet_texture: Rc<R::Texture>,
    gravity_texture: Rc<R::Texture>,
    planet_radius: f64,
    gravity_radius: f64,
}

impl<R: Renderer> Planet<R> {
    pub fn new(center: glm::UVec2,
               gravity_radius: f64,
               planet_radius: f64,
               planet_texture: Rc<R::Texture>,
               gravity_texture: Rc<R::Texture>)
               -> Self {
        let center = glm::dvec2(center.x as f64, center.y as f64);

        Planet {
            center: center,
            planet_radius: planet_radius,
            gravity_radius: gravity_radius,
            planet_texture: planet_texture,
            gravity_texture: gravity_texture,
        }
    }

    pub fn draw(&self, renderer: &mut ResourceManager<R>) -> Result<(), String> {
        let center = (self.center.x as i32, self.center.y as i32);
        let planet_diameter = (self.planet_radius * 2.) as u32;
        let gravity_diameter = (self.gravity_radius * 2.) as u32;
        let gravity_dst = rect::Rect::from_center(center, gravity_diameter, gravity_diameter);
        let planet_dst = rect::Rect::from_center(center, planet_diameter, planet_diameter);

        renderer.draw(&*self.gravity_texture, None, Some(gravity_dst), None)?;
        renderer.draw(&*self.planet_texture, None, Some(planet_dst), None)
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
