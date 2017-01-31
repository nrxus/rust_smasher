use super::asset_manager::{Animation, AnimationAsset, Asset, AssetManager, TextureAsset};
use super::meteor::{UnlaunchedMeteor, LaunchedMeteor};
use super::planet::{Planet, PlanetKind};
use super::star::Star;

use glm;
use moho::errors::*;
use moho::input_manager::{EventPump, InputManager};
use moho::renderer::Renderer;
use moho::resource_manager::ResourceManager;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;

pub enum MeteorState {
    UNLAUNCHED(UnlaunchedMeteor),
    LAUNCHED(LaunchedMeteor),
}

pub struct Level {
    planets: Vec<Planet>,
    stars: Vec<Star>,
    animations: Vec<Animation>,
    state: MeteorState,
    max_coords: glm::UVec2,
    explosion: Animation,
    asset: Asset,
}

impl Level {
    pub fn new(window_size: glm::UVec2, asset_manager: &AssetManager) -> Level {
        let blue_center = glm::ivec2(840, 478);
        let white_center = glm::ivec2(346, 298);
        let meteor_center = glm::ivec2(130, 402);
        let star_center = glm::ivec2(500, 130);
        let blue_planet = Planet::new(blue_center, 700., 215., PlanetKind::BLUE, asset_manager);
        let white_planet = Planet::new(white_center, 400., 175., PlanetKind::WHITE, asset_manager);
        let star = Star::new(star_center, asset_manager);
        let mut asset = asset_manager.get_asset(TextureAsset::Meteor);
        asset.set_center(meteor_center);
        let unlaunched_meteor = UnlaunchedMeteor::new(asset.clone());
        let state = MeteorState::UNLAUNCHED(unlaunched_meteor);
        let explosion = asset_manager.get_animation(AnimationAsset::ExplosionLarge);

        Level {
            planets: vec![blue_planet, white_planet],
            stars: vec![star],
            animations: Vec::new(),
            state: state,
            max_coords: window_size,
            explosion: explosion,
            asset: asset,
        }
    }

    pub fn update<E: EventPump>(&mut self, input_manager: &InputManager<E>) {
        let next_state = match self.state {
            MeteorState::UNLAUNCHED(_) if input_manager.did_click_mouse(MouseButton::Left) => {
                let meteor = self.launch(input_manager.mouse_coords());
                Some(MeteorState::LAUNCHED(meteor))
            }
            MeteorState::LAUNCHED(ref mut m) if input_manager.did_press_key(Keycode::R) => {
                let explosion = m.explode();
                self.animations.push(explosion);
                let unlaunched_meteor = UnlaunchedMeteor::new(self.asset.clone());
                Some(MeteorState::UNLAUNCHED(unlaunched_meteor))
            }
            _ => None,
        };

        if let Some(s) = next_state {
            self.state = s;
        }

        let next_state = if let MeteorState::LAUNCHED(ref mut m) = self.state {
            m.update(&self.planets);
            let collidable_indices = self.stars
                .iter()
                .enumerate()
                .filter(|&(_, s)| m.collides(s))
                .map(|(i, _)| i)
                .collect::<Vec<_>>();
            for i in collidable_indices {
                let star = self.stars.swap_remove(i);
                self.animations.push(star.explode());
            }
            if self.planets.iter().any(|p| m.collides(p)) {
                let explosion = m.explode();
                self.animations.push(explosion);
                let unlaunched_meteor = UnlaunchedMeteor::new(self.asset.clone());
                Some(MeteorState::UNLAUNCHED(unlaunched_meteor))
            } else {
                None
            }
        } else {
            None
        };

        if let Some(s) = next_state {
            self.state = s;
        }

        if let MeteorState::UNLAUNCHED(ref mut m) = self.state {
            m.update(input_manager.mouse_coords());
        }

        for star in &mut self.stars {
            star.update();
        }

        for animation in &mut self.animations {
            animation.update();
        }

        self.animations.retain(Animation::is_active);
    }

    pub fn draw<R: Renderer>(&self, renderer: &mut ResourceManager<R>) -> Result<()> {
        for star in &self.stars {
            star.draw(renderer)?;
        }
        for planet in &self.planets {
            planet.draw(renderer)?;
        }
        for animation in &self.animations {
            animation.asset.draw(renderer)?;
        }
        match self.state {
            MeteorState::LAUNCHED(ref m) => m.draw(renderer),
            MeteorState::UNLAUNCHED(ref m) => m.draw(renderer),
        }
    }

    fn launch(&self, target: glm::IVec2) -> LaunchedMeteor {
        const FACTOR: f64 = 50.;
        let asset = self.asset.clone();
        let offset = target - glm::to_ivec2(asset.center());
        let velocity = glm::to_dvec2(offset) / FACTOR;
        LaunchedMeteor::new(asset, self.explosion.clone(), self.max_coords, velocity)
    }
}
