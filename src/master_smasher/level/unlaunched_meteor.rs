use master_smasher::drawable::{Asset, Drawable};
use super::MeteorState;
use super::launched_meteor::LaunchedMeteor;

use glm;
use glm::ext::normalize_to;
use sdl2::rect;

pub struct UnlaunchedMeteor {
    asset: Asset,
    target: glm::IVec2,
}

impl UnlaunchedMeteor {
    pub fn new(asset: Asset) -> Self {
        UnlaunchedMeteor {
            asset: asset,
            target: glm::ivec2(0, 0),
        }
    }

    pub fn update(&mut self, target: glm::IVec2) {
        self.target = target;
    }

    pub fn drawables(&self) -> Vec<Drawable> {
        const NUM_RECTS: u32 = 10;
        const SIDE_LEN: u32 = 5;

        let target = glm::to_dvec2(self.target);
        let center = glm::to_dvec2(self.asset.center());
        let distance = target - center;
        let offset = self.asset.dst_rect.z / 2 + 10;
        let offset_vector = normalize_to(distance, offset as f64);
        let anchor_point = center + offset_vector;
        let step = (target - anchor_point) / NUM_RECTS as f64;

        let rects = (0..NUM_RECTS)
            .map(|i| anchor_point + (step * i as f64))
            .map(|p| (p.x as i32, p.y as i32))
            .map(|p| rect::Rect::from_center(p, SIDE_LEN, SIDE_LEN))
            .collect();

        vec![Drawable::Asset(self.asset), Drawable::Rectangles(rects)]
    }

    pub fn launch(&self, max_coords: glm::UVec2) -> MeteorState {
        const FACTOR: f64 = 50.;
        let offset = self.target - glm::to_ivec2(self.asset.center());
        let velocity = glm::to_dvec2(offset) / FACTOR;
        MeteorState::LAUNCHED(LaunchedMeteor::new(self.asset, max_coords, velocity))
    }
}
