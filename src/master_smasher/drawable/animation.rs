use super::asset::Asset;
use super::animation_loader::AnimationData;

use glm;
use moho::frame_animator::FrameAnimator;
use moho::tile_sheet::TileSheet;

#[derive(Clone)]
pub struct Animation {
    pub asset: Asset,
    sheet: TileSheet,
    animator: FrameAnimator,
}

impl Animation {
    pub fn start(data: &AnimationData, center: glm::IVec2) -> Animation {
        let dims = data.texture.dims;
        let animator = data.animator.clone();
        let dims = glm::uvec2(dims.x / animator.num_frames(), dims.y);
        let asset = Asset::centered_on(data.texture.id, center, dims);
        Self::new(asset, data.sheet.clone(), animator)
    }

    pub fn new(asset: Asset, sheet: TileSheet, animator: FrameAnimator) -> Self {
        Animation {
            asset: asset,
            sheet: sheet,
            animator: animator,
        }
    }

    pub fn update(&mut self) {
        self.animator.animate();
        let frame = self.animator.frame();
        let src_rect = self.sheet.uv(frame);
        self.asset.src_rect = Some(src_rect);
    }

    pub fn is_active(&self) -> bool {
        self.animator.is_active()
    }
}
