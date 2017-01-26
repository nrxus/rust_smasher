use super::asset_manager::{Animation, AssetManager, AnimationAsset};
use super::collidable::Collidable;
use super::shape::{Intersect, Rectangle};

use glm;
use moho::errors::*;
use moho::renderer::Renderer;
use moho::resource_manager::ResourceManager;

pub struct Star {
    body: Rectangle,
    animation: Animation,
    explosion: Animation,
}

impl Star {
    pub fn new(center: glm::IVec2, asset_manager: &AssetManager) -> Self {
        let mut animation = asset_manager.get_animation(AnimationAsset::Star);
        animation.set_center(center);
        let explosion = asset_manager.get_animation(AnimationAsset::ExplosionSmall);

        let body = Rectangle {
            center: glm::to_dvec2(center),
            dims: glm::dvec2(animation.dst_rect().z as f64, animation.dst_rect().w as f64),
        };

        Star {
            body: body,
            explosion: explosion,
            animation: animation,
        }
    }

    pub fn explode(mut self) -> Animation {
        self.explosion.set_center(glm::to_ivec2(self.body.center));
        self.explosion
    }

    pub fn update(&mut self) {
        self.animation.update();
    }

    pub fn draw<R: Renderer>(&self, renderer: &mut ResourceManager<R>) -> Result<()> {
        self.animation.draw(None, renderer)
    }
}

impl<I: Intersect<Rectangle>> Collidable<Rectangle, I> for Star {
    fn collides(&self, shape: &I) -> bool {
        shape.intersects(&self.body)
    }
}
