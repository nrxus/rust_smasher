extern crate moho;
extern crate sdl2_image;
extern crate sdl2;
extern crate glm;

use self::moho::input_manager::*;

use self::sdl2::keyboard::Keycode;
use self::sdl2::mouse::Mouse;
use self::sdl2::render::Renderer;
use self::sdl2_image::LoadTexture;

use std::error::Error;
use std::path::Path;

use meteor::Meteor;
use drawable::Drawable;

pub struct MasterSmasher<'a> {
    meteor: Meteor,
    background: Drawable,
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

        let background = Drawable::new(try!(renderer.load_texture(background_path)));
        let meteor = Meteor::new(try!(renderer.load_texture(meteor_path)),
                                 glm::uvec2(WINDOW_WIDTH, WINDOW_HEIGHT));

        Ok(MasterSmasher {
            meteor: meteor,
            background: background,
            input_manager: input_manager,
            renderer: renderer,
        })
    }

    pub fn run(&mut self) -> Result<(), Box<Error>> {
        'running: loop {
            self.input_manager.update();

            if self.input_manager.is_key_down(Keycode::Escape) {
                break;
            }

            if self.input_manager.is_mouse_down(Mouse::Left) {
                if !self.meteor.is_launched() {
                    self.meteor.launch(self.input_manager.mouse_coords());
                }
            }

            self.meteor.update();
            try!(self.draw());
        }

        Ok(())
    }

    fn draw(&mut self) -> Result<(), Box<Error>> {
        self.renderer.clear();
        try!(self.background.draw(&mut self.renderer));
        try!(self.meteor.draw(&mut self.renderer));
        self.renderer.present();
        Ok(())
    }
}
