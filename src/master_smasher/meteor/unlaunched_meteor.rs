use master_smasher::asset_manager::Asset;

use glm;
use glm::ext::normalize_to;
use moho::errors::*;
use moho::renderer::Renderer;
use moho::resource_manager::ResourceManager;
use sdl2::rect;

pub struct UnlaunchedMeteor {
    asset: Asset,
    rects: [rect::Rect; 10],
}

impl UnlaunchedMeteor {
    pub fn new(asset: Asset) -> Self {
        let rects = [rect::Rect::new(0, 0, 5, 5); 10];
        UnlaunchedMeteor {
            asset: asset,
            rects: rects,
        }
    }

    pub fn update(&mut self, target: glm::IVec2) {
        let target = glm::to_dvec2(target);
        let center = glm::to_dvec2(self.asset.center());
        let distance = target - center;
        let offset = self.asset.dst_rect.z / 2 + 10;
        let offset_vector = normalize_to(distance, offset as f64);
        let anchor_point = center + offset_vector;
        let step = (target - anchor_point) / (self.rects.len() as f64);

        for (i, rect) in self.rects.iter_mut().enumerate() {
            let point = glm::to_ivec2(anchor_point + (step * i as f64));
            rect.center_on((point.x, point.y));
        }
    }

    pub fn draw<R: Renderer>(&self, renderer: &mut ResourceManager<R>) -> Result<()> {
        self.asset.draw(None, None, renderer)?;
        renderer.fill_rects(&self.rects)
    }
}
