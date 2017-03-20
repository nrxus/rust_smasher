use super::collidable::Collidable;
use master_smasher::drawable::Animation;
use master_smasher::shape::{Intersect, Rectangle};

use glm;
use moho::renderer::Renderer;
use moho::resource_manager::{ResourceManager, Scene};
use moho::errors as moho_errors;

use std::time::Duration;

pub struct Star {
    body: Rectangle,
    animation: Animation,
}

impl Star {
    pub fn new(animation: Animation) -> Self {
        let rect = animation.dst_rect;
        let dims = glm::dvec2(rect.z as f64, rect.w as f64);
        let center = glm::dvec2((rect.x + rect.z / 2) as f64, (rect.y + rect.w / 2) as f64);

        let body = Rectangle {
            center: center,
            dims: dims,
        };

        Star {
            body: body,
            animation: animation,
        }
    }

    pub fn animate(&mut self, delta: Duration) {
        self.animation.update(delta);
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

impl Scene for Star {
    fn show<R: Renderer>(&self, renderer: &mut ResourceManager<R>) -> moho_errors::Result<()> {
        renderer.show(&self.animation)
    }
}
