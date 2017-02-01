use super::drawable::{Animation, AnimationAsset, Asset, Drawable, AssetManager, TextureAsset};
use super::meteor::{UnlaunchedMeteor, LaunchedMeteor};
use super::planet::{Planet, PlanetKind};
use super::star::Star;
use super::super::errors::*;

use glm;
use moho::input_manager::{EventPump, InputManager};
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use serde_yaml;

use std::fs::File;

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

#[derive(Debug,Deserialize)]
pub struct ObjectData {
    x: i32,
    y: i32,
}

#[derive(Debug,Deserialize)]
pub struct PlanetData {
    x: i32,
    y: i32,
    ring: f64,
    strength: f64,
    kind: PlanetKind,
}

#[derive(Debug,Deserialize)]
pub struct LevelData {
    meteor: ObjectData,
    stars: Vec<ObjectData>,
    planets: Vec<PlanetData>,
}

impl LevelData {
    pub fn load(path: &'static str) -> Result<LevelData> {
        let f = File::open(path)?;
        Ok(serde_yaml::from_reader(&f)?)
    }
}

impl Level {
    pub fn new(data: LevelData, window_size: glm::UVec2, asset_manager: &AssetManager) -> Level {
        let planets = data.planets
            .iter()
            .map(|p| {
                let center = glm::ivec2(p.x, p.y);
                Planet::new(center, p.strength, p.ring, p.kind.clone(), asset_manager)
            })
            .collect::<Vec<_>>();

        let stars = data.stars
            .iter()
            .map(|s| Star::new(glm::ivec2(s.x, s.y), asset_manager))
            .collect::<Vec<_>>();

        let meteor_center = glm::ivec2(data.meteor.x, data.meteor.y);
        let asset = asset_manager.get_asset(TextureAsset::Meteor, meteor_center);
        let unlaunched_meteor = UnlaunchedMeteor::new(asset.clone());
        let state = MeteorState::UNLAUNCHED(unlaunched_meteor);
        let explosion = asset_manager.get_animation(AnimationAsset::ExplosionLarge, meteor_center);

        Level {
            planets: planets,
            stars: stars,
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

        self.update_animations();
    }

    pub fn drawables(&self) -> Vec<Drawable> {
        let mut drawables: Vec<Drawable> = vec![];
        for planet in &self.planets {
            drawables.append(&mut planet.drawables());
        }

        for star in &self.stars {
            drawables.append(&mut star.drawables());
        }

        match self.state {
            MeteorState::LAUNCHED(ref m) => drawables.append(&mut m.drawables()),
            MeteorState::UNLAUNCHED(ref m) => drawables.append(&mut m.drawables()),
        }

        for animation in &self.animations {
            drawables.push(Drawable::Asset(&animation.asset));
        }

        drawables
    }

    fn update_animations(&mut self) {
        for star in &mut self.stars {
            star.update();
        }

        for animation in &mut self.animations {
            animation.update();
        }

        self.animations.retain(Animation::is_active);
    }

    fn launch(&self, target: glm::IVec2) -> LaunchedMeteor {
        const FACTOR: f64 = 50.;
        let asset = self.asset.clone();
        let offset = target - glm::to_ivec2(asset.center());
        let velocity = glm::to_dvec2(offset) / FACTOR;
        LaunchedMeteor::new(asset, self.explosion.clone(), self.max_coords, velocity)
    }
}
