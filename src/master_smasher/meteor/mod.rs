mod unlaunched_meteor;
mod launched_meteor;

use self::unlaunched_meteor::UnlaunchedMeteor;
use self::launched_meteor::LaunchedMeteor;
use super::asset_manager::{Animation, AnimationAsset, Asset, AssetManager, TextureAsset};
use super::collidable::Collidable;
use super::shape::{Circle, Intersect, Shape};
use super::planet::Planet;

use glm;
use num_traits::Zero;
use moho::errors::*;
use moho::renderer::Renderer;
use moho::resource_manager::ResourceManager;

pub enum MeteorState {
    UNLAUNCHED,
    LAUNCHED(LaunchedMeteor),
}

pub struct Meteor {
    unlaunched_meteor: UnlaunchedMeteor,
    max_coords: glm::UVec2,
    explosion: Animation,
    asset: Asset,
    state: MeteorState,
    target: glm::IVec2,
}

impl Meteor {
    pub fn new(center: glm::IVec2, max_coords: glm::UVec2, asset_manager: &AssetManager) -> Self {
        let mut asset = asset_manager.get_asset(TextureAsset::Meteor);
        asset.set_center(center);
        let unlaunched_meteor = UnlaunchedMeteor::new(asset.clone());
        let explosion = asset_manager.get_animation(AnimationAsset::ExplosionLarge);

        Meteor {
            unlaunched_meteor: unlaunched_meteor,
            max_coords: max_coords,
            asset: asset,
            explosion: explosion,
            state: MeteorState::UNLAUNCHED,
            target: glm::IVec2::zero(),
        }
    }

    pub fn update(&mut self, planets: &[Planet]) {
        match self.state {
            MeteorState::UNLAUNCHED => self.unlaunched_meteor.update(self.target),
            MeteorState::LAUNCHED(ref mut m) => m.update(planets),
        }
    }

    pub fn draw<R: Renderer>(&self, renderer: &mut ResourceManager<R>) -> Result<()> {
        match self.state {
            MeteorState::UNLAUNCHED => self.unlaunched_meteor.draw(renderer),
            MeteorState::LAUNCHED(ref m) => m.draw(renderer),
        }
    }

    pub fn state(&self) -> &MeteorState {
        &self.state
    }

    pub fn explode(&mut self) -> Animation {
        let mut explosion = self.explosion.clone();
        if let MeteorState::LAUNCHED(ref m) = self.state {
            explosion.set_center(glm::to_ivec2(m.center()));
        }
        self.state = MeteorState::UNLAUNCHED;
        explosion
    }

    pub fn launch(&mut self) {
        const FACTOR: f64 = 50.;
        let asset = self.asset.clone();
        let offset = self.target - glm::to_ivec2(asset.center());
        let velocity = glm::to_dvec2(offset) / FACTOR;
        let launched = LaunchedMeteor::new(asset, self.max_coords, velocity);
        self.state = MeteorState::LAUNCHED(launched);
    }

    pub fn update_target(&mut self, target: glm::IVec2) {
        self.target = target;
    }

    pub fn collides<S, C>(&self, collidable: &C) -> bool
        where S: Shape,
              C: Collidable<S, Circle>,
              Circle: Intersect<S>
    {
        match self.state {
            MeteorState::LAUNCHED(ref m) => m.collides(collidable),
            MeteorState::UNLAUNCHED => false,
        }
    }
}
