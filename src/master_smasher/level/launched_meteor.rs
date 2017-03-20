use master_smasher::drawable::{Animation, AnimationData, GameRenderer, Rectifiable};
use master_smasher::shape::{Circle, Intersect, Shape};
use super::collidable::Collidable;
use super::interpolate::*;
use super::planet::Planet;
use super::MeteorState;
use errors::*;

use glm;
use moho::renderer::Renderer;
use moho::resource_manager::{ResourceManager, Texture};
use num_traits::One;

pub struct LaunchedMeteor {
    body: State<Wrapped<Circle>>,
    texture: Texture,
    velocity: glm::DVec2,
}

impl LaunchedMeteor {
    pub fn new(body: Circle,
               texture: Texture,
               max_coords: glm::UVec2,
               velocity: glm::DVec2)
               -> Self {
        let wrapped = Wrapped {
            actual: body,
            unwrapped: None,
            wrapping: glm::to_dvec2(max_coords),
        };

        let body = State::new(wrapped);

        LaunchedMeteor {
            texture: texture,
            body: body,
            velocity: velocity,
        }
    }

    pub fn update(&mut self, planets: &[Planet]) {
        self.velocity = self.velocity + self.acceleration(planets);
        self.displace();
    }

    pub fn draw<R>(&self, interpolation: f64, renderer: &mut ResourceManager<R>) -> Result<()>
        where R: Renderer
    {
        let body = self.body.interpolated(interpolation).actual;
        renderer.render(&self.texture, body.rectify())
    }

    pub fn collides<S, C>(&self, collidable: &C) -> bool
        where S: Shape,
              C: Collidable<S, Circle>,
              Circle: Intersect<S>
    {
        collidable.collides(&self.body.current.actual)
    }

    pub fn explode(&self, explosion: AnimationData) -> MeteorState {
        let center = glm::to_ivec2(self.body
                                       .current
                                       .actual
                                       .center);
        let explosion = Animation::from_data(explosion, center, glm::DVec2::one());
        MeteorState::EXPLODED(explosion)
    }

    fn acceleration(&self, planets: &[Planet]) -> glm::DVec2 {
        let body = self.body.current.actual;
        planets.iter().map(|p| p.pull_vector(body.center, body.radius)).fold(glm::dvec2(0., 0.),
                                                                             |c, a| c + a) /
        50.
    }

    fn displace(&mut self) {
        let next = self.body.current.displace(self.velocity);
        self.body.update(next);
    }
}
