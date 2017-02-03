use master_smasher::drawable::{Animation, AnimationAsset, Asset, Drawable, AssetManager,
                               TextureAsset};
use super::unlaunched_meteor::UnlaunchedMeteor;
use super::launched_meteor::LaunchedMeteor;
use super::planet::Planet;

use glm;
use moho::input_manager::{EventPump, InputManager};
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;

pub enum MeteorState {
    UNLAUNCHED(UnlaunchedMeteor),
    LAUNCHED(LaunchedMeteor),
    EXPLODED(Animation),
}

pub struct Player {
    pub state: MeteorState,
    max_coords: glm::UVec2,
    explosion: Animation,
    asset: Asset,
}

impl Player {
    pub fn new(asset_manager: &AssetManager, center: glm::IVec2, max_coords: glm::UVec2) -> Self {
        let asset = asset_manager.get_asset(TextureAsset::Meteor, center);
        let explosion = asset_manager.get_animation(AnimationAsset::ExplosionLarge, center);
        let meteor = UnlaunchedMeteor::new(asset.clone());
        let state = MeteorState::UNLAUNCHED(meteor);
        Player {
            max_coords: max_coords,
            asset: asset,
            state: state,
            explosion: explosion,
        }
    }

    pub fn update<E: EventPump>(&mut self, planets: &[Planet], input_manager: &InputManager<E>) {
        let target = input_manager.mouse_coords();

        let next_state = match self.state {
            MeteorState::UNLAUNCHED(_) if input_manager.did_click_mouse(MouseButton::Left) => {
                Some(MeteorState::LAUNCHED(self.launch(target)))
            }
            MeteorState::LAUNCHED(ref m) if input_manager.did_press_key(Keycode::R) => {
                let mut explosion = m.explode();
                explosion.update();
                Some(MeteorState::EXPLODED(explosion))
            }
            MeteorState::UNLAUNCHED(ref mut m) => {
                m.update(target);
                None
            }
            MeteorState::LAUNCHED(ref mut m) => {
                m.update(planets);
                if planets.iter().any(|p| m.collides(p)) {
                    let mut explosion = m.explode();
                    explosion.update();
                    Some(MeteorState::EXPLODED(explosion))
                } else {
                    None
                }
            }
            MeteorState::EXPLODED(ref mut a) => {
                a.update();
                if a.is_active() {
                    None
                } else {
                    let mut meteor = UnlaunchedMeteor::new(self.asset.clone());
                    meteor.update(target);
                    Some(MeteorState::UNLAUNCHED(meteor))
                }
            }
        };

        if let Some(s) = next_state {
            self.state = s;
        }
    }

    pub fn drawables(&self) -> Vec<Drawable> {
        match self.state {
            MeteorState::LAUNCHED(ref m) => m.drawables(),
            MeteorState::UNLAUNCHED(ref m) => m.drawables(),
            MeteorState::EXPLODED(ref a) => vec![Drawable::Asset(&a.asset)],
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
