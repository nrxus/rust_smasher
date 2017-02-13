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
    pub enemies: Vec<Star>,
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

        let enemies = data.enemies
            .iter()
            .map(|s| Star::new(s, &assets.enemy))
            .collect::<Vec<_>>();

        World {
            planets: planets,
            stars: stars,
            enemies: enemies,
            explosions: Vec::new(),
            explosion_data: assets.explosion,
        }
    }

    pub fn collide(&mut self, meteor: &LaunchedMeteor) {
        let explosion = &self.explosion_data;
        let explosions = &mut self.explosions;

        self.stars.retain(|s| if meteor.collides(s) {
            let center = glm::to_ivec2(s.center());
            let animation = Animation::start(explosion, center);
            explosions.push(animation);
            false
        } else {
            true
        });

        self.enemies.retain(|e| if meteor.collides(e) {
            let center = glm::to_ivec2(e.center());
            let animation = Animation::start(explosion, center);
            explosions.push(animation);
            false
        } else {
            true
        });
    }

    pub fn update(&mut self) {
        for planet in &mut self.planets {
            planet.update();
        }

        for star in &mut self.stars {
            star.update();
        }

        for enemy in &mut self.enemies {
            enemy.update();
        }

        for animation in &mut self.explosions {
            animation.update();
        }

        self.explosions.retain(Animation::is_active);
    }

    pub fn drawables(&self) -> Vec<Drawable> {
        let planets = self.planets.iter().map(Planet::drawables).flat_map(|v| v.into_iter());
        let stars = self.stars.iter().map(Star::drawables).flat_map(|v| v.into_iter());
        let enemies = self.enemies.iter().map(Star::drawables).flat_map(|v| v.into_iter());
        let explosions = self.explosions.iter().map(|a| &a.asset).map(Drawable::Asset);
        planets.chain(stars).chain(enemies).chain(explosions).collect()
    }
}