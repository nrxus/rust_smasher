use master_smasher::drawable::{Animation, AnimationData, Asset, Drawable};
use master_smasher::shape::{Circle, Intersect, Shape};
use super::collidable::Collidable;
use super::interpolate::State;
use super::planet::Planet;
use super::MeteorState;

use glm::{self, GenNum};
use num_traits::One;

use std::cmp;

pub struct LaunchedMeteor {
    body: State<Circle>,
    texture: usize,
    velocity: glm::DVec2,
    max_coords: glm::DVec2,
}

impl LaunchedMeteor {
    pub fn new(asset: Asset, max_coords: glm::UVec2, velocity: glm::DVec2) -> Self {
        let center = glm::to_dvec2(asset.center());
        let dims = glm::to_ivec2(asset.dims());
        let radius = cmp::min(dims.x, dims.y) as f64 / 2.;
        let circle = Circle {
            center: center,
            radius: radius,
        };
        let body = State::new(circle);

        LaunchedMeteor {
            texture: asset.texture_id,
            body: body,
            velocity: velocity,
            max_coords: glm::to_dvec2(max_coords),
        }
    }

    pub fn update(&mut self, planets: &[Planet]) {
        self.velocity = self.velocity + self.acceleration(planets);
        self.displace();
    }

    pub fn drawables(&self, interpolation: f64) -> Vec<Drawable> {
        let body = self.body.interpolated(interpolation);
        let diameter = (body.radius * 2.) as u32;
        let center = glm::to_ivec2(body.center);
        let dims = glm::UVec2::from_s(diameter);
        let asset = Asset::centered_on(self.texture, center, dims);
        vec![Drawable::Asset(asset)]
    }

    pub fn collides<S, C>(&self, collidable: &C) -> bool
        where S: Shape,
              C: Collidable<S, Circle>,
              Circle: Intersect<S>
    {
        collidable.collides(&self.body.current)
    }

    pub fn explode(&self, explosion: AnimationData) -> MeteorState {
        let center = glm::to_ivec2(self.body.current.center);
        let explosion = Animation::from_data(explosion, center, glm::DVec2::one());
        MeteorState::EXPLODED(explosion)
    }

    fn acceleration(&self, planets: &[Planet]) -> glm::DVec2 {
        planets.iter()
            .map(|p| p.pull_vector(self.body.current.center, self.body.current.radius))
            .fold(glm::dvec2(0., 0.), |c, a| c + a) / 50.
    }

    fn displace(&mut self) {
        let mut next = self.body.current;
        next.center = next.center + self.velocity;
        let wrapped_center = (next.center + self.max_coords) % self.max_coords;
        if glm::length(wrapped_center - next.center) > 1. {
            next.center = wrapped_center;
            self.body.update(next);
        }
        self.body.update(next);
    }
}
