use master_smasher::drawable::{Asset, Drawable};
use super::MeteorState;
use super::launched_meteor::LaunchedMeteor;

use glm;
use glm::ext::normalize_to;
use sdl2::rect;

pub trait Interpolate {
    fn interpolate(&self, next: Self, interpolation: f64) -> Self;
}

impl Interpolate for glm::IVec2 {
    fn interpolate(&self, next: glm::IVec2, interpolation: f64) -> glm::IVec2 {
        let delta = next - *self;
        *self + glm::to_ivec2((glm::to_dvec2(delta) * interpolation))
    }
}

pub struct State<T> {
    old: T,
    current: T,
}

impl<T: Copy + Interpolate> State<T> {
    pub fn new(state: T) -> Self {
        State {
            old: state,
            current: state,
        }
    }

    pub fn update(&mut self, new: T) {
        self.old = self.current;
        self.current = new;
    }

    pub fn interpolated(&self, interpolation: f64) -> T {
        self.old.interpolate(self.current, interpolation)
    }
}

pub struct UnlaunchedMeteor {
    asset: Asset,
    target: State<glm::IVec2>,
}

impl UnlaunchedMeteor {
    pub fn new(asset: Asset) -> Self {
        UnlaunchedMeteor {
            asset: asset,
            target: State::new(glm::ivec2(0, 0)),
        }
    }

    pub fn update(&mut self, target: glm::IVec2) {
        self.target.update(target);
    }

    pub fn drawables(&self, interpolation: f64) -> Vec<Drawable> {
        const NUM_RECTS: u32 = 10;
        const SIDE_LEN: u32 = 5;

        let target = glm::to_dvec2(self.target.interpolated(interpolation));
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
        let offset = self.target.current - glm::to_ivec2(self.asset.center());
        let velocity = glm::to_dvec2(offset) / FACTOR;
        MeteorState::LAUNCHED(LaunchedMeteor::new(self.asset, max_coords, velocity))
    }
}
