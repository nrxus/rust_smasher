use animation::Animation;
use asset::Asset;

use glm;
use moho::errors::*;
use moho::frame_animator::FrameAnimator;
use moho::renderer::Renderer;
use moho::resource_manager::ResourceManager;
use moho::tile_sheet::TileSheet;

use std::time::Duration;

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
    red_planet: Asset,
    white_planet: Asset,
    blue_planet: Asset,
    red_ring: Asset,
    white_ring: Asset,
    blue_ring: Asset,
    meteor: Asset,

    explosion_small: Animation,
    explosion_large: Animation,
    star: Animation,
}

impl AssetManager {
    pub fn new<R>(resource_manager: &ResourceManager<R>) -> Result<Self>
        where R: Renderer
    {
        let star = Self::load_star(resource_manager)?;
        let explosion_small = Self::load_small_explosion(resource_manager)?;
        let explosion_large = Self::load_large_explosion(resource_manager)?;
        let red_planet = Self::load_asset("resources/red_planet.png", resource_manager)?;
        let white_planet = Self::load_asset("resources/white_planet.png", resource_manager)?;
        let blue_planet = Self::load_asset("resources/blue_planet.png", resource_manager)?;
        let red_ring = Self::load_asset("resources/red_ring.png", resource_manager)?;
        let white_ring = Self::load_asset("resources/white_ring.png", resource_manager)?;
        let blue_ring = Self::load_asset("resources/blue_ring.png", resource_manager)?;
        let meteor = Self::load_asset("resources/meteor.png", resource_manager)?;

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

    pub fn get_asset(&self, kind: TextureAsset) -> Asset {
        let asset = match kind {
            TextureAsset::RedPlanet => &self.red_planet,
            TextureAsset::WhitePlanet => &self.white_planet,
            TextureAsset::BluePlanet => &self.blue_planet,
            TextureAsset::RedRing => &self.red_ring,
            TextureAsset::WhiteRing => &self.white_ring,
            TextureAsset::BlueRing => &self.blue_ring,
            TextureAsset::Meteor => &self.meteor,
        };
        asset.clone()
    }

    pub fn get_animation(&self, kind: AnimationAsset) -> Animation {
        let animation = match kind {
            AnimationAsset::Star => &self.star,
            AnimationAsset::ExplosionSmall => &self.explosion_small,
            AnimationAsset::ExplosionLarge => &self.explosion_large,
        };
        animation.clone()
    }

    fn load_star<R: Renderer>(resource_manager: &ResourceManager<R>) -> Result<Animation> {
        static PATH: &'static str = "resources/star.png";
        const FRAMES: u32 = 2;
        const DURATION_MS: u64 = 150;

        Self::load_animation(PATH, FRAMES, DURATION_MS, true, resource_manager)
    }

    fn load_small_explosion<R: Renderer>(resource_manager: &ResourceManager<R>)
                                         -> Result<Animation> {
        static PATH: &'static str = "resources/explosion_small.png";
        const FRAMES: u32 = 10;
        const DURATION_MS: u64 = 100;

        Self::load_animation(PATH, FRAMES, DURATION_MS, false, resource_manager)
    }

    fn load_large_explosion<R: Renderer>(resource_manager: &ResourceManager<R>)
                                         -> Result<Animation> {
        static PATH: &'static str = "resources/explosion_large.png";
        const FRAMES: u32 = 8;
        const DURATION_MS: u64 = 80;

        Self::load_animation(PATH, FRAMES, DURATION_MS, false, resource_manager)
    }

    fn load_animation<R>(path: &'static str,
                         frames: u32,
                         duration_ms: u64,
                         repeat: bool,
                         resource_manager: &ResourceManager<R>)
                         -> Result<Animation>
        where R: Renderer
    {
        let duration = Duration::from_millis(duration_ms);
        let asset = Self::load_asset(path, resource_manager)?;
        Ok(Self::create_animation(asset, frames, duration, repeat))
    }

    fn load_asset<R>(path: &'static str, resource_manager: &ResourceManager<R>) -> Result<Asset>
        where R: Renderer
    {
        let texture = resource_manager.load_texture(path)?;
        Ok(Asset::from_texture(&texture))
    }

    fn create_animation(mut asset: Asset,
                        frames: u32,
                        duration: Duration,
                        repeat: bool)
                        -> Animation {
        asset.dst_rect.z /= frames as i32;
        let tile_sheet = TileSheet::new(glm::uvec2(frames, 1));
        let animator = FrameAnimator::new(frames, duration, repeat);
        Animation::new(asset, tile_sheet, animator)
    }
}
