use asset::Asset;

use glm;
use moho::errors::*;
use moho::frame_animator::FrameAnimator;
use moho::renderer::Renderer;
use moho::resource_manager::ResourceManager;
use moho::tile_sheet::TileSheet;

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

    pub fn set_center(&mut self, center: glm::IVec2) {
        self.asset.set_center(center);
    }

    pub fn is_active(&self) -> bool {
        self.animator.is_active()
    }

    pub fn draw<R>(&self,
                   wrapping: Option<glm::UVec2>,
                   renderer: &mut ResourceManager<R>)
                   -> Result<()>
        where R: Renderer
    {
        let src = Some(self.src_rect());
        self.asset.draw(src, wrapping, renderer)
    }

    fn src_rect(&self) -> glm::DVec4 {
        let frame = self.animator.frame();
        self.sheet.uv(frame)
    }
}
