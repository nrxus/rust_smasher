use super::asset_manager::{Animation, AssetManager, AnimationAsset, Drawable};
use super::collidable::Collidable;
use super::shape::{Intersect, Rectangle};

use glm;

pub struct Star {
    body: Rectangle,
    animation: Animation,
    explosion: Animation,
}

impl Star {
    pub fn new(center: glm::IVec2, asset_manager: &AssetManager) -> Self {
        let mut animation = asset_manager.get_animation(AnimationAsset::Star);
        animation.asset.set_center(center);
        let explosion = asset_manager.get_animation(AnimationAsset::ExplosionSmall);
        let rect = animation.asset.dst_rect;

        let body = Rectangle {
            center: glm::to_dvec2(center),
            dims: glm::dvec2(rect.z as f64, rect.w as f64),
        };

        Star {
            body: body,
            explosion: explosion,
            animation: animation,
        }
    }

    pub fn explode(mut self) -> Animation {
        self.explosion.asset.set_center(glm::to_ivec2(self.body.center));
        self.explosion
    }

    pub fn update(&mut self) {
        self.animation.update();
    }

    pub fn drawables(&self) -> Vec<Drawable> {
        vec![Drawable::Asset(&self.animation.asset)]
    }
}

impl<I: Intersect<Rectangle>> Collidable<Rectangle, I> for Star {
    fn collides(&self, shape: &I) -> bool {
        shape.intersects(&self.body)
    }
}
