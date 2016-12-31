extern crate glm;
extern crate sdl2;
extern crate moho;

use self::moho::resource_manager::*;

use std::rc::Rc;

use circle::Circle;
use planet::Planet;

pub struct Meteor<R: Renderer> {
    center: glm::DVec2,
    radius: f64,
    max_coords: glm::UVec2,
    velocity: glm::DVec2,
    launched: bool,
    texture: Rc<R::Texture>,
}

impl<R: Renderer> Meteor<R> {
    pub fn new(center: glm::UVec2,
               radius: f64,
               max_coords: glm::UVec2,
               texture: Rc<R::Texture>)
               -> Self {
        let center = glm::dvec2(center.x as f64, center.y as f64);

        Meteor {
            center: center,
            radius: radius,
            max_coords: max_coords,
            velocity: glm::dvec2(0., 0.),
            launched: false,
            texture: texture,
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

    pub fn update(&mut self, planets: &[Planet<R>]) -> bool {
        self.pull(planets);
        self.displace();
        !self.collides_with(planets)
    }

    pub fn is_launched(&self) -> bool {
        self.launched
    }

    pub fn draw(&self, renderer: &mut ResourceManager<R>) -> Result<(), String> {
        let center = glm::ivec2(self.center.x as i32, self.center.y as i32);
        let diameter = (self.radius * 2.) as u32;
        let dims = glm::uvec2(diameter, diameter);
        renderer.draw_from_center(&*self.texture, None, center, dims, Some(self.max_coords))
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    pub fn center(&self) -> glm::DVec2 {
        self.center
    }

    fn pull(&mut self, planets: &[Planet<R>]) {
        for planet in planets {
            let acceleration = planet.pull_vector(self.center);
            self.velocity = self.velocity + acceleration;
        }
    }

    fn displace(&mut self) {
        self.center.y += self.velocity.y;
        self.center.x += self.velocity.x;

        let max_height = self.max_coords.y as f64;
        let max_width = self.max_coords.x as f64;

        self.center.y = (self.center.y + max_height) % max_height;
        self.center.x = (self.center.x + max_width) % max_width;
    }

    fn collides_with(&self, planets: &[Planet<R>]) -> bool {
        let body = self.collision_body();
        planets.iter().any(|p| p.collides_with(&body))
    }

    fn collision_body(&self) -> Circle {
        Circle {
            center: self.center,
            radius: self.radius as f64,
        }
    }
}
