use super::collidable::Collidable;
use master_smasher::drawable::{Animation, Drawable};
use master_smasher::shape::{Intersect, Rectangle};

use glm;

pub struct Star {
    body: Rectangle,
    animation: Animation,
}

impl Star {
    pub fn new(animation: Animation) -> Self {
        let dims = glm::to_dvec2(animation.asset.dims());

        let body = Rectangle {
            center: glm::to_dvec2(animation.asset.center()),
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
