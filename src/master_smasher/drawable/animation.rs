use super::{GameRenderer, Scene};
use super::animation_data::AnimationData;
use errors::*;

use glm;
use moho::frame_animator::FrameAnimator;
use moho::renderer::Renderer;
use moho::resource_manager::ResourceManager;
use moho::tile_sheet::TileSheet;

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

impl<R: Renderer> Scene<ResourceManager<R>> for Animation {
    fn show(&self, renderer: &mut ResourceManager<R>) -> Result<()> {
        if let Some(frame) = self.animator.frame() {
            let tile = self.sheet.tile(frame);
            renderer.render(&tile, self.dst_rect)
        } else {
            Ok(())
        }
    }
}
