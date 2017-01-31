use super::asset::Asset;

use moho::frame_animator::FrameAnimator;
use moho::tile_sheet::TileSheet;

#[derive(Clone)]
pub struct Animation {
    pub asset: Asset,
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
        let frame = self.animator.frame();
        let src_rect = self.sheet.uv(frame);
        self.asset.src_rect = Some(src_rect);
    }

    pub fn is_active(&self) -> bool {
        self.animator.is_active()
    }
}
