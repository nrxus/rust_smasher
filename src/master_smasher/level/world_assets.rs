use master_smasher::drawable::{Animation, AnimationData};

use glm::{self, GenNum};
use moho::errors::*;
use moho::resource_manager::{ResourceLoader, Texture};
use num_traits::One;

#[derive(Clone)]
pub struct WorldAssets {
    pub red_planet: Texture,
    pub white_planet: Texture,
    pub blue_planet: Texture,
    pub red_ring: Texture,
    pub white_ring: Texture,
    pub blue_ring: Texture,
    pub dead_planet: Texture,

    pub star: AnimationData,
    pub explosion: AnimationData,
    pub enemy: AnimationData,
}

impl WorldAssets {
    pub fn new<L: ResourceLoader>(resource_loader: &L) -> Result<Self> {
        let red_planet = resource_loader.load_texture("resources/red_planet.png")?;
        let white_planet = resource_loader.load_texture("resources/white_planet.png")?;
        let blue_planet = resource_loader.load_texture("resources/blue_planet.png")?;
        let dead_planet = resource_loader.load_texture("resources/dead_planet.png")?;
        let red_ring = resource_loader.load_texture("resources/red_ring.png")?;
        let white_ring = resource_loader.load_texture("resources/white_ring.png")?;
        let blue_ring = resource_loader.load_texture("resources/blue_ring.png")?;

        let star_path = "resources/star.png";
        let enemy_path = "resources/spaceship.png";
        let explosion_path = "resources/explosion_small.png";
        let star = AnimationData::new(star_path, 2, 150, true, resource_loader)?;
        let enemy = AnimationData::new(enemy_path, 2, 100, true, resource_loader)?;
        let explosion = AnimationData::new(explosion_path, 10, 100, false, resource_loader)?;
        let assets = WorldAssets {
            red_planet: red_planet,
            white_planet: white_planet,
            blue_planet: blue_planet,
            dead_planet: dead_planet,
            red_ring: red_ring,
            white_ring: white_ring,
            blue_ring: blue_ring,
            star: star,
            enemy: enemy,
            explosion: explosion,
        };
        Ok(assets)
    }

    pub fn star(&self, center: glm::IVec2) -> Animation {
        Animation::from_data(self.star.clone(), center, glm::DVec2::one())
    }

    pub fn enemy(&self, center: glm::IVec2) -> Animation {
        Animation::from_data(self.enemy.clone(), center, glm::DVec2::from_s(0.75))
    }
}
