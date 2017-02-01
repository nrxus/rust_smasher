use master_smasher::drawable::{Asset, AssetManager, Drawable, TextureAsset};
use master_smasher::shape::{Circle, Intersect};
use super::collidable::Collidable;
use super::level_data::PlanetKind;

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
    pub fn new(center: glm::IVec2,
               strength: f64,
               gravity_radius: f64,
               kind: PlanetKind,
               asset_manager: &AssetManager)
               -> Self {
        let (planet_asset, mut gravity_asset) = Self::load_assets(kind, center, asset_manager);
        gravity_asset.resize(glm::UVec2::from_s((gravity_radius * 2.) as u32));
        let dims = planet_asset.dims();
        let planet_radius = cmp::min(dims.x, dims.y) as f64 / 2.;
        let center = glm::to_dvec2(center);
        let body = Circle {
            center: center,
            radius: planet_radius,
        };

        Planet {
            body: body,
            strength: strength,
            gravity_radius: gravity_radius,
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

    fn load_assets(kind: PlanetKind,
                   center: glm::IVec2,
                   asset_manager: &AssetManager)
                   -> (Asset, Asset) {
        let (planet, ring) = match kind {
            PlanetKind::RED => (TextureAsset::RedPlanet, TextureAsset::RedRing),
            PlanetKind::BLUE => (TextureAsset::BluePlanet, TextureAsset::BlueRing),
            PlanetKind::WHITE => (TextureAsset::WhitePlanet, TextureAsset::WhiteRing),
        };

        (asset_manager.get_asset(planet, center), asset_manager.get_asset(ring, center))
    }
}

impl<I: Intersect<Circle>> Collidable<Circle, I> for Planet {
    fn collides(&self, shape: &I) -> bool {
        shape.intersects(&self.body)
    }
}
