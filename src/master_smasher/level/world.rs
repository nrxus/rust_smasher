use super::level_data::LevelData;
use super::launched_meteor::LaunchedMeteor;
use super::planet::Planet;
use super::star::Star;
use super::world_assets::WorldAssets;
use master_smasher::drawable::{Animation, AnimationData, Drawable, GameRenderer};
use errors::*;

use glm;
use moho::renderer::Renderer;
use moho::resource_manager::ResourceManager;
use num_traits::One;

use std::time::Duration;

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
            .collect();
        let stars = data.stars
            .iter()
            .map(|s| Star::new(assets.star(s.into())))
            .collect();
        let enemies = data.enemies
            .iter()
            .map(|e| Star::new(assets.enemy(e.into())))
            .collect();

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
            let animation = Animation::from_data(explosion.clone(), center, glm::DVec2::one());
            explosions.push(animation);
            false
        } else {
            true
        });

        self.enemies.retain(|e| if meteor.collides(e) {
            let center = glm::to_ivec2(e.center());
            let animation = Animation::from_data(explosion.clone(), center, glm::DVec2::one());
            explosions.push(animation);
            false
        } else {
            true
        });
    }

    pub fn animate(&mut self, delta: Duration) {
        for planet in &mut self.planets {
            planet.animate(delta);
        }

        for star in &mut self.stars {
            star.animate(delta);
        }

        for enemy in &mut self.enemies {
            enemy.animate(delta);
        }

        for animation in &mut self.explosions {
            animation.update(delta);
        }

        self.explosions.retain(Animation::is_active);
    }
}

impl<R: Renderer> Drawable<ResourceManager<R>> for World {
    fn draw(&self, renderer: &mut ResourceManager<R>) -> Result<()> {
        renderer.render_all(&self.planets)?;
        renderer.render_all(&self.enemies)?;
        renderer.render_all(&self.stars)?;
        renderer.render_all(&self.explosions)
    }
}
