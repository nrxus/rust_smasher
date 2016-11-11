extern crate moho;
extern crate sdl2_image;
extern crate sdl2;
extern crate glm;

use self::moho::input_manager::*;
use self::moho::resource_manager::*;

use self::sdl2::keyboard::Keycode;
use self::sdl2::mouse::Mouse;
use self::sdl2::render::Renderer as SdlRenderer;
use self::sdl2::EventPump as SdlEventPump;
use self::sdl2::render::Texture;

use std::error::Error;
use std::rc::Rc;

use meteor::Meteor;
use planet::Planet;
use shape::Intersect;
use animation::Animation;
use sprite_strip::SpriteStrip;
use explosion::Explosion;
use shape::Shape;

pub struct MasterSmasher<'a> {
    meteor: Meteor,
    planet: Planet,
    background: Rc<Texture>,
    explosion: Option<Explosion>,
    input_manager: InputManager<SdlEventPump>,
    renderer: ResourceManager<'a, SdlRenderer<'a>>,
}

impl<'a> MasterSmasher<'a> {
    pub fn new() -> Result<Self, Box<Error>> {
        const WINDOW_HEIGHT: u32 = 600;
        const WINDOW_WIDTH: u32 = 800;

        let (renderer, input_manager) =
            try!(moho::init("Master Smasher", WINDOW_WIDTH, WINDOW_HEIGHT));
        let background_path = "resources/background_game.png";
        let meteor_path = "resources/meteor.png";
        let planet_path = "resources/blue_planet.png";

        let background = try!(renderer.load_texture(background_path));
        let planet = Planet::new(try!(renderer.load_texture(planet_path)),
                                 glm::ivec2(400, 300));
        let meteor = Meteor::new(try!(renderer.load_texture(meteor_path)),
                                 glm::ivec2(50, 50),
                                 glm::uvec2(WINDOW_WIDTH, WINDOW_HEIGHT));

        Ok(MasterSmasher {
            meteor: meteor,
            planet: planet,
            background: background,
            explosion: None,
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
        let explosion_ended = match self.explosion {
            Some(ref mut expl) => !expl.update(),
            None => false,
        };

        if explosion_ended {
            self.explosion = None
        };

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
        let meteor_body = self.meteor.collision_body();

        if meteor_body.intersects(&self.planet.collision_body()) {
            let explosion_path = "resources/explosion_large.png";
            let explosion_texture = self.renderer.load_texture(explosion_path).unwrap();
            let explosion_sprite = SpriteStrip::new(explosion_texture, 8, None);
            let animation = Animation::new(explosion_sprite, 8, false, 80);
            let center = glm::ivec2(meteor_body.get_center().x as i32,
                                    meteor_body.get_center().y as i32);
            self.explosion = Some(Explosion::new(animation, center));
            self.meteor.restart_at(glm::ivec2(50, 50));
        }

        true
    }

    fn draw(&mut self) -> Result<(), Box<Error>> {
        self.renderer.clear();
        try!(self.renderer.draw(self.background.clone(), None, None));
        try!(self.meteor.draw(&mut self.renderer));
        try!(self.planet.draw(&mut self.renderer));
        match self.explosion {
            Some(ref expl) => try!(expl.draw(&mut self.renderer)),
            None => {}
        }
        self.renderer.present();
        Ok(())
    }
}
