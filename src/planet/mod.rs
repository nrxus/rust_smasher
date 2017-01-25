use asset::Asset;
use asset_manager::{AssetManager, TextureAsset};
use circle::Circle;
use collidable::Collidable;
use shape::Intersect;

use glm;
use glm::ext::normalize_to;
use moho::errors::*;
use moho::renderer::Renderer;
use moho::resource_manager::ResourceManager;
use num_traits::Zero;

use std::cmp;

pub enum PlanetKind {
    RED,
    BLUE,
    WHITE,
}

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
        let (mut planet_asset, mut gravity_asset) = Self::load_assets(kind, asset_manager);
        gravity_asset.dst_rect.z = gravity_radius as i32 * 2;
        gravity_asset.dst_rect.w = gravity_radius as i32 * 2;
        planet_asset.set_center(center);
        gravity_asset.set_center(center);
        let rect = planet_asset.dst_rect;
        let planet_radius = cmp::min(rect.z, rect.w) as f64 / 2.;
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

    pub fn draw<R: Renderer>(&self, renderer: &mut ResourceManager<R>) -> Result<()> {
        self.gravity_asset.draw(None, None, renderer)?;
        self.planet_asset.draw(None, None, renderer)
    }

    fn load_assets(kind: PlanetKind, asset_manager: &AssetManager) -> (Asset, Asset) {
        let (planet, ring) = match kind {
            PlanetKind::RED => (TextureAsset::RedPlanet, TextureAsset::RedRing),
            PlanetKind::BLUE => (TextureAsset::BluePlanet, TextureAsset::BlueRing),
            PlanetKind::WHITE => (TextureAsset::WhitePlanet, TextureAsset::WhiteRing),
        };

        (asset_manager.get_asset(planet), asset_manager.get_asset(ring))
    }
}

impl<I: Intersect<Circle>> Collidable<Circle, I> for Planet {
    fn collides(&self, shape: &I) -> bool {
        shape.intersects(&self.body)
    }
}
