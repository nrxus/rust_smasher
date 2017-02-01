use master_smasher::drawable::{Asset, AssetManager, Drawable, TextureAsset};
use master_smasher::shape::{Circle, Intersect};
use super::collidable::Collidable;
use super::level_data::{PlanetData, PlanetKind};

use glm;
use glm::GenNum;
use glm::ext::normalize_to;
use num_traits::Zero;

use std::cmp;

pub struct Planet {
    body: Circle,
    strength: f64,
    gravity_radius: f64,
    planet_asset: Asset,
    gravity_asset: Asset,
}

impl Planet {
    pub fn new(data: &PlanetData, asset_manager: &AssetManager) -> Self {
        let (planet_asset, gravity_asset) = Self::load_assets(data, asset_manager);
        let dims = planet_asset.dims();
        let planet_radius = cmp::min(dims.x, dims.y) as f64 / 2.;
        let center = glm::dvec2(data.x as f64, data.y as f64);
        let body = Circle {
            center: center,
            radius: planet_radius,
        };

        Planet {
            body: body,
            strength: data.strength,
            gravity_radius: data.ring,
            planet_asset: planet_asset,
            gravity_asset: gravity_asset,
        }
    }

    pub fn pull_vector(&self, point: glm::DVec2, radius: f64) -> glm::DVec2 {
        let dist = self.body.center - point;
        let len = glm::length(dist);
        if len > (self.gravity_radius + radius) {
            glm::DVec2::zero()
        } else {
            let force = self.strength / (len.powf(0.8));
            normalize_to(dist, force)
        }
    }

    pub fn drawables(&self) -> Vec<Drawable> {
        vec![Drawable::Asset(&self.gravity_asset), Drawable::Asset(&self.planet_asset)]
    }

    fn load_assets(data: &PlanetData, asset_manager: &AssetManager) -> (Asset, Asset) {
        let (planet, ring) = match data.kind {
            PlanetKind::RED => (TextureAsset::RedPlanet, TextureAsset::RedRing),
            PlanetKind::BLUE => (TextureAsset::BluePlanet, TextureAsset::BlueRing),
            PlanetKind::WHITE => (TextureAsset::WhitePlanet, TextureAsset::WhiteRing),
        };
        let center = glm::ivec2(data.x, data.y);
        let planet = asset_manager.get_asset(planet, center);
        let mut ring = asset_manager.get_asset(ring, center);
        ring.resize(glm::UVec2::from_s((data.ring * 2.) as u32));
        (planet, ring)
    }
}

impl<I: Intersect<Circle>> Collidable<Circle, I> for Planet {
    fn collides(&self, shape: &I) -> bool {
        shape.intersects(&self.body)
    }
}
