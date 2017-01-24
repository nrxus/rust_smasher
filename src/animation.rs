use asset::Asset;

use glm;
use moho::tile_sheet::TileSheet;
use moho::frame_animator::FrameAnimator;

pub struct Animation {
    asset: Asset,
    sheet: TileSheet,
    animator: FrameAnimator,
}

impl Animation {
    pub fn new(asset: Asset, sheet: TileSheet, animator: FrameAnimator) -> Self {
        Animation {
            asset: asset,
            sheet: sheet,
            animator: animator,
        }
    }

    pub fn update(&mut self) {
        self.animator.animate();
    }

    pub fn src_rect(&self) -> glm::DVec4 {
        let frame = self.animator.frame();
        self.sheet.uv(frame)
    }

    pub fn is_active(&self) -> bool {
        self.animator.is_active()
    }

    pub fn asset(&self) -> &Asset {
        &self.asset
    }
}
