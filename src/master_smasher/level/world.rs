use super::level_data::LevelData;
use super::launched_meteor::LaunchedMeteor;
use super::planet::Planet;
use super::star::Star;
use super::world_assets::WorldAssets;
use master_smasher::drawable::{Animation, AnimationData, Drawable};

use glm;

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
