use glm;
use moho::errors::*;
use moho::frame_animator::FrameAnimator;
use moho::renderer::Renderer;
use moho::resource_manager::{ResourceManager, Texture};
use moho::tile_sheet::TileSheet;

use std::time::Duration;

#[derive(Clone)]
pub struct AnimationData {
    pub texture: Texture,
    pub animator: FrameAnimator,
    pub sheet: TileSheet,
}

impl AnimationData {
    pub fn new<R>(path: &'static str,
                  frames: u32,
                  duration_ms: u64,
                  repeat: bool,
                  resource_manager: &ResourceManager<R>)
                  -> Result<AnimationData>
        where R: Renderer
    {
        let texture = resource_manager.load_texture(path)?;
        let duration = Duration::from_millis(duration_ms);
        let sheet = TileSheet::new(glm::uvec2(frames, 1));
        let animator = FrameAnimator::new(frames, duration, repeat);
        let data = AnimationData {
            texture: texture,
            sheet: sheet,
            animator: animator,
        };
        Ok(data)
    }
}

pub fn load_star<R: Renderer>(resource_manager: &ResourceManager<R>) -> Result<AnimationData> {
    static PATH: &'static str = "resources/star.png";
    const FRAMES: u32 = 2;
    const DURATION_MS: u64 = 150;

    AnimationData::new(PATH, FRAMES, DURATION_MS, true, resource_manager)
}

pub fn load_small_explosion<R: Renderer>(resource_manager: &ResourceManager<R>)
                                         -> Result<AnimationData> {
    static PATH: &'static str = "resources/explosion_small.png";
    const FRAMES: u32 = 10;
    const DURATION_MS: u64 = 100;

    AnimationData::new(PATH, FRAMES, DURATION_MS, false, resource_manager)
}
