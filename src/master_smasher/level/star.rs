use super::collidable::Collidable;
use super::level_data::ObjectData;
use master_smasher::drawable::{Animation, AnimationData, Drawable};
use master_smasher::shape::{Intersect, Rectangle};

use glm;

pub struct Star {
    body: Rectangle,
    animation: Animation,
}

impl Star {
    pub fn new(data: &ObjectData, animation: &AnimationData) -> Self {
        let center = glm::ivec2(data.x, data.y);
        let animation = Animation::start(animation, center);
        let dims = glm::to_dvec2(animation.asset.dims());

        let body = Rectangle {
            center: glm::to_dvec2(center),
            dims: dims,
        };

        Star {
            body: body,
            animation: animation,
        }
    }

    pub fn update(&mut self) {
        self.animation.update();
    }

    pub fn drawables(&self) -> Vec<Drawable> {
        vec![Drawable::Asset(&self.animation.asset)]
    }

    pub fn center(&self) -> glm::DVec2 {
        self.body.center
    }
}

impl<I: Intersect<Rectangle>> Collidable<Rectangle, I> for Star {
    fn collides(&self, shape: &I) -> bool {
        shape.intersects(&self.body)
    }
}
