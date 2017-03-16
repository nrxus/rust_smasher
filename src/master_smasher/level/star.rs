use super::collidable::Collidable;
use master_smasher::drawable::{Animation, Drawable};
use master_smasher::shape::{Intersect, Rectangle};
use errors::*;

use glm;
use moho::renderer::Renderer;
use moho::resource_manager::ResourceManager;

use std::time::Duration;

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

impl<R: Renderer> Drawable<ResourceManager<R>> for Star {
    fn draw(&self, renderer: &mut ResourceManager<R>) -> Result<()> {
        self.animation.asset.draw(renderer)
    }
}
