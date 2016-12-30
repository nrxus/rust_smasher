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
use std::cmp;

use meteor::Meteor;
use planet::Planet;
use animation::Animation;
use explosion::Explosion;

pub struct MasterSmasher<E: MohoEngine> {
    meteor: Meteor<E::Renderer>,
    planets: Vec<Planet<E::Renderer>>,
    background: TextureData<<E::Renderer as Renderer>::Texture>,
    explosions: Vec<Explosion<E::Renderer>>,
    input_manager: InputManager<E::EventPump>,
    renderer: ResourceManager<E::Renderer>,
    rects: [rect::Rect; 10],
}

impl<E: MohoEngine> MasterSmasher<E> {
    pub fn new(renderer: ResourceManager<E::Renderer>,
               input_manager: InputManager<E::EventPump>)
               -> Result<Self, Box<Error>> {
        let meteor_texture = renderer.load_texture("resources/meteor.png")?;
        let blue_planet_texture = renderer.load_texture("resources/blue_planet.png")?;
        let red_planet_texture = renderer.load_texture("resources/red_planet.png")?;
        let blue_ring_texture = renderer.load_texture("resources/blue_ring.png")?;
        let red_ring_texture = renderer.load_texture("resources/red_ring.png")?;
        let background = renderer.load_texture("resources/background_game.png")?;

        let blue_planet = Planet::new(glm::uvec2(400, 300),
                                      300.,
                                      Self::texture_radius(&blue_planet_texture),
                                      blue_planet_texture.texture,
                                      blue_ring_texture.texture);

        let red_planet = Planet::new(glm::uvec2(700, 500),
                                     424.,
                                     Self::texture_radius(&red_planet_texture),
                                     red_planet_texture.texture,
                                     red_ring_texture.texture);

        let (window_width, window_height) = renderer.output_size()?;
        let meteor = Meteor::new(glm::uvec2(50, 50),
                                 Self::texture_radius(&meteor_texture),
                                 glm::uvec2(window_width, window_height),
                                 meteor_texture.texture);

        Ok(MasterSmasher {
            meteor: meteor,
            planets: vec![blue_planet, red_planet],
            background: background,
            explosions: vec![],
            input_manager: input_manager,
            renderer: renderer,
            rects: [rect::Rect::new(0, 0, 5, 5); 10],
        })
    }

    fn texture_radius(texture_data: &TextureData<<E::Renderer as Renderer>::Texture>) -> f64 {
        cmp::min(texture_data.width, texture_data.height) as f64 / 2.
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
        } else if self.input_manager.did_click_mouse(MouseButton::Left) {
            self.meteor.launch(self.input_manager.mouse_coords());
        } else {
            self.update_launch_vector();
        }

        self.retain_mut(|ref mut e| e.update());

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
        for explosion in &self.explosions {
            explosion.draw(&mut self.renderer)?
        }
        self.renderer.present();
        Ok(())
    }

    fn update_meteor(&mut self) {
        self.meteor.update();

        if self.meteor.collides_with(&self.planets) {
            let explosion_path = "resources/explosion_large.png";
            let explosion_texture = self.renderer.load_texture(explosion_path).unwrap();
            let dims = glm::uvec2(explosion_texture.width / 8, explosion_texture.height);
            let animation = Animation::new(explosion_texture, 8, false, 80);
            let center = glm::ivec2(self.meteor.center().x as i32, self.meteor.center().y as i32);
            self.explosions.push(Explosion::new(animation, center, dims));
            self.meteor.restart_at(glm::ivec2(50, 50));
        }
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

    pub fn retain_mut<F>(&mut self, mut f: F)
        where F: FnMut(&mut Explosion<E::Renderer>) -> bool
    {
        let vector = &mut self.explosions;
        let len = vector.len();
        let mut del = 0;
        {
            let v = &mut **vector;

            for i in 0..len {
                if !f(&mut v[i]) {
                    del += 1;
                } else if del > 0 {
                    v.swap(i - del, i);
                }
            }
        }
        if del > 0 {
            vector.truncate(len - del);
        }
    }
}
