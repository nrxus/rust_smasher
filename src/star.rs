use glm;
use moho::errors::*;
use moho::resource_manager::{Renderer, ResourceManager, TextureData};

use animation::Animation;
use circle::Circle;
use collidable::Collidable;
use shape::Intersect;

use std::cmp;
use std::time::Duration;

pub enum State {
    INACTIVE,
    ACTIVE,
    EXPLODED,
}

pub struct Star<R: Renderer> {
    state: State,
    body: Circle,
    texture: TextureData<R>,
    explosion_texture: TextureData<R>,
    animation: Animation,
    explosion_animation: Animation,
}

impl<R: Renderer> Star<R> {
    pub fn new(center: glm::IVec2, resource_manager: &ResourceManager<R>) -> Result<Self> {
        let mut texture = resource_manager.load_texture("resources/star.png")?;
        let mut explosion_texture = resource_manager.load_texture("resources/explosion_small.png")?;
        let star_duration = Duration::from_millis(150);
        let explosion_duration = Duration::from_millis(100);
        let animation = Animation::new(2, star_duration, texture.dims, true);
        let explosion_animation =
            Animation::new(10, explosion_duration, explosion_texture.dims, false);
        let radius = cmp::max(texture.dims.x, texture.dims.y) as f64 / 2.;
        texture.dims.x /= 2;
        explosion_texture.dims.x /= 10;
        let body = Circle {
            center: glm::to_dvec2(center),
            radius: radius,
        };

        let star = Star {
            state: State::ACTIVE,
            body: body,
            texture: texture,
            explosion_texture: explosion_texture,
            animation: animation,
            explosion_animation: explosion_animation,
        };

        Ok(star)
    }

    pub fn explode(&mut self) {
        self.state = State::EXPLODED;
    }

    pub fn update(&mut self) {
        match self.state {
            State::INACTIVE => {}
            State::ACTIVE => self.animation.update(),
            State::EXPLODED => {
                self.explosion_animation.update();
                if !self.explosion_animation.is_active() {
                    self.state = State::INACTIVE;
                }
            }
        }
    }

    pub fn draw(&self, renderer: &mut ResourceManager<R>) -> Result<()> {
        match self.state {
            State::INACTIVE => Ok(()),
            State::ACTIVE => {
                let src_rect = self.animation.src_rect();
                self.draw_on_center(&self.texture, src_rect, renderer)
            }
            State::EXPLODED => {
                let src_rect = self.explosion_animation.src_rect();
                self.draw_on_center(&self.explosion_texture, src_rect, renderer)
            }
        }
    }

    fn draw_on_center(&self,
                      texture: &TextureData<R>,
                      src_rect: glm::IVec4,
                      renderer: &mut ResourceManager<R>)
                      -> Result<()> {
        let center = glm::to_ivec2(self.body.center);
        renderer.draw_from_center(texture, center, Some(src_rect), None)
    }
}

impl<R: Renderer, I: Intersect<Circle>> Collidable<Circle, I> for Star<R> {
    fn collides(&self, collision: &I) -> bool {
        collision.intersects(&self.body)
    }
}
