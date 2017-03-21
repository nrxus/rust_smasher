use super::animation_data::AnimationData;

use glm;
use moho::resource_manager::{FrameAnimator, Renderer, Scene, TileSheet};
use moho::errors as moho_errors;

use std::time::Duration;

#[derive(Clone)]
pub struct Animation {
    pub dst_rect: glm::IVec4,
    sheet: TileSheet,
    animator: FrameAnimator,
    started: bool,
}

impl Animation {
    pub fn from_data(data: AnimationData, center: glm::IVec2, scale: glm::DVec2) -> Animation {
        let dims = glm::to_ivec2(glm::to_dvec2(data.sheet.dimensions) * scale);
        let dst_rect = glm::ivec4(center.x - dims.x / 2, center.y - dims.y / 2, dims.x, dims.y);
        Self::new(dst_rect, data.sheet.clone(), data.animator)
    }

    pub fn new(dst_rect: glm::IVec4, sheet: TileSheet, animator: FrameAnimator) -> Self {
        Animation {
            dst_rect: dst_rect,
            sheet: sheet,
            animator: animator,
            started: false,
        }
    }

    pub fn update(&mut self, delta: Duration) {
        if !self.started {
            self.animator.start();
            self.started = true;
        } else if self.animator.frame().is_some() {
            self.animator.animate(delta);
        }
    }

    pub fn is_active(&self) -> bool {
        self.animator.frame().is_some()
    }
}

impl Scene for Animation {
    fn show<R: Renderer>(&self, renderer: &mut R) -> moho_errors::Result<()> {
        if let Some(frame) = self.animator.frame() {
            let tile = self.sheet.tile(frame);
            renderer.render(&tile, self.dst_rect)
        } else {
            Ok(())
        }
    }
}
