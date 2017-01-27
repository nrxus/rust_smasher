mod unlaunched_meteor;

use self::unlaunched_meteor::UnlaunchedMeteor;
use super::asset_manager::{Animation, AnimationAsset, Asset, AssetManager, TextureAsset};
use super::collidable::Collidable;
use super::shape::{Circle, Intersect, Shape};
use super::planet::Planet;

use glm;
use num_traits::Zero;
use moho::errors::*;
use moho::renderer::Renderer;
use moho::resource_manager::ResourceManager;

use std::cmp;

pub enum MeteorState {
    UNLAUNCHED,
    LAUNCHED,
}

pub struct Meteor {
    unlaunched_meteor: UnlaunchedMeteor,
    max_coords: glm::UVec2,
    explosion: Animation,
    asset: Asset,
    state: MeteorState,
    target: glm::IVec2,
    body: Circle,
    initial_center: glm::IVec2,
    velocity: glm::DVec2,
}

impl Meteor {
    pub fn new(center: glm::IVec2, max_coords: glm::UVec2, asset_manager: &AssetManager) -> Self {
        let mut asset = asset_manager.get_asset(TextureAsset::Meteor);
        asset.set_center(center);
        let unlaunched_meteor = UnlaunchedMeteor::new(asset.clone());
        let explosion = asset_manager.get_animation(AnimationAsset::ExplosionLarge);
        let dims = glm::ivec2(asset.dst_rect.z, asset.dst_rect.w);
        let radius = cmp::min(dims.x, dims.y) as f64 / 2.;
        let body = Circle {
            center: glm::to_dvec2(center),
            radius: radius,
        };

        Meteor {
            unlaunched_meteor: unlaunched_meteor,
            max_coords: max_coords,
            asset: asset,
            explosion: explosion,
            state: MeteorState::UNLAUNCHED,
            target: glm::IVec2::zero(),
            body: body,
            initial_center: center,
            velocity: glm::DVec2::zero(),
        }
    }

    pub fn update(&mut self, planets: &[Planet]) {
        match self.state {
            MeteorState::UNLAUNCHED => self.unlaunched_meteor.update(self.target),
            MeteorState::LAUNCHED => {
                self.pull(planets);
                self.displace();
                self.move_drawable();
            }
        }
    }

    pub fn draw<R: Renderer>(&self, renderer: &mut ResourceManager<R>) -> Result<()> {
        match self.state {
            MeteorState::UNLAUNCHED => self.unlaunched_meteor.draw(renderer),
            MeteorState::LAUNCHED => self.asset.draw(None, Some(self.max_coords), renderer),
        }
    }

    pub fn state(&self) -> &MeteorState {
        &self.state
    }

    pub fn explode(&mut self) -> Animation {
        self.explosion.set_center(glm::to_ivec2(self.body.center));
        let explosion = self.explosion.clone();
        self.state = MeteorState::UNLAUNCHED;
        self.body.center = glm::to_dvec2(self.initial_center);
        self.velocity = glm::DVec2::zero();
        self.move_drawable();
        explosion
    }

    pub fn launch(&mut self) {
        const FACTOR: f64 = 50.;
        let offset = self.target - glm::to_ivec2(self.body.center);
        self.velocity = glm::to_dvec2(offset) / FACTOR;
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
        collidable.collides(&self.body)
    }

    fn move_drawable(&mut self) {
        let center = glm::to_ivec2(self.body.center);
        self.asset.set_center(center);
    }

    fn pull(&mut self, planets: &[Planet]) {
        for planet in planets {
            let acceleration = planet.pull_vector(self.body.center, self.body.radius);
            self.velocity = self.velocity + acceleration / 50.;
        }
    }

    fn displace(&mut self) {
        let max_coords = glm::to_dvec2(self.max_coords);
        self.body.center = self.body.center + self.velocity;
        self.body.center = (self.body.center + max_coords) % max_coords;
    }
}
