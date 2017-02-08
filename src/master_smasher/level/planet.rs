use master_smasher::drawable::{Asset, Drawable};
use master_smasher::shape::{Circle, Intersect};
use super::world_assets::WorldAssets;
use super::collidable::Collidable;
use super::level_data::{PlanetData, PlanetKind};

use glm;
use glm::GenNum;
use glm::ext::normalize_to;
use num_traits::Zero;

use std::cmp;

struct Ring {
    radius: f64,
    strength: f64,
    asset: Asset,
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

    pub fn pull_vector(&self, point: glm::DVec2, radius: f64) -> glm::DVec2 {
        self.ring.as_ref().map_or(glm::DVec2::zero(), |r| {
            let dist = self.body.center - point;
            let len = glm::length(dist);
            if len > (r.radius + radius) {
                glm::DVec2::zero()
            } else {
                let force = r.strength / (len.powf(0.8));
                normalize_to(dist, force)
            }
        })
    }

    pub fn drawables(&self) -> Vec<Drawable> {
        let mut drawables = self.ring.as_ref().map_or(vec![], |r| vec![Drawable::Asset(&r.asset)]);
        drawables.push(Drawable::Asset(&self.asset));
        drawables
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
            Ring {
                asset: asset,
                radius: r,
                strength: s,
            }
        });

        (planet, ring)
    }
}

impl<I: Intersect<Circle>> Collidable<Circle, I> for Planet {
    fn collides(&self, shape: &I) -> bool {
        shape.intersects(&self.body)
    }
}
