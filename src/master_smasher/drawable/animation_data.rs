use glm;
use moho::errors::*;
use moho::resource_manager::{FrameAnimator, ResourceLoader, TileSheet};

use std::time::Duration;

#[derive(Clone)]
pub struct AnimationData {
    pub animator: FrameAnimator,
    pub sheet: TileSheet,
}

impl AnimationData {
    pub fn new<R>(path: &'static str,
                  frames: u32,
                  duration_ms: u64,
                  repeat: bool,
                  resource_manager: &R)
                  -> Result<AnimationData>
        where R: ResourceLoader
    {
        let texture = resource_manager.load_texture(path)?;
        let duration = Duration::from_millis(duration_ms);
        let sheet = TileSheet::new(glm::uvec2(frames, 1), texture);
        let animator = FrameAnimator::new(frames, duration, repeat);
        let data = AnimationData {
            sheet: sheet,
            animator: animator,
        };
        Ok(data)
    }
}
