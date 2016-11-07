extern crate moho;
extern crate sdl2_image;
extern crate sdl2;
extern crate glm;

use self::moho::input_manager::*;

use self::sdl2::keyboard::Keycode;
use self::sdl2::mouse::Mouse;
use self::sdl2::render::{Renderer, Texture};
use self::sdl2_image::LoadTexture;

use std::error::Error;
use std::path::Path;

use meteor::Meteor;
use drawable::Drawable;
use shape::Intersect;

pub struct MasterSmasher<'a> {
    meteor: Meteor,
    planet: Drawable,
    background: Texture,
    input_manager: InputManager<SdlEventStreamGenerator>,
    renderer: Renderer<'a>,
}

impl<'a> MasterSmasher<'a> {
    pub fn new() -> Result<Self, Box<Error>> {
        const WINDOW_HEIGHT: u32 = 600;
        const WINDOW_WIDTH: u32 = 800;

        let (renderer, input_manager) =
            try!(moho::init("Master Smasher", WINDOW_WIDTH, WINDOW_HEIGHT));
        let background_path = Path::new("resources/background_game.png");
        let meteor_path = Path::new("resources/meteor.png");
        let planet_path = Path::new("resources/blue_planet.png");

        let background = try!(renderer.load_texture(background_path));
        let planet = Drawable::new(try!(renderer.load_texture(planet_path)),
                                   glm::ivec2(400, 300));
        let meteor = Meteor::new(try!(renderer.load_texture(meteor_path)),
                                 glm::ivec2(50, 50),
                                 glm::uvec2(WINDOW_WIDTH, WINDOW_HEIGHT));

        Ok(MasterSmasher {
            meteor: meteor,
            planet: planet,
            background: background,
            input_manager: input_manager,
            renderer: renderer,
        })
    }

    pub fn run(&mut self) -> Result<(), Box<Error>> {
        loop {
            if !self.update() {
                break;
            }
            try!(self.draw());
        }

        Ok(())
    }

    fn update(&mut self) -> bool {
        if !self.input_manager.update() || self.input_manager.is_key_down(Keycode::Escape) {
            return false;
        }

        if self.input_manager.did_click_mouse(Mouse::Left) {
            if !self.meteor.is_launched() {
                self.meteor.launch(self.input_manager.mouse_coords());
            }
        }

        if self.input_manager.did_press_key(Keycode::R) {
            self.meteor.restart_at(glm::ivec2(50, 50));
        }

        self.meteor.update();
        if self.meteor.collision_body().intersects(&self.planet.collision_body()) {
            self.meteor.restart_at(glm::ivec2(50, 50));
        }

        true
    }

    fn draw(&mut self) -> Result<(), Box<Error>> {
        self.renderer.clear();
        try!(self.renderer.copy(&self.background, None, None));
        try!(self.meteor.draw(&mut self.renderer));
        try!(self.planet.draw(&mut self.renderer));
        self.renderer.present();
        Ok(())
    }
}
