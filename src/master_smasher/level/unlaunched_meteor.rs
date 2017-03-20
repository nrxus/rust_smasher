use master_smasher::drawable::Rectifiable;
use master_smasher::shape::Circle;
use super::player_assets::PlayerAssets;
use super::MeteorState;
use super::interpolate::State;
use super::launched_meteor::LaunchedMeteor;
use errors::*;

use glm;
use glm::ext::normalize_to;
use moho::renderer::Renderer;
use moho::resource_manager::{ResourceManager, TextureId};
use sdl2::rect;

use std::cmp;

pub struct UnlaunchedMeteor {
    body: Circle,
    texture: TextureId,
    target: State<glm::IVec2>,
}

impl UnlaunchedMeteor {
    pub fn load(assets: &PlayerAssets, center: glm::IVec2) -> Self {
        let texture = assets.meteor;
        let center = glm::to_dvec2(center);
        let radius = cmp::min(texture.dims.x, texture.dims.y) as f64 / 2.;
        let body = Circle {
            center: center,
            radius: radius,
        };

        Self::new(body, texture.id)
    }

    pub fn new(body: Circle, texture: TextureId) -> Self {
        UnlaunchedMeteor {
            body: body,
            texture: texture,
            target: State::new(glm::ivec2(0, 0)),
        }
    }

    pub fn update(&mut self, target: glm::IVec2) {
        self.target.update(target);
    }

    pub fn draw<R>(&self, interpolation: f64, renderer: &mut ResourceManager<R>) -> Result<()>
        where R: Renderer
    {
        let target = glm::to_dvec2(self.target.interpolated(interpolation));
        let rects = self.target_rects(target, self.body.center);
        renderer.render(&self.texture, self.body.rectify())?;
        renderer.fill_rects(&rects).map_err(Into::into)
    }

    fn target_rects(&self, target: glm::DVec2, center: glm::DVec2) -> Vec<rect::Rect> {
        const NUM_RECTS: u32 = 10;
        const SIDE_LEN: u32 = 5;

        let distance = target - center;
        let offset = self.body.radius + 10.;
        let offset_vector = normalize_to(distance, offset);
        let anchor_point = center + offset_vector;
        let step = (target - anchor_point) / NUM_RECTS as f64;

        (0..NUM_RECTS)
            .map(|i| anchor_point + (step * i as f64))
            .map(|p| (p.x as i32, p.y as i32))
            .map(|p| rect::Rect::from_center(p, SIDE_LEN, SIDE_LEN))
            .collect()
    }

    pub fn launch(&self, max_coords: glm::UVec2) -> MeteorState {
        const FACTOR: f64 = 50.;
        let offset = self.target.current - glm::to_ivec2(self.body.center);
        let velocity = glm::to_dvec2(offset) / FACTOR;
        MeteorState::LAUNCHED(LaunchedMeteor::new(self.body, self.texture, max_coords, velocity))
    }
}
