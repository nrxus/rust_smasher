use master_smasher::drawable::{Animation, AnimationData, Drawable};
use super::level_data::LevelData;
use super::launched_meteor::LaunchedMeteor;
use super::planet::Planet;
use super::star::Star;

use glm;
use moho::errors::*;
use moho::renderer::Renderer;
use moho::resource_manager::{ResourceManager, Texture};

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
}

impl WorldAssets {
    pub fn new<R: Renderer>(resource_manager: &ResourceManager<R>) -> Result<Self> {
        let red_planet = resource_manager.load_texture("resources/red_planet.png")?;
        let white_planet = resource_manager.load_texture("resources/white_planet.png")?;
        let blue_planet = resource_manager.load_texture("resources/blue_planet.png")?;
        let dead_planet = resource_manager.load_texture("resources/dead_planet.png")?;
        let red_ring = resource_manager.load_texture("resources/red_ring.png")?;
        let white_ring = resource_manager.load_texture("resources/white_ring.png")?;
        let blue_ring = resource_manager.load_texture("resources/blue_ring.png")?;

        let star_path = "resources/star.png";
        let explosion_path = "resources/explosion_small.png";
        let star = AnimationData::new(star_path, 2, 150, true, resource_manager)?;
        let explosion = AnimationData::new(explosion_path, 10, 100, false, resource_manager)?;
        let assets = WorldAssets {
            red_planet: red_planet,
            white_planet: white_planet,
            blue_planet: blue_planet,
            dead_planet: dead_planet,
            red_ring: red_ring,
            white_ring: white_ring,
            blue_ring: blue_ring,
            star: star,
            explosion: explosion,
        };
        Ok(assets)
    }
}

pub struct World {
    pub planets: Vec<Planet>,
    pub stars: Vec<Star>,
    pub explosions: Vec<Animation>,
    pub explosion_data: AnimationData,
}

impl World {
    pub fn new(data: &LevelData, assets: WorldAssets) -> Self {
        let planets = data.planets
            .iter()
            .map(|p| Planet::new(p, &assets))
            .collect::<Vec<_>>();

        let stars = data.stars
            .iter()
            .map(|s| Star::new(s, &assets.star))
            .collect::<Vec<_>>();

        World {
            planets: planets,
            stars: stars,
            explosions: Vec::new(),
            explosion_data: assets.explosion,
        }
    }

    pub fn collide(&mut self, meteor: &LaunchedMeteor) {
        let collidable_indices = self.stars
            .iter()
            .enumerate()
            .filter(|&(_, s)| meteor.collides(s))
            .map(|(i, _)| i)
            .collect::<Vec<_>>();
        for i in collidable_indices {
            let star = self.stars.swap_remove(i);
            let center = glm::to_ivec2(star.center());
            self.explosions.push(Animation::start(&self.explosion_data, center));
        }
    }

    pub fn update(&mut self) {
        for star in &mut self.stars {
            star.update();
        }

        for animation in &mut self.explosions {
            animation.update();
        }

        self.explosions.retain(Animation::is_active);
    }

    pub fn drawables(&self) -> Vec<Drawable> {
        let planets = self.planets.iter().map(Planet::drawables).flat_map(|v| v.into_iter());
        let stars = self.stars.iter().map(Star::drawables).flat_map(|v| v.into_iter());
        let explosions = self.explosions.iter().map(|a| &a.asset).map(Drawable::Asset);
        planets.chain(stars).chain(explosions).collect()
    }
}
