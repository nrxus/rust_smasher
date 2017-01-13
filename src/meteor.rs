use std::rc::Rc;

use glm;
use moho::resource_manager::{Renderer, ResourceManager};

use animation::Animation;
use circle::Circle;
use planet::Planet;

#[derive(Copy, Clone)]
pub enum MeteorState {
    UNLAUNCHED,
    LAUNCHED,
    EXPLODED,
}

pub struct Meteor<R: Renderer> {
    initial_center: glm::DVec2,
    center: glm::DVec2,
    radius: f64,
    max_coords: glm::UVec2,
    velocity: glm::DVec2,
    state: MeteorState,
    explosion_dims: glm::UVec2,
    explosion_animation: Animation,
    texture: Rc<R::Texture>,
    explosion_texture: Rc<R::Texture>,
}

impl<R: Renderer> Meteor<R> {
    pub fn new(center: glm::UVec2,
               radius: f64,
               max_coords: glm::UVec2,
               explosion_dims: glm::UVec2,
               explosion_animation: Animation,
               texture: Rc<R::Texture>,
               explosion_texture: Rc<R::Texture>)
               -> Self {
        let center = glm::dvec2(center.x as f64, center.y as f64);

        Meteor {
            initial_center: center,
            center: center,
            radius: radius,
            max_coords: max_coords,
            velocity: glm::dvec2(0., 0.),
            state: MeteorState::UNLAUNCHED,
            explosion_dims: explosion_dims,
            explosion_animation: explosion_animation,
            texture: texture,
            explosion_texture: explosion_texture,
        }
    }

    pub fn restart(&mut self) {
        self.center = self.initial_center;
        self.velocity = glm::dvec2(0., 0.);
        self.state = MeteorState::UNLAUNCHED;
    }

    pub fn launch(&mut self, target: glm::Vector2<i32>) {
        const FACTOR: f64 = 50.;
        let offset = glm::ivec2(target.x - self.center.x as i32,
                                target.y - self.center.y as i32);
        self.velocity = glm::dvec2(offset.x as f64 / FACTOR, offset.y as f64 / FACTOR);
        self.state = MeteorState::LAUNCHED;
    }

    pub fn update(&mut self, planets: &[Planet<R>]) {

        match self.state {
            MeteorState::UNLAUNCHED => {}
            MeteorState::LAUNCHED => {
                self.pull(planets);
                self.displace();
                if self.collides_with(planets) {
                    self.state = MeteorState::EXPLODED;
                }
            }
            MeteorState::EXPLODED => {
                self.explosion_animation.update();
                if !self.explosion_animation.is_active() {
                    self.restart();
                }
            }
        }
    }

    pub fn state(&self) -> MeteorState {
        self.state
    }

    pub fn draw(&self, renderer: &mut ResourceManager<R>) -> Result<(), String> {
        let center = glm::ivec2(self.center.x as i32, self.center.y as i32);
        let (texture, src_rect, dims) = if let MeteorState::EXPLODED = self.state {
            let src_rect = Some(self.explosion_animation.src_rect());
            (&*self.explosion_texture, src_rect, self.explosion_dims)
        } else {
            let diameter = (self.radius * 2.) as u32;
            (&*self.texture, None, glm::uvec2(diameter, diameter))
        };

        renderer.draw_from_center(texture, src_rect, center, dims, Some(self.max_coords))
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    pub fn center(&self) -> glm::DVec2 {
        self.center
    }

    fn pull(&mut self, planets: &[Planet<R>]) {
        for planet in planets {
            let acceleration = planet.pull_vector(self.center, self.radius);
            self.velocity = self.velocity + acceleration / 50.;
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
