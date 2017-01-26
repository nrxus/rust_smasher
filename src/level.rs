use animation::Animation;
use asset_manager::AssetManager;
use meteor::{Meteor, MeteorState};
use planet::{Planet, PlanetKind};
use star::Star;

use glm;
use moho::errors::*;
use moho::input_manager::{EventPump, InputManager};
use moho::renderer::Renderer;
use moho::resource_manager::ResourceManager;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;

pub struct Level {
    initial: glm::IVec2,
    meteor: Meteor,
    planets: Vec<Planet>,
    stars: Vec<Star>,
    animations: Vec<Animation>,
}

impl Level {
    pub fn new(window_size: glm::UVec2, asset_manager: &AssetManager) -> Level {
        let blue_center = glm::ivec2(840, 478);
        let white_center = glm::ivec2(346, 298);
        let meteor_center = glm::ivec2(130, 402);
        let star_center = glm::ivec2(500, 130);
        let blue_planet = Planet::new(blue_center, 700., 215., PlanetKind::BLUE, &asset_manager);
        let white_planet = Planet::new(white_center, 400., 175., PlanetKind::WHITE, &asset_manager);
        let meteor = Meteor::new(meteor_center, window_size, &asset_manager);
        let star = Star::new(star_center, &asset_manager);

        Level {
            initial: meteor_center,
            meteor: meteor,
            planets: vec![blue_planet, white_planet],
            stars: vec![star],
            animations: Vec::new(),
        }
    }

    pub fn update<E: EventPump>(&mut self, input_manager: &InputManager<E>) {
        match *self.meteor.state() {
            MeteorState::UNLAUNCHED => {
                self.meteor.update_target(input_manager.mouse_coords());
                if input_manager.did_click_mouse(MouseButton::Left) {
                    self.meteor.launch();
                }
            }
            MeteorState::LAUNCHED => {
                if input_manager.did_press_key(Keycode::R) {
                    self.meteor.explode();
                }
            }
            MeteorState::EXPLODED => {}
        }

        self.meteor.update(&self.planets);
        if self.planets.iter().any(|p| self.meteor.collides(p)) {
            self.meteor.explode();
        }

        let collidable_indices = self.stars
            .iter()
            .enumerate()
            .filter(|&(_, s)| self.meteor.collides(s))
            .map(|(i, _)| i)
            .collect::<Vec<_>>();

        for i in collidable_indices {
            let star = self.stars.swap_remove(i);
            self.animations.push(star.explode());
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
            animation.draw(None, renderer)?;
        }
        self.meteor.draw(renderer)
    }
}
