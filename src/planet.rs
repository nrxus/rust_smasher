use std::rc::Rc;

use glm;
use glm::ext::normalize_to;
use moho::resource_manager::{Renderer, ResourceManager};

use circle::Circle;
use shape::Intersect;

pub struct Planet<R: Renderer> {
    center: glm::DVec2,
    strength: f64,
    planet_radius: f64,
    gravity_radius: f64,
    planet_texture: Rc<R::Texture>,
    gravity_texture: Rc<R::Texture>,
}

impl<R: Renderer> Planet<R> {
    pub fn new(center: glm::UVec2,
               strength: f64,
               gravity_radius: f64,
               planet_radius: f64,
               planet_texture: Rc<R::Texture>,
               gravity_texture: Rc<R::Texture>)
               -> Self {
        let center = glm::dvec2(center.x as f64, center.y as f64);

        Planet {
            center: center,
            strength: strength,
            planet_radius: planet_radius,
            gravity_radius: gravity_radius,
            planet_texture: planet_texture,
            gravity_texture: gravity_texture,
        }
    }

    pub fn draw(&self, renderer: &mut ResourceManager<R>) -> Result<(), String> {
        let center = glm::ivec2(self.center.x as i32, self.center.y as i32);
        let planet_diameter = (self.planet_radius * 2.) as u32;
        let gravity_diameter = (self.gravity_radius * 2.) as u32;
        let planet_dims = glm::uvec2(planet_diameter, planet_diameter);
        let gravity_dims = glm::uvec2(gravity_diameter, gravity_diameter);

        renderer.draw_from_center(&*self.gravity_texture, None, center, gravity_dims, None)?;
        renderer.draw_from_center(&*self.planet_texture, None, center, planet_dims, None)
    }

    pub fn pull_vector(&self, point: glm::DVec2, radius: f64) -> glm::DVec2 {
        let dist = self.center - point;
        let len = glm::length(dist);
        if len > (self.gravity_radius + radius) {
            glm::dvec2(0., 0.)
        } else {
            let force = self.strength / (len.powf(0.8));
            normalize_to(dist, force)
        }
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
