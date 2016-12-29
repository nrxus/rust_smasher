extern crate moho;
extern crate sdl2;
extern crate glm;

use self::moho::input_manager::*;
use self::moho::resource_manager::*;
use self::moho::MohoEngine;

use self::sdl2::keyboard::Keycode;
use self::sdl2::rect;
use self::sdl2::mouse::MouseButton;

use self::glm::ext::normalize_to;

use std::error::Error;

use meteor::Meteor;
use planet::Planet;
use shape::Intersect;
use animation::Animation;
use sprite_strip::SpriteStrip;
use explosion::Explosion;
use shape::Shape;

pub struct MasterSmasher<E: MohoEngine> {
    meteor: Meteor<E::Renderer>,
    planet: Planet<E::Renderer>,
    background: TextureData<<E::Renderer as Renderer>::Texture>,
    explosion: Option<Explosion<E::Renderer>>,
    input_manager: InputManager<E::EventPump>,
    renderer: ResourceManager<E::Renderer>,
    rects: [rect::Rect; 10],
}

impl<E: MohoEngine> MasterSmasher<E> {
    pub fn new(renderer: ResourceManager<E::Renderer>,
               input_manager: InputManager<E::EventPump>)
               -> Result<Self, Box<Error>> {
        let background_path = "resources/background_game.png";
        let meteor_path = "resources/meteor.png";
        let planet_path = "resources/blue_planet.png";

        let (window_width, window_height) = renderer.output_size()?;
        let background = renderer.load_texture(background_path)?;
        let planet = Planet::new(renderer.load_texture(planet_path)?, glm::ivec2(400, 300));
        let meteor = Meteor::new(renderer.load_texture(meteor_path)?,
                                 glm::ivec2(50, 50),
                                 glm::uvec2(window_width, window_height));

        Ok(MasterSmasher {
            meteor: meteor,
            planet: planet,
            background: background,
            explosion: None,
            input_manager: input_manager,
            renderer: renderer,
            rects: [rect::Rect::new(0, 0, 5, 5); 10],
        })
    }

    pub fn run(&mut self) -> Result<(), Box<Error>> {
        while self.update() {
            self.draw()?;
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

        if !self.meteor.is_launched() {
            let mouse_coords = self.input_manager.mouse_coords();
            let mouse_coords = glm::dvec2(mouse_coords.x as f64, mouse_coords.y as f64);
            let meteor = self.meteor.collision_body();
            let distance = mouse_coords - meteor.center;
            let offset = meteor.radius + 10.;
            let offset_vector = normalize_to(distance, offset);
            let anchor_point = meteor.center + offset_vector;
            let step = (mouse_coords - anchor_point) / (self.rects.len() as f64);

            for (i, rect) in self.rects.iter_mut().enumerate() {
                let point = anchor_point + (step * i as f64);
                rect.center_on((point.x as i32, point.y as i32));
            }
        }

        if self.input_manager.did_click_mouse(MouseButton::Left) && !self.meteor.is_launched() {
            self.meteor.launch(self.input_manager.mouse_coords());
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
        self.renderer.draw(&*self.background.texture, None, None, None)?;
        self.meteor.draw(&mut self.renderer)?;
        self.planet.draw(&mut self.renderer)?;
        if !self.meteor.is_launched() {
            self.renderer.fill_rects(&self.rects)?;
        }
        if let Some(ref expl) = self.explosion {
            expl.draw(&mut self.renderer)?
        }
        self.renderer.present();
        Ok(())
    }
}
