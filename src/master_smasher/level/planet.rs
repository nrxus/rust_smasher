use master_smasher::drawable::{Asset, Scene, GameRenderer};
use master_smasher::shape::{Circle, Intersect};
use super::world_assets::WorldAssets;
use super::collidable::Collidable;
use super::level_data::{PlanetData, PlanetKind};
use errors::*;

use glm;
use glm::ext::normalize_to;
use moho::renderer::Renderer;
use moho::resource_manager::{ResourceManager, Texture};
use num_traits::Zero;

use std::cmp;
use std::time::Duration;

fn square(side: i32, center: glm::IVec2) -> glm::IVec4 {
    glm::ivec4(center.x - side / 2, center.y - side / 2, side, side)
}

struct Ring {
    radius: f64,
    strength: f64,
    zoom: f64,
    center: glm::IVec2,
    texture: Texture,
}

impl Ring {
    pub fn new(radius: f64, strength: f64, center: glm::IVec2, texture: Texture) -> Self {
        Ring {
            radius: radius,
            strength: strength,
            zoom: 1.,
            center: center,
            texture: texture,
        }
    }

    pub fn animate(&mut self, delta: Duration) {
        const K: f64 = 0.166;
        const NANO_IN_SEC: f64 = 1000000000.;
        let moving_radius = self.radius * self.zoom;
        let pull = glm::length(self.pull_vector(glm::dvec2(moving_radius, 0.), 0.));
        let time = delta.as_secs() as f64 + delta.subsec_nanos() as f64 / NANO_IN_SEC;
        self.zoom *= 1. / 2_f64.powf(K * pull * time);
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
            if r.radius * r.zoom < self.body.radius {
                r.zoom = 1.;
            }
            r.animate(delta)
        }
    }

    pub fn pull_vector(&self, point: glm::DVec2, radius: f64) -> glm::DVec2 {
        self.ring.as_ref().map_or(glm::DVec2::zero(),
                                  |r| r.pull_vector(self.body.center - point, radius))
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

        let planet = Asset::from_texture(planet, center);
        let ring = ring.map(|(t, r, s)| Ring::new(r, s, center, t));

        (planet, ring)
    }
}

impl<I: Intersect<Circle>> Collidable<Circle, I> for Planet {
    fn collides(&self, shape: &I) -> bool {
        shape.intersects(&self.body)
    }
}

impl<R: Renderer> Scene<ResourceManager<R>> for Planet {
    fn show(&self, renderer: &mut ResourceManager<R>) -> Result<()> {
        if let Some(ref r) = self.ring {
            renderer.show(r)?;
        }
        renderer.show(&self.asset)
    }
}

impl<R: Renderer> Scene<ResourceManager<R>> for Ring {
    fn show(&self, renderer: &mut ResourceManager<R>) -> Result<()> {
        let diameter = self.radius * 2.;
        let dst_rect = square(diameter as i32, self.center);
        let moving_rect = square((diameter * self.zoom) as i32, self.center);
        renderer.render(&self.texture, dst_rect)?;
        renderer.render(&self.texture, moving_rect)
    }
}
