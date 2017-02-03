use super::asset::Asset;

use glm;
use moho::errors::*;
use moho::renderer::Renderer;
use moho::resource_manager::{ResourceManager, Texture};

pub enum TextureAsset {
    RedPlanet,
    WhitePlanet,
    BluePlanet,
    DeadPlanet,
    RedRing,
    WhiteRing,
    BlueRing,
}

pub struct AssetManager {
    red_planet: Texture,
    white_planet: Texture,
    blue_planet: Texture,
    dead_planet: Texture,
    red_ring: Texture,
    white_ring: Texture,
    blue_ring: Texture,
}

impl AssetManager {
    pub fn new<R>(resource_manager: &ResourceManager<R>) -> Result<Self>
        where R: Renderer
    {
        let red_planet = resource_manager.load_texture("resources/red_planet.png")?;
        let white_planet = resource_manager.load_texture("resources/white_planet.png")?;
        let blue_planet = resource_manager.load_texture("resources/blue_planet.png")?;
        let dead_planet = resource_manager.load_texture("resources/dead_planet.png")?;
        let red_ring = resource_manager.load_texture("resources/red_ring.png")?;
        let white_ring = resource_manager.load_texture("resources/white_ring.png")?;
        let blue_ring = resource_manager.load_texture("resources/blue_ring.png")?;

        let manager = AssetManager {
            red_planet: red_planet,
            white_planet: white_planet,
            blue_planet: blue_planet,
            dead_planet: dead_planet,
            red_ring: red_ring,
            white_ring: white_ring,
            blue_ring: blue_ring,
        };
        Ok(manager)
    }

    pub fn get_asset(&self, kind: TextureAsset, center: glm::IVec2) -> Asset {
        let texture = match kind {
            TextureAsset::RedPlanet => &self.red_planet,
            TextureAsset::WhitePlanet => &self.white_planet,
            TextureAsset::BluePlanet => &self.blue_planet,
            TextureAsset::DeadPlanet => &self.dead_planet,
            TextureAsset::RedRing => &self.red_ring,
            TextureAsset::WhiteRing => &self.white_ring,
            TextureAsset::BlueRing => &self.blue_ring,
        };
        Asset::from_texture(texture, center)
    }
}
