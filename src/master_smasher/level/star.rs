use super::collidable::Collidable;
use super::level_data::ObjectData;
use master_smasher::drawable::{Animation, AnimationData, Asset, Drawable};
use master_smasher::shape::{Intersect, Rectangle};

use moho::errors::*;
use moho::renderer::Renderer;
use moho::resource_manager::ResourceManager;

use glm;

#[derive(Clone)]
pub struct StarAssets {
    star: AnimationData,
    explosion: AnimationData,
}

impl StarAssets {
    pub fn new<R: Renderer>(resource_manager: &ResourceManager<R>) -> Result<Self> {
        let star_path = "resources/star.png";
        let explosion_path = "resources/explosion_small.png";
        let star = AnimationData::new(star_path, 2, 150, true, resource_manager)?;
        let explosion = AnimationData::new(explosion_path, 10, 100, false, resource_manager)?;
        let assets = StarAssets {
            star: star,
            explosion: explosion,
        };
        Ok(assets)
    }

    pub fn star(&self, center: glm::IVec2) -> Animation {
        Animation::start(&self.star, center)
    }

    pub fn explosion(&self, center: glm::IVec2) -> Animation {
        Animation::start(&self.explosion, center)
    }
}

pub struct Star {
    body: Rectangle,
    animation: Animation,
    assets: StarAssets,
}

impl Star {
    pub fn new(data: &ObjectData, assets: StarAssets) -> Self {
        let center = glm::ivec2(data.x, data.y);
        let animation = assets.star(center);
        let dims = glm::to_dvec2(animation.asset.dims());

        let body = Rectangle {
            center: glm::to_dvec2(center),
            dims: dims,
        };

        Star {
            body: body,
            animation: animation,
            assets: assets,
        }
    }

    pub fn explode(self) -> Animation {
        self.assets.explosion(glm::to_ivec2(self.body.center))
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
