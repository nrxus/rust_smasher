use master_smasher::drawable::{Animation, AnimationData, Asset};

use glm;
use moho::errors::*;
use moho::resource_manager::{ResourceManager, Texture};
use moho::renderer::Renderer;
use num_traits::One;

#[derive(Clone)]
pub struct PlayerAssets {
    meteor: Texture,
    explosion: AnimationData,
}

impl PlayerAssets {
    pub fn new<R: Renderer>(resource_manager: &ResourceManager<R>) -> Result<Self> {
        let meteor = resource_manager.load_texture("resources/meteor.png")?;
        let explosion_path = "resources/explosion_large.png";
        let explosion = AnimationData::new(explosion_path, 8, 80, false, resource_manager)?;
        let assets = PlayerAssets {
            meteor: meteor,
            explosion: explosion,
        };
        Ok(assets)
    }

    pub fn meteor(&self, center: glm::IVec2) -> Asset {
        Asset::from_texture(&self.meteor, center)
    }

    pub fn explosion(&self, center: glm::IVec2) -> Animation {
        Animation::start(&self.explosion, center, glm::DVec2::one())
    }
}
