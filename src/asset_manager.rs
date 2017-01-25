use animation::Animation;
use asset::Asset;

use glm;
use moho::errors::*;
use moho::frame_animator::FrameAnimator;
use moho::renderer::Renderer;
use moho::resource_manager::ResourceManager;
use moho::tile_sheet::TileSheet;

use std::time::Duration;

pub enum AnimationAsset {
    ExplosionSmall,
    ExplosionLarge,
    Star,
}

pub struct AssetManager {
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

        let manager = AssetManager {
            star: star,
            explosion_small: explosion_small,
            explosion_large: explosion_large,
        };
        Ok(manager)
    }

    pub fn get_animation(&self, asset: AnimationAsset) -> Animation {
        let animation = match asset {
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

        Self::load_animation(PATH, FRAMES, DURATION_MS, resource_manager)
    }

    fn load_small_explosion<R: Renderer>(resource_manager: &ResourceManager<R>)
                                         -> Result<Animation> {
        static PATH: &'static str = "resources/explosion_small.png";
        const FRAMES: u32 = 10;
        const DURATION_MS: u64 = 100;

        Self::load_animation(PATH, FRAMES, DURATION_MS, resource_manager)
    }

    fn load_large_explosion<R: Renderer>(resource_manager: &ResourceManager<R>)
                                         -> Result<Animation> {
        static PATH: &'static str = "resources/explosion_large.png";
        const FRAMES: u32 = 8;
        const DURATION_MS: u64 = 80;

        Self::load_animation(PATH, FRAMES, DURATION_MS, resource_manager)
    }

    fn load_animation<R>(path: &'static str,
                         frames: u32,
                         duration_ms: u64,
                         resource_manager: &ResourceManager<R>)
                         -> Result<Animation>
        where R: Renderer
    {
        let duration = Duration::from_millis(duration_ms);
        let asset = Self::load_asset(path, resource_manager)?;
        Ok(Self::create_animation(asset, frames, duration))
    }

    fn load_asset<R>(path: &'static str, resource_manager: &ResourceManager<R>) -> Result<Asset>
        where R: Renderer
    {
        let texture = resource_manager.load_texture(path)?;
        Ok(Asset::from_texture(&texture))
    }

    fn create_animation(mut asset: Asset, frames: u32, duration: Duration) -> Animation {
        asset.dst_rect.z /= frames as i32;
        let tile_sheet = TileSheet::new(glm::uvec2(frames, 1));
        let animator = FrameAnimator::new(frames, duration, false);
        Animation::new(asset, tile_sheet, animator)
    }
}
