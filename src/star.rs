use animation::Animation;
use asset_manager::{AssetManager, AnimationAsset};
use collidable::Collidable;
use rectangle::Rectangle;
use shape::Intersect;

use glm;
use moho::errors::*;
use moho::renderer::Renderer;
use moho::resource_manager::ResourceManager;

pub enum State {
    INACTIVE,
    ACTIVE,
    EXPLODED,
}

pub struct Star {
    state: State,
    body: Rectangle,
    animation: Animation,
    explosion: Animation,
}

impl Star {
    pub fn new(center: glm::IVec2, asset_manager: &AssetManager) -> Self {
        let mut animation = asset_manager.get_animation(AnimationAsset::Star);
        let mut explosion = asset_manager.get_animation(AnimationAsset::ExplosionSmall);
        animation.set_center(center);
        explosion.set_center(center);

        let body = Rectangle {
            center: glm::to_dvec2(center),
            dims: glm::dvec2(animation.dst_rect().z as f64, animation.dst_rect().w as f64),
        };

        Star {
            state: State::ACTIVE,
            body: body,
            explosion: explosion,
            animation: animation,
        }
    }

    pub fn explode(&mut self) {
        self.state = State::EXPLODED;
    }

    pub fn update(&mut self) {
        match self.state {
            State::INACTIVE => {}
            State::ACTIVE => self.animation.update(),
            State::EXPLODED => {
                self.explosion.update();
                if !self.explosion.is_active() {
                    self.state = State::INACTIVE;
                }
            }
        }
    }

    pub fn draw<R: Renderer>(&self, renderer: &mut ResourceManager<R>) -> Result<()> {
        match self.state {
            State::INACTIVE => Ok(()),
            State::ACTIVE => self.animation.draw(None, renderer),
            State::EXPLODED => self.explosion.draw(None, renderer),
        }
    }
}

impl<I: Intersect<Rectangle>> Collidable<Rectangle, I> for Star {
    fn collides(&self, shape: &I) -> bool {
        shape.intersects(&self.body)
    }
}
