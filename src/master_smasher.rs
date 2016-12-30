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
use animation::Animation;
use sprite_strip::SpriteStrip;
use explosion::Explosion;

pub struct MasterSmasher<E: MohoEngine> {
    meteor: Meteor<E::Renderer>,
    planets: Vec<Planet<E::Renderer>>,
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
        let blue_planet_path = "resources/blue_planet.png";
        let red_planet_path = "resources/red_planet.png";

        let (window_width, window_height) = renderer.output_size()?;
        let background = renderer.load_texture(background_path)?;
        let blue_planet = Planet::new(renderer.load_texture(blue_planet_path)?,
                                      glm::ivec2(400, 300));
        let red_planet = Planet::new(renderer.load_texture(red_planet_path)?,
                                     glm::ivec2(700, 500));
        let meteor = Meteor::new(renderer.load_texture(meteor_path)?,
                                 glm::ivec2(50, 50),
                                 glm::uvec2(window_width, window_height));

        Ok(MasterSmasher {
            meteor: meteor,
            planets: vec![blue_planet, red_planet],
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
        if !self.input_manager.update() || self.input_manager.is_key_down(Keycode::Escape) {
            return false;
        }

        if self.meteor.is_launched() {
            if self.input_manager.did_press_key(Keycode::R) {
                self.meteor.restart_at(glm::ivec2(50, 50));
            } else {
                self.update_meteor();
            }
        } else {
            if self.input_manager.did_click_mouse(MouseButton::Left) {
                self.meteor.launch(self.input_manager.mouse_coords());
            } else {
                self.update_launch_vector();
            }
        }

        let explosion_ended = match self.explosion {
            Some(ref mut expl) => !expl.update(),
            None => false,
        };

        if explosion_ended {
            self.explosion = None
        };

        true
    }

    fn draw(&mut self) -> Result<(), Box<Error>> {
        self.renderer.clear();
        self.renderer.draw(&*self.background.texture, None, None, None)?;
        self.meteor.draw(&mut self.renderer)?;
        for planet in &self.planets {
            planet.draw(&mut self.renderer)?;
        }
        if !self.meteor.is_launched() {
            self.renderer.fill_rects(&self.rects)?;
        }
        if let Some(ref expl) = self.explosion {
            expl.draw(&mut self.renderer)?
        }
        self.renderer.present();
        Ok(())
    }

    fn update_meteor(&mut self) {
        self.meteor.update();

        if self.meteor.collides_with(&self.planets) {
            let explosion_path = "resources/explosion_large.png";
            let explosion_texture = self.renderer.load_texture(explosion_path).unwrap();
            let explosion_sprite = SpriteStrip::new(explosion_texture, 8, None);
            let animation = Animation::new(explosion_sprite, 8, false, 80);
            let center = glm::ivec2(self.meteor.center().x as i32, self.meteor.center().y as i32);
            self.explosion = Some(Explosion::new(animation, center));
            self.meteor.restart_at(glm::ivec2(50, 50));
        }
    }

    fn update_launch_vector(&mut self) {
        let mouse_coords = self.input_manager.mouse_coords();
        let mouse_coords = glm::dvec2(mouse_coords.x as f64, mouse_coords.y as f64);
        let distance = mouse_coords - self.meteor.center();
        let offset = self.meteor.radius() + 10.;
        let offset_vector = normalize_to(distance, offset);
        let anchor_point = self.meteor.center() + offset_vector;
        let step = (mouse_coords - anchor_point) / (self.rects.len() as f64);

        for (i, rect) in self.rects.iter_mut().enumerate() {
            let point = anchor_point + (step * i as f64);
            rect.center_on((point.x as i32, point.y as i32));
        }
    }
}
