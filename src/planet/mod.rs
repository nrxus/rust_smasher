mod object;

use self::object::Object;

use std::cmp;

use asset::Asset;
use asset_manager::{AssetManager, TextureAsset};
use circle::Circle;
use collidable::Collidable;
use shape::Intersect;
use glm;
use moho::errors::*;
use moho::renderer::Renderer;
use moho::resource_manager::ResourceManager;

pub enum PlanetKind {
    RED,
    BLUE,
    WHITE,
}

pub struct Planet {
    object: Object,
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
        let object = Object::new(center, strength, planet_radius, gravity_radius);

        Planet {
            object: object,
            planet_asset: planet_asset,
            gravity_asset: gravity_asset,
        }
    }

    pub fn pull_vector(&self, point: glm::DVec2, radius: f64) -> glm::DVec2 {
        self.object.pull_vector(point, radius)
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
    fn collides(&self, collision: &I) -> bool {
        self.object.collides_with(collision)
    }
}
