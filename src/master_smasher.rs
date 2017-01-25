use asset_manager::AssetManager;
use meteor::{Meteor, MeteorState};
use planet::{Planet, PlanetKind};
use star::Star;

use glm;
use moho::errors::*;
use moho::input_manager::*;
use moho::resource_manager::*;
use moho::MohoEngine;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;

pub struct MasterSmasher<E: MohoEngine> {
    meteor: Meteor,
    planets: Vec<Planet>,
    stars: Vec<Star>,
    background: Texture,
    input_manager: InputManager<E::EventPump>,
    renderer: ResourceManager<E::Renderer>,
    asset_manager: AssetManager,
}

impl<E: MohoEngine> MasterSmasher<E> {
    pub fn new(renderer: ResourceManager<E::Renderer>,
               input_manager: InputManager<E::EventPump>)
               -> Result<Self> {
        let asset_manager = AssetManager::new(&renderer)?;
        let background = renderer.load_texture("resources/background_game.png")?;
        let blue_center = glm::ivec2(840, 478);
        let white_center = glm::ivec2(346, 298);
        let meteor_center = glm::ivec2(130, 402);
        let star_center = glm::ivec2(500, 130);
        let blue_planet = Planet::new(blue_center, 700., 215., PlanetKind::BLUE, &asset_manager)?;
        let white_planet =
            Planet::new(white_center, 400., 175., PlanetKind::WHITE, &asset_manager)?;
        let meteor = Meteor::new(meteor_center, &renderer)?;
        let star = Star::new(star_center, &asset_manager);

        Ok(MasterSmasher {
            meteor: meteor,
            planets: vec![white_planet, blue_planet],
            stars: vec![star],
            background: background,
            input_manager: input_manager,
            renderer: renderer,
            asset_manager: asset_manager,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        while !self.game_quit() {
            self.update();
            self.draw()?;
        }
        Ok(())
    }

    fn update(&mut self) {
        self.input_manager.update();
        if self.game_quit() {
            return;
        }

        match *self.meteor.state() {
            MeteorState::UNLAUNCHED => {
                self.meteor.update_target(self.input_manager.mouse_coords());
                if self.input_manager.did_click_mouse(MouseButton::Left) {
                    self.meteor.launch();
                }
            }
            MeteorState::LAUNCHED => {
                if self.input_manager.did_press_key(Keycode::R) {
                    self.meteor.explode();
                }
            }
            MeteorState::EXPLODED => {}
        }

        self.meteor.update(&self.planets);
        if self.planets.iter().any(|p| self.meteor.collides(p)) {
            self.meteor.explode();
        }

        let meteor = &mut self.meteor;

        for star in self.stars.iter_mut().filter(|s| meteor.collides(*s)) {
            star.explode();
        }

        for star in &mut self.stars {
            star.update();
        }
    }

    fn game_quit(&self) -> bool {
        self.input_manager.game_quit() || self.input_manager.is_key_down(Keycode::Escape)
    }

    fn draw(&mut self) -> Result<()> {
        self.renderer.clear();
        self.renderer.draw(self.background.id, None, None, None)?;
        for star in &self.stars {
            star.draw(&mut self.renderer)?;
        }
        for planet in &self.planets {
            planet.draw(&mut self.renderer)?;
        }
        self.meteor.draw(&mut self.renderer)?;
        self.renderer.present();
        Ok(())
    }
}
