mod object;

use self::object::Object;
use super::asset_manager::{Animation, AnimationAsset, Asset, AssetManager, TextureAsset};
use super::collidable::Collidable;
use super::shape::{Circle, Intersect, Shape};
use super::planet::Planet;

use glm;
use glm::ext::normalize_to;
use num_traits::Zero;
use moho::errors::*;
use moho::renderer::Renderer;
use moho::resource_manager::ResourceManager;
use sdl2::rect;

use std::cmp;

pub enum MeteorState {
    UNLAUNCHED,
    LAUNCHED,
}

pub struct Meteor {
    max_coords: glm::UVec2,
    explosion: Animation,
    asset: Asset,
    rects: [rect::Rect; 10],
    object: Object,
    state: MeteorState,
    target: glm::IVec2,
}

impl Meteor {
    pub fn new(center: glm::IVec2, max_coords: glm::UVec2, asset_manager: &AssetManager) -> Self {
        let mut asset = asset_manager.get_asset(TextureAsset::Meteor);
        asset.set_center(center);
        let explosion = asset_manager.get_animation(AnimationAsset::ExplosionLarge);
        let dims = glm::ivec2(asset.dst_rect.z, asset.dst_rect.w);
        let radius = cmp::min(dims.x, dims.y) as f64 / 2.;
        let object = Object::new(glm::to_dvec2(center), radius, glm::to_dvec2(max_coords));

        Meteor {
            max_coords: max_coords,
            asset: asset,
            explosion: explosion,
            rects: [rect::Rect::new(0, 0, 5, 5); 10],
            object: object,
            state: MeteorState::UNLAUNCHED,
            target: glm::IVec2::zero(),
        }
    }

    pub fn update(&mut self, planets: &[Planet]) {
        match self.state {
            MeteorState::UNLAUNCHED => {
                self.update_launch_vector();
            }
            MeteorState::LAUNCHED => {
                self.object.update(planets);
                self.move_drawable();
            }
        }
    }

    pub fn draw<R: Renderer>(&self, renderer: &mut ResourceManager<R>) -> Result<()> {
        match self.state {
            MeteorState::UNLAUNCHED => {
                self.asset.draw(None, Some(self.max_coords), renderer)?;
                renderer.fill_rects(&self.rects)
            }
            MeteorState::LAUNCHED => self.asset.draw(None, Some(self.max_coords), renderer),
        }
    }

    pub fn state(&self) -> &MeteorState {
        &self.state
    }

    pub fn explode(&mut self) -> Animation {
        self.explosion.set_center(glm::to_ivec2(self.object.body().center));
        let explosion = self.explosion.clone();
        self.state = MeteorState::UNLAUNCHED;
        self.object.restart();
        self.move_drawable();
        explosion
    }

    pub fn launch(&mut self) {
        self.object.launch(self.target);
        self.state = MeteorState::LAUNCHED;
    }

    pub fn update_target(&mut self, target: glm::IVec2) {
        self.target = target;
    }

    pub fn collides<S, C>(&self, collidable: &C) -> bool
        where S: Shape,
              C: Collidable<S, Circle>,
              Circle: Intersect<S>
    {
        collidable.collides(self.object.body())
    }

    fn move_drawable(&mut self) {
        let center = glm::to_ivec2(self.object.body().center);
        self.asset.set_center(center);
    }

    fn update_launch_vector(&mut self) {
        let target = glm::to_dvec2(self.target);
        let center = glm::to_dvec2(self.object.body().center);
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
}
