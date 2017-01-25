use asset::Asset;
use asset_manager::{AssetManager, TextureAsset};

use glm;
use moho::resource_manager::ResourceManager;
use moho::renderer::Renderer;
use moho::errors::*;

pub enum PlanetKind {
    RED,
    BLUE,
    WHITE,
}

pub struct Drawable {
    planet: Asset,
    gravity: Asset,
}

impl Drawable {
    pub fn new(center: glm::IVec2,
               gravity_radius: u32,
               kind: PlanetKind,
               asset_manager: &AssetManager)
               -> Self {
        let (mut planet, mut gravity) = Self::load_assets(kind, asset_manager);
        gravity.dst_rect.z = gravity_radius as i32 * 2;
        gravity.dst_rect.w = gravity_radius as i32 * 2;
        planet.set_center(center);
        gravity.set_center(center);

        Drawable {
            planet: planet,
            gravity: gravity,
        }
    }

    pub fn planet_dims(&self) -> glm::UVec2 {
        glm::uvec2(self.planet.dst_rect.z as u32, self.planet.dst_rect.w as u32)
    }

    pub fn draw<R>(&self, renderer: &mut ResourceManager<R>) -> Result<()>
        where R: Renderer
    {
        self.gravity.draw(None, None, renderer)?;
        self.planet.draw(None, None, renderer)
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
