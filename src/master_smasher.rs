use glm;
use moho::errors::*;
use moho::input_manager::*;
use moho::resource_manager::*;
use moho::MohoEngine;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;

use meteor::{Meteor, MeteorState};
use planet::{Planet, PlanetKind};
use self::star::Star;

mod star {
    use glm;
    use moho::errors::*;
    use moho::resource_manager::{Renderer, ResourceManager, TextureData};

    use animation::Animation;
    use circle::Circle;
    use shape::Intersect;

    use std::cmp;
    use std::time::Duration;
    use std::rc::Rc;

    pub struct Star<R: Renderer> {
        drawable: Drawable<R>,
        object: Object,
    }

    impl<R: Renderer> Star<R> {
        pub fn new(center: glm::IVec2, resource_manager: &ResourceManager<R>) -> Result<Self> {
            let drawable = Drawable::new(center, resource_manager)?;
            let dims = drawable.dims();
            let radius = cmp::max(dims.x, dims.y) as f64 / 2.;
            let object = Object::new(glm::to_dvec2(center), radius);
            let star = Star {
                drawable: drawable,
                object: object,
            };

            Ok(star)
        }

        pub fn update(&mut self) {
            self.drawable.animate();
        }

        pub fn draw(&self, renderer: &mut ResourceManager<R>) -> Result<()> {
            self.drawable.draw(renderer)
        }
    }

    struct Drawable<R: Renderer> {
        star: Rc<R::Texture>,
        explosion: Rc<R::Texture>,
        animation: Animation,
        explosion_animation: Animation,
        center: glm::IVec2,
        star_dims: glm::UVec2,
    }

    impl<R: Renderer> Drawable<R> {
        fn new(center: glm::IVec2, resource_manager: &ResourceManager<R>) -> Result<Self> {
            let star = resource_manager.load_texture("resources/star.png")?;
            let explosion = resource_manager.load_texture("resources/explosion_small.png")?;
            let star_duration = Duration::from_millis(150);
            let explosion_duration = Duration::from_millis(100);
            let star_dims = glm::uvec2(star.dims.x / 2, star.dims.y);
            let explosion_dims = glm::uvec2(explosion.dims.x / 10, explosion.dims.y);
            let animation = Animation::new(2, star_duration, star.dims, true);
            let explosion_animation = Animation::new(10, explosion_duration, explosion_dims, false);
            let drawable = Drawable {
                star: star.texture,
                explosion: explosion.texture,
                animation: animation,
                explosion_animation: explosion_animation,
                center: center,
                star_dims: star_dims,
            };

            Ok(drawable)
        }

        fn dims(&self) -> glm::UVec2 {
            self.star_dims
        }

        fn animate(&mut self) {
            self.animation.update();
        }

        fn animate_explosion(&mut self) {
            self.explosion_animation.update();
        }

        fn draw(&self, renderer: &mut ResourceManager<R>) -> Result<()> {
            let src = Some(self.animation.src_rect());
            renderer.draw_from_center(&*self.star, src, self.center, self.star_dims, None)
        }

        fn is_exploding(&self) -> bool {
            self.explosion_animation.is_active()
        }
    }

    struct Object {
        body: Circle,
    }

    impl Object {
        fn new(center: glm::DVec2, radius: f64) -> Self {
            let body = Circle {
                center: center,
                radius: radius,
            };

            Object { body: body }
        }

        fn collides<S: Intersect<Circle>>(&self, body: S) -> bool {
            body.intersects(&self.body)
        }
    }
}

pub struct MasterSmasher<E: MohoEngine> {
    meteor: Meteor<E::Renderer>,
    planets: Vec<Planet<E::Renderer>>,
    stars: Vec<Star<E::Renderer>>,
    background: TextureData<<E::Renderer as Renderer>::Texture>,
    input_manager: InputManager<E::EventPump>,
    renderer: ResourceManager<E::Renderer>,
}

impl<E: MohoEngine> MasterSmasher<E> {
    pub fn new(mut renderer: ResourceManager<E::Renderer>,
               input_manager: InputManager<E::EventPump>)
               -> Result<Self> {
        let background = renderer.load_texture("resources/background_game.png")?;
        let blue_center = glm::ivec2(840, 478);
        let white_center = glm::ivec2(346, 298);
        let meteor_center = glm::ivec2(130, 402);
        let star_center = glm::ivec2(500, 130);
        let blue_planet = Planet::new(blue_center, 700., 215., PlanetKind::BLUE, &mut renderer)?;
        let white_planet = Planet::new(white_center, 400., 175., PlanetKind::WHITE, &mut renderer)?;
        let meteor = Meteor::new(meteor_center, &mut renderer)?;
        let star = Star::new(star_center, &mut renderer)?;

        Ok(MasterSmasher {
            meteor: meteor,
            planets: vec![white_planet, blue_planet],
            stars: vec![star],
            background: background,
            input_manager: input_manager,
            renderer: renderer,
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
        for star in &mut self.stars {
            star.update();
        }
    }

    fn game_quit(&self) -> bool {
        self.input_manager.game_quit() || self.input_manager.is_key_down(Keycode::Escape)
    }

    fn draw(&mut self) -> Result<()> {
        self.renderer.clear();
        self.renderer.draw(&*self.background.texture, None, None, None)?;
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
