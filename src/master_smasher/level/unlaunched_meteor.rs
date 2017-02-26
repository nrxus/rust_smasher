use master_smasher::drawable::{Asset, Drawable};
use super::MeteorState;
use super::launched_meteor::LaunchedMeteor;

use glm;
use glm::ext::normalize_to;
use sdl2::rect;

pub struct UnlaunchedMeteor {
    asset: Asset,
    rects: [rect::Rect; 10],
    target: glm::IVec2,
}

impl UnlaunchedMeteor {
    pub fn new(asset: Asset) -> Self {
        let rects = [rect::Rect::new(0, 0, 5, 5); 10];
        UnlaunchedMeteor {
            asset: asset,
            rects: rects,
            target: glm::ivec2(0, 0),
        }
    }

    pub fn update(&mut self, target: glm::IVec2) {
        self.target = target;
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

    pub fn drawables(&self) -> Vec<Drawable> {
        vec![Drawable::Asset(self.asset), Drawable::Rectangles(self.rects.to_vec())]
    }

    pub fn launch(&self, max_coords: glm::UVec2) -> MeteorState {
        const FACTOR: f64 = 50.;
        let asset = self.asset.clone();
        let offset = self.target - glm::to_ivec2(asset.center());
        let velocity = glm::to_dvec2(offset) / FACTOR;
        MeteorState::LAUNCHED(LaunchedMeteor::new(asset, max_coords, velocity))
    }
}
