use master_smasher::asset_manager::Asset;
use master_smasher::collidable::Collidable;
use master_smasher::planet::Planet;
use master_smasher::shape::{Circle, Intersect, Shape};

use glm;
use moho::errors::*;
use moho::renderer::Renderer;
use moho::resource_manager::ResourceManager;

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
        let dims = glm::ivec2(asset.dst_rect.z, asset.dst_rect.w);
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
        self.pull(planets);
        self.displace();
    }

    pub fn draw<R: Renderer>(&self, renderer: &mut ResourceManager<R>) -> Result<()> {
        self.asset.draw(None, Some(self.max_coords), renderer)
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

    fn pull(&mut self, planets: &[Planet]) {
        for planet in planets {
            let acceleration = planet.pull_vector(self.body.center, self.body.radius);
            self.velocity = self.velocity + acceleration / 50.;
        }
    }

    fn displace(&mut self) {
        let max_coords = glm::to_dvec2(self.max_coords);
        self.body.center = self.body.center + self.velocity;
        self.body.center = (self.body.center + max_coords) % max_coords;
        self.asset.set_center(glm::to_ivec2(self.body.center));
    }
}
