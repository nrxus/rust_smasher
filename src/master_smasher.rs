use std::error::Error;

use glm;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use moho::input_manager::*;
use moho::resource_manager::*;
use moho::MohoEngine;

use meteor::{Meteor, MeteorState};
use planet::{Planet, PlanetKind};

pub struct MasterSmasher<E: MohoEngine> {
    meteor: Meteor<E::Renderer>,
    planets: Vec<Planet<E::Renderer>>,
    background: TextureData<<E::Renderer as Renderer>::Texture>,
    input_manager: InputManager<E::EventPump>,
    renderer: ResourceManager<E::Renderer>,
}

impl<E: MohoEngine> MasterSmasher<E> {
    pub fn new(mut renderer: ResourceManager<E::Renderer>,
               input_manager: InputManager<E::EventPump>)
               -> Result<Self, Box<Error>> {
        let background = renderer.load_texture("resources/background_game.png")?;
        let blue_center = glm::ivec2(840, 478);
        let white_center = glm::ivec2(346, 298);
        let meteor_center = glm::ivec2(130, 402);
        let blue_planet = Planet::new(blue_center, 700., 215., PlanetKind::BLUE, &mut renderer)?;
        let white_planet = Planet::new(white_center, 400., 175., PlanetKind::WHITE, &mut renderer)?;
        let meteor = Meteor::new(meteor_center, &mut renderer)?;

        Ok(MasterSmasher {
            meteor: meteor,
            planets: vec![white_planet, blue_planet],
            background: background,
            input_manager: input_manager,
            renderer: renderer,
        })
    }

    pub fn run(&mut self) -> Result<(), Box<Error>> {
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

        match self.meteor.state() {
            MeteorState::UNLAUNCHED => {
                self.meteor.update_target(self.input_manager.mouse_coords());
                if self.input_manager.did_click_mouse(MouseButton::Left) {
                    self.meteor.launch();
                }
            }
            MeteorState::LAUNCHED => {
                if self.input_manager.did_press_key(Keycode::R) {
                    self.meteor.restart();
                }
            }
            MeteorState::EXPLODED => {}
        }

        self.meteor.update(&self.planets)
    }

    fn game_quit(&self) -> bool {
        self.input_manager.game_quit() || self.input_manager.is_key_down(Keycode::Escape)
    }

    fn draw(&mut self) -> Result<(), Box<Error>> {
        self.renderer.clear();
        self.renderer.draw(&*self.background.texture, None, None, None)?;
        for planet in &self.planets {
            planet.draw(&mut self.renderer)?;
        }
        self.meteor.draw(&mut self.renderer)?;
        self.renderer.present();
        Ok(())
    }
}
