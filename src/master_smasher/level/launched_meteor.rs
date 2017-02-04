use master_smasher::drawable::{Asset, Drawable};
use master_smasher::shape::{Circle, Intersect, Shape};
use super::collidable::Collidable;
use super::planet::Planet;

use glm;

use std::cmp;

pub struct LaunchedMeteor {
    asset: Asset,
    body: Circle,
    velocity: glm::DVec2,
    max_coords: glm::UVec2,
}

impl LaunchedMeteor {
    pub fn new(asset: Asset, max_coords: glm::UVec2, velocity: glm::DVec2) -> Self {
        let center = glm::to_dvec2(asset.center());
        let dims = glm::to_ivec2(asset.dims());
        let radius = cmp::min(dims.x, dims.y) as f64 / 2.;
        let body = Circle {
            center: center,
            radius: radius,
        };

        LaunchedMeteor {
            asset: asset,
            body: body,
            velocity: velocity,
            max_coords: max_coords,
        }
    }

    pub fn update(&mut self, planets: &[Planet]) {
        self.velocity = self.velocity + self.acceleration(planets);
        self.displace();
    }

    pub fn drawables(&self) -> Vec<Drawable> {
        vec![Drawable::Asset(&self.asset)]
    }

    pub fn collides<S, C>(&self, collidable: &C) -> bool
        where S: Shape,
              C: Collidable<S, Circle>,
              Circle: Intersect<S>
    {
        collidable.collides(&self.body)
    }

    pub fn center(&self) -> glm::DVec2 {
        self.body.center
    }

    fn acceleration(&self, planets: &[Planet]) -> glm::DVec2 {
        planets.iter()
            .map(|p| p.pull_vector(self.body.center, self.body.radius))
            .fold(glm::dvec2(0., 0.), |c, a| c + a) / 50.
    }

    fn displace(&mut self) {
        let max_coords = glm::to_dvec2(self.max_coords);
        self.body.center = self.body.center + self.velocity;
        self.body.center = (self.body.center + max_coords) % max_coords;
        self.asset.center_on(glm::to_ivec2(self.body.center));
    }
}
