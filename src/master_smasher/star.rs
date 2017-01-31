use super::drawable::{Animation, AssetManager, AnimationAsset, Drawable};
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
        let animation = asset_manager.get_animation(AnimationAsset::Star, center);
        let explosion = asset_manager.get_animation(AnimationAsset::ExplosionSmall, center);
        let dims = glm::to_dvec2(animation.asset.dims());

        let body = Rectangle {
            center: glm::to_dvec2(center),
            dims: dims,
        };

        Star {
            body: body,
            explosion: explosion,
            animation: animation,
        }
    }

    pub fn explode(self) -> Animation {
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
