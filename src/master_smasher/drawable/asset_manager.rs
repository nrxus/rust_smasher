use super::animation::Animation;
use super::animation_loader::*;
use super::asset::Asset;

use glm;
use moho::errors::*;
use moho::renderer::Renderer;
use moho::resource_manager::{ResourceManager, Texture};

pub enum TextureAsset {
    RedPlanet,
    WhitePlanet,
    BluePlanet,
    RedRing,
    WhiteRing,
    BlueRing,
    Meteor,
}

pub enum AnimationAsset {
    ExplosionSmall,
    ExplosionLarge,
    Star,
}

pub struct AssetManager {
    red_planet: Texture,
    white_planet: Texture,
    blue_planet: Texture,
    red_ring: Texture,
    white_ring: Texture,
    blue_ring: Texture,
    meteor: Texture,

    explosion_small: AnimationData,
    explosion_large: AnimationData,
    star: AnimationData,
}

impl AssetManager {
    pub fn new<R>(resource_manager: &ResourceManager<R>) -> Result<Self>
        where R: Renderer
    {
        let star = load_star(resource_manager)?;
        let explosion_small = load_small_explosion(resource_manager)?;
        let explosion_large = load_large_explosion(resource_manager)?;
        let red_planet = resource_manager.load_texture("resources/red_planet.png")?;
        let white_planet = resource_manager.load_texture("resources/white_planet.png")?;
        let blue_planet = resource_manager.load_texture("resources/blue_planet.png")?;
        let red_ring = resource_manager.load_texture("resources/red_ring.png")?;
        let white_ring = resource_manager.load_texture("resources/white_ring.png")?;
        let blue_ring = resource_manager.load_texture("resources/blue_ring.png")?;
        let meteor = resource_manager.load_texture("resources/meteor.png")?;

        let manager = AssetManager {
            star: star,
            explosion_small: explosion_small,
            explosion_large: explosion_large,
            red_planet: red_planet,
            white_planet: white_planet,
            blue_planet: blue_planet,
            red_ring: red_ring,
            white_ring: white_ring,
            blue_ring: blue_ring,
            meteor: meteor,
        };
        Ok(manager)
    }

    pub fn get_asset(&self, kind: TextureAsset, center: glm::IVec2) -> Asset {
        let texture = match kind {
            TextureAsset::RedPlanet => &self.red_planet,
            TextureAsset::WhitePlanet => &self.white_planet,
            TextureAsset::BluePlanet => &self.blue_planet,
            TextureAsset::RedRing => &self.red_ring,
            TextureAsset::WhiteRing => &self.white_ring,
            TextureAsset::BlueRing => &self.blue_ring,
            TextureAsset::Meteor => &self.meteor,
        };
        Asset::from_texture(texture, center)
    }

    pub fn get_animation(&self, kind: AnimationAsset, center: glm::IVec2) -> Animation {
        let data = match kind {
            AnimationAsset::Star => &self.star,
            AnimationAsset::ExplosionSmall => &self.explosion_small,
            AnimationAsset::ExplosionLarge => &self.explosion_large,
        };
        let dims = data.texture.dims;
        let dims = glm::uvec2(dims.x / data.animator.num_frames(), dims.y);
        let asset = Asset::centered_on(data.texture.id, center, dims);
        Animation::new(asset, data.sheet.clone(), data.animator.clone())
    }
}
