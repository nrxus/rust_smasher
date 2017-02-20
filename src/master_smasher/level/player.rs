use master_smasher::drawable::{Animation, Drawable};
use super::unlaunched_meteor::UnlaunchedMeteor;
use super::launched_meteor::LaunchedMeteor;
use super::planet::Planet;
use super::player_assets::PlayerAssets;

use glm;
use moho::input_manager::{EventPump, InputManager};
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;

use std::time::Duration;

pub enum MeteorState {
    UNLAUNCHED(UnlaunchedMeteor),
    LAUNCHED(LaunchedMeteor),
    EXPLODED(Animation),
}

pub struct Player {
    pub state: MeteorState,
    max_coords: glm::UVec2,
    assets: PlayerAssets,
    initial_center: glm::IVec2,
}

impl Player {
    pub fn new(assets: PlayerAssets, center: glm::IVec2, max_coords: glm::UVec2) -> Self {
        let asset = assets.meteor(center);
        let meteor = UnlaunchedMeteor::new(asset.clone());
        let state = MeteorState::UNLAUNCHED(meteor);
        Player {
            state: state,
            max_coords: max_coords,
            assets: assets,
            initial_center: center,
        }
    }

    pub fn update<E: EventPump>(&mut self, planets: &[Planet], input_manager: &InputManager<E>) {
        let target = input_manager.mouse_coords();

        let next_state = match self.state {
            MeteorState::UNLAUNCHED(ref m) if input_manager.did_click_mouse(MouseButton::Left) => {
                Some(m.next(self.max_coords))
            }
            MeteorState::LAUNCHED(ref m) if input_manager.did_press_key(Keycode::R) => {
                let explosion = self.assets.explosion(glm::to_ivec2(m.center()));
                Some(MeteorState::EXPLODED(explosion))
            }
            MeteorState::UNLAUNCHED(ref mut m) => {
                m.update(target);
                None
            }
            MeteorState::LAUNCHED(ref mut m) => {
                m.update(planets);
                if planets.iter().any(|p| m.collides(p)) {
                    Some(MeteorState::EXPLODED(self.assets.explosion(glm::to_ivec2(m.center()))))
                } else {
                    None
                }
            }
            MeteorState::EXPLODED(ref mut a) => {
                if a.is_active() {
                    None
                } else {
                    let mut meteor = UnlaunchedMeteor::new(self.assets.meteor(self.initial_center));
                    meteor.update(target);
                    Some(MeteorState::UNLAUNCHED(meteor))
                }
            }
        };

        if let Some(s) = next_state {
            self.state = s;
        }
    }

    pub fn animate(&mut self, delta: Duration) {
        if let MeteorState::EXPLODED(ref mut a) = self.state {
            a.update(delta);
        }
    }

    pub fn drawables(&self) -> Vec<Drawable> {
        match self.state {
            MeteorState::LAUNCHED(ref m) => m.drawables(),
            MeteorState::UNLAUNCHED(ref m) => m.drawables(),
            MeteorState::EXPLODED(ref a) => vec![Drawable::Asset(&a.asset)],
        }
    }
}
