use master_smasher::drawable::AnimationData;

use moho::errors::*;
use moho::resource_manager::{ResourceLoader, Texture};

#[derive(Clone)]
pub struct PlayerAssets {
    pub meteor: Texture,
    pub explosion: AnimationData,
}

impl PlayerAssets {
    pub fn new<R: ResourceLoader>(resource_manager: &R) -> Result<Self> {
        let meteor = resource_manager.load_texture("resources/meteor.png")?;
        let explosion_path = "resources/explosion_large.png";
        let explosion = AnimationData::new(explosion_path, 8, 80, false, resource_manager)?;
        let assets = PlayerAssets {
            meteor: meteor,
            explosion: explosion,
        };
        Ok(assets)
    }
}
