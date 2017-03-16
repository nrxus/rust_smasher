use master_smasher::drawable::Asset;
use master_smasher::shape::{Circle, Intersect};
use super::world_assets::WorldAssets;
use super::collidable::Collidable;
use super::level_data::{PlanetData, PlanetKind};
use errors::*;

use glm::{self, GenNum};
use glm::ext::normalize_to;
use moho::renderer::Renderer;
use moho::resource_manager::ResourceManager;
use num_traits::Zero;

use std::cmp;
use std::time::Duration;

struct Ring {
    radius: f64,
    strength: f64,
    asset: Asset,
    zoom: f64,
}

impl Ring {
    pub fn new(radius: f64, strength: f64, asset: Asset) -> Self {
        Ring {
            radius: radius,
            strength: strength,
            asset: asset,
            zoom: 1.,
        }
    }

    pub fn animate(&mut self, delta: Duration) {
        const K: f64 = 0.166;
        const NANO_IN_SEC: f64 = 1000000000.;
        let moving_radius = self.asset.dims().x as f64 * self.zoom / 2.;
        let pull = glm::length(self.pull_vector(glm::dvec2(moving_radius, 0.), 0.));
        let time = delta.as_secs() as f64 + delta.subsec_nanos() as f64 / NANO_IN_SEC;
        self.zoom *= 1. / 2_f64.powf(K * pull * time);
    }

    pub fn draw<R>(&self, renderer: &mut ResourceManager<R>) -> Result<()>
        where R: Renderer
    {
        let mut moving = self.asset;
        moving.zoom(glm::DVec2::from_s(self.zoom));

        self.asset.draw(renderer)?;
        moving.draw(renderer)
    }

    pub fn pull_vector(&self, dist: glm::DVec2, radius: f64) -> glm::DVec2 {
        let len = glm::length(dist);
        if len > (self.radius + radius) {
            glm::DVec2::zero()
        } else {
            let force = self.strength / (len.powf(0.8));
            normalize_to(dist, force)
        }
    }
}

pub struct Planet {
    body: Circle,
    asset: Asset,
    ring: Option<Ring>,
}

impl Planet {
    pub fn new(data: &PlanetData, textures: &WorldAssets) -> Self {
        let (asset, ring) = Self::load_assets(data, textures);
        let dims = asset.dims();
        let radius = cmp::min(dims.x, dims.y) as f64 / 2.;
        let center = glm::dvec2(data.x as f64, data.y as f64);
        let body = Circle {
            center: center,
            radius: radius,
        };

        Planet {
            body: body,
            asset: asset,
            ring: ring,
        }
    }

    pub fn animate(&mut self, delta: Duration) {
        if let Some(ref mut r) = self.ring {
            if r.asset.dims().x as f64 * r.zoom / 2. < self.body.radius {
                r.zoom = 1.;
            }
            r.animate(delta)
        }
    }

    pub fn pull_vector(&self, point: glm::DVec2, radius: f64) -> glm::DVec2 {
        self.ring.as_ref().map_or(glm::DVec2::zero(),
                                  |r| r.pull_vector(self.body.center - point, radius))
    }

    pub fn draw<R>(&self, renderer: &mut ResourceManager<R>) -> Result<()>
        where R: Renderer
    {
        if let Some(ref r) = self.ring {
            r.draw(renderer)?;
        }
        self.asset.draw(renderer)
    }

    fn load_assets(data: &PlanetData, textures: &WorldAssets) -> (Asset, Option<Ring>) {
        let center = glm::ivec2(data.x, data.y);
        let (planet, ring) = match data.kind {
            PlanetKind::RED { ring, strength } => {
                (textures.red_planet, Some((textures.red_ring, ring, strength)))
            }
            PlanetKind::BLUE { ring, strength } => {
                (textures.blue_planet, Some((textures.blue_ring, ring, strength)))
            }
            PlanetKind::WHITE { ring, strength } => {
                (textures.white_planet, Some((textures.white_ring, ring, strength)))
            }
            PlanetKind::DEAD => (textures.dead_planet, None),
        };

        let planet = Asset::from_texture(&planet, center);
        let ring = ring.map(|(t, r, s)| {
            let dims = glm::UVec2::from_s((r * 2.) as u32);
            let asset = Asset::centered_on(t.id, center, dims);
            Ring::new(r, s, asset)
        });

        (planet, ring)
    }
}

impl<I: Intersect<Circle>> Collidable<Circle, I> for Planet {
    fn collides(&self, shape: &I) -> bool {
        shape.intersects(&self.body)
    }
}
