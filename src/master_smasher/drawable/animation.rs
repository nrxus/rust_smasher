use super::asset::Asset;
use super::animation_data::AnimationData;

use glm;
use moho::frame_animator::FrameAnimator;
use moho::tile_sheet::TileSheet;

use std::time::Duration;

#[derive(Clone)]
pub struct Animation {
    pub asset: Asset,
    sheet: TileSheet,
    animator: FrameAnimator,
}

impl Animation {
    pub fn from_data(data: &AnimationData, center: glm::IVec2, scale: glm::DVec2) -> Animation {
        let animator = data.animator.clone();
        let scale = glm::dvec2(1. / animator.num_frames() as f64, 1.) * scale;
        let asset = Asset::scaled_texture(&data.texture, center, scale);
        Self::new(asset, data.sheet.clone(), animator)
    }

    pub fn new(asset: Asset, sheet: TileSheet, animator: FrameAnimator) -> Self {
        Animation {
            asset: asset,
            sheet: sheet,
            animator: animator,
        }
    }

    pub fn update(&mut self, delta: Duration) {
        if self.is_active() {
            self.animator.animate(delta);
        } else {
            self.animator.start();
        }
        if let Some(frame) = self.animator.frame() {
            let src_rect = self.sheet.uv(frame);
            self.asset.src_rect = Some(src_rect);
        }
    }

    pub fn start(&mut self) {
        self.animator.start();
    }

    pub fn is_active(&self) -> bool {
        self.animator.frame().is_some()
    }
}
