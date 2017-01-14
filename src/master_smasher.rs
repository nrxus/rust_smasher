use std::error::Error;

use glm;
use glm::ext::normalize_to;
use sdl2::keyboard::Keycode;
use sdl2::rect;
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
    rects: [rect::Rect; 10],
}

impl<E: MohoEngine> MasterSmasher<E> {
    pub fn new(mut renderer: ResourceManager<E::Renderer>,
               input_manager: InputManager<E::EventPump>)
               -> Result<Self, Box<Error>> {
        let background = renderer.load_texture("resources/background_game.png")?;

        let blue_planet = Planet::new(glm::uvec2(840, 478),
                                      700.,
                                      215.,
                                      PlanetKind::BLUE,
                                      &mut renderer)?;

        let white_planet = Planet::new(glm::uvec2(346, 298),
                                       400.,
                                       175.,
                                       PlanetKind::WHITE,
                                       &mut renderer)?;

        let meteor = Meteor::new(glm::ivec2(130, 402), &mut renderer)?;

        Ok(MasterSmasher {
            meteor: meteor,
            planets: vec![white_planet, blue_planet],
            background: background,
            input_manager: input_manager,
            renderer: renderer,
            rects: [rect::Rect::new(0, 0, 5, 5); 10],
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
                if self.input_manager.did_click_mouse(MouseButton::Left) {
                    self.meteor.launch(self.input_manager.mouse_coords());
                } else {
                    self.update_launch_vector();
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
        if let MeteorState::UNLAUNCHED = self.meteor.state() {
            self.renderer.fill_rects(&self.rects)?;
        }
        self.renderer.present();
        Ok(())
    }

    fn update_launch_vector(&mut self) {
        let mouse_coords = self.input_manager.mouse_coords();
        let mouse_coords = glm::dvec2(mouse_coords.x as f64, mouse_coords.y as f64);
        let distance = mouse_coords - self.meteor.center();
        let offset = self.meteor.radius() + 10.;
        let offset_vector = normalize_to(distance, offset as f64);
        let anchor_point = self.meteor.center() + offset_vector;
        let step = (mouse_coords - anchor_point) / (self.rects.len() as f64);

        for (i, rect) in self.rects.iter_mut().enumerate() {
            let point = anchor_point + (step * i as f64);
            rect.center_on((point.x as i32, point.y as i32));
        }
    }
}
