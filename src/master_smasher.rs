use std::error::Error;
use std::cmp;
use std::time::Duration;

use glm;
use glm::ext::normalize_to;
use sdl2::keyboard::Keycode;
use sdl2::rect;
use sdl2::mouse::MouseButton;
use moho::input_manager::*;
use moho::resource_manager::*;
use moho::MohoEngine;

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
    explosion_texture: TextureData<<E::Renderer as Renderer>::Texture>,
    rects: [rect::Rect; 10],
}

impl<E: MohoEngine> MasterSmasher<E> {
    pub fn new(renderer: ResourceManager<E::Renderer>,
               input_manager: InputManager<E::EventPump>)
               -> Result<Self, Box<Error>> {
        let meteor_texture = renderer.load_texture("resources/meteor.png")?;
        let white_planet_texture = renderer.load_texture("resources/white_planet.png")?;
        let blue_planet_texture = renderer.load_texture("resources/blue_planet.png")?;
        let white_ring_texture = renderer.load_texture("resources/white_ring.png")?;
        let blue_ring_texture = renderer.load_texture("resources/blue_ring.png")?;
        let background = renderer.load_texture("resources/background_game.png")?;
        let explosion_texture = renderer.load_texture("resources/explosion_large.png")?;

        let blue_planet = Planet::new(glm::uvec2(840, 478),
                                      700.,
                                      215.,
                                      Self::texture_radius(&blue_planet_texture),
                                      blue_planet_texture.texture,
                                      blue_ring_texture.texture);

        let white_planet = Planet::new(glm::uvec2(346, 298),
                                       400.,
                                       175.,
                                       Self::texture_radius(&white_planet_texture),
                                       white_planet_texture.texture,
                                       white_ring_texture.texture);

        let (window_width, window_height) = renderer.output_size()?;
        let meteor = Meteor::new(glm::uvec2(130, 402),
                                 Self::texture_radius(&meteor_texture),
                                 glm::uvec2(window_width, window_height),
                                 meteor_texture.texture);

        Ok(MasterSmasher {
            meteor: meteor,
            planets: vec![white_planet, blue_planet],
            background: background,
            explosions: vec![],
            input_manager: input_manager,
            renderer: renderer,
            explosion_texture: explosion_texture,
            rects: [rect::Rect::new(0, 0, 5, 5); 10],
        })
    }

    fn texture_radius(texture_data: &TextureData<<E::Renderer as Renderer>::Texture>) -> f64 {
        cmp::min(texture_data.dims.x, texture_data.dims.y) as f64 / 2.
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

        if self.meteor.is_launched() {
            if self.input_manager.did_press_key(Keycode::R) {
                self.meteor.restart_at(glm::ivec2(130, 402));
            } else {
                self.update_meteor();
            }
        } else if self.input_manager.did_click_mouse(MouseButton::Left) {
            self.meteor.launch(self.input_manager.mouse_coords());
        } else {
            self.update_launch_vector();
        }

        for mut explosion in &mut self.explosions {
            explosion.update();
        }

        self.explosions.retain(Explosion::is_active);
    }

    fn game_quit(&self) -> bool {
        self.input_manager.game_quit() || self.input_manager.is_key_down(Keycode::Escape)
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
        if !self.meteor.update(&self.planets) {
            self.explode_meteor();
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

    fn explode_meteor(&mut self) {
        let frame_duration = Duration::from_millis(80_u64);
        let texture_dims = self.explosion_texture.dims;
        let animation = Animation::new(8, frame_duration, texture_dims, false);
        let center = glm::ivec2(self.meteor.center().x as i32, self.meteor.center().y as i32);
        let dims = glm::uvec2(texture_dims.x / 8, texture_dims.y);
        let texture = self.explosion_texture.texture.clone();
        let explosion = Explosion::new(center, dims, animation, texture);
        self.explosions.push(explosion);
        self.meteor.restart_at(glm::ivec2(130, 402));
    }
}
