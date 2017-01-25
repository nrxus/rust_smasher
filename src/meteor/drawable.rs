use animation::Animation;
use asset::Asset;
use asset_manager::{AnimationAsset, AssetManager, TextureAsset};

use glm;
use glm::ext::normalize_to;
use moho::errors::*;
use moho::renderer::Renderer;
use moho::resource_manager::ResourceManager;
use sdl2::rect;

pub struct Drawable {
    max_coords: glm::UVec2,
    explosion: Animation,
    meteor: Asset,
    rects: [rect::Rect; 10],
}

impl Drawable {
    pub fn new(center: glm::IVec2, max_coords: glm::UVec2, asset_manager: &AssetManager) -> Self {
        let mut meteor = asset_manager.get_asset(TextureAsset::Meteor);
        meteor.set_center(center);
        let explosion = asset_manager.get_animation(AnimationAsset::ExplosionLarge);

        Drawable {
            max_coords: max_coords,
            meteor: meteor,
            explosion: explosion,
            rects: [rect::Rect::new(0, 0, 5, 5); 10],
        }
    }

    pub fn animate_explosion(&mut self) {
        self.explosion.update();
    }

    pub fn update_launch_vector(&mut self, target: glm::IVec2) {
        let target = glm::to_dvec2(target);
        let rect = self.meteor.dst_rect;
        let center = glm::ivec2(rect.x + rect.z / 2, rect.y + rect.w / 2);
        let center = glm::to_dvec2(center);
        let distance = target - center;
        let offset = self.meteor.dst_rect.z / 2 + 10;
        let offset_vector = normalize_to(distance, offset as f64);
        let anchor_point = center + offset_vector;
        let step = (target - anchor_point) / (self.rects.len() as f64);

        for (i, rect) in self.rects.iter_mut().enumerate() {
            let point = glm::to_ivec2(anchor_point + (step * i as f64));
            rect.center_on((point.x, point.y));
        }
    }

    pub fn meteor_dims(&self) -> glm::UVec2 {
        glm::uvec2(self.meteor.dst_rect.z as u32, self.meteor.dst_rect.w as u32)
    }

    pub fn draw_unlaunched<R: Renderer>(&self, renderer: &mut ResourceManager<R>) -> Result<()> {
        self.draw_meteor(renderer)?;
        renderer.fill_rects(&self.rects)
    }

    pub fn draw_meteor<R: Renderer>(&self, renderer: &mut ResourceManager<R>) -> Result<()> {
        self.meteor.draw(None, Some(self.max_coords), renderer)
    }

    pub fn draw_explosion<R: Renderer>(&self, renderer: &mut ResourceManager<R>) -> Result<()> {
        self.explosion.draw(Some(self.max_coords), renderer)
    }

    pub fn is_exploding(&self) -> bool {
        self.explosion.is_active()
    }

    pub fn set_center(&mut self, center: glm::IVec2) {
        self.meteor.set_center(center);
        self.explosion.set_center(center);
    }
}
