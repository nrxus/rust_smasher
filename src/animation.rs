use glm;
use moho::tile_sheet::TileSheet;
use moho::frame_animator::FrameAnimator;

pub struct Animation {
    sheet: TileSheet,
    animator: FrameAnimator,
}

impl Animation {
    pub fn new(sheet: TileSheet, animator: FrameAnimator) -> Self {
        Animation {
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
}
