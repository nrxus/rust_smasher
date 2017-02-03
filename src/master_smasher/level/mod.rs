mod unlaunched_meteor;
mod launched_meteor;
mod star;
mod planet;
mod level_data;
mod collidable;
mod player;

use super::drawable::{Animation, Drawable, AssetManager};
use self::planet::Planet;
use self::player::{MeteorState, Player, PlayerAssets};
use self::star::{Star, StarAssets};
use self::level_data::LevelData;
use errors::*;

use glm;
use moho::input_manager::{EventPump, InputManager};
use moho::renderer::Renderer;
use moho::resource_manager::ResourceManager;

pub struct Level {
    planets: Vec<Planet>,
    stars: Vec<Star>,
    animations: Vec<Animation>,
    player: Player,
}

impl Level {
    pub fn load<R>(path: &'static str,
                   size: glm::UVec2,
                   resource_manager: &ResourceManager<R>)
                   -> Result<Level>
        where R: Renderer
    {
        let data = LevelData::load(path)?;
        let asset_manager = AssetManager::new(resource_manager)?;
        let player_assets = PlayerAssets::new(resource_manager)?;
        let star_assets = StarAssets::new(resource_manager)?;
        Ok(Level::new(data, size, player_assets, star_assets, asset_manager))
    }

    pub fn new(data: LevelData,
               window_size: glm::UVec2,
               player_assets: PlayerAssets,
               star_assets: StarAssets,
               asset_manager: AssetManager)
               -> Level {
        let planets = data.planets
            .iter()
            .map(|p| Planet::new(p, &asset_manager))
            .collect::<Vec<_>>();

        let stars = data.stars
            .iter()
            .map(|s| Star::new(s, star_assets.clone()))
            .collect::<Vec<_>>();

        let meteor_center = glm::ivec2(data.meteor.x, data.meteor.y);
        let player = Player::new(player_assets, meteor_center, window_size);

        Level {
            planets: planets,
            stars: stars,
            animations: Vec::new(),
            player: player,
        }
    }

    pub fn update<E: EventPump>(&mut self, input_manager: &InputManager<E>) {
        self.player.update(&self.planets, input_manager);
        if let MeteorState::LAUNCHED(ref m) = self.player.state {
            let collidable_indices = self.stars
                .iter()
                .enumerate()
                .filter(|&(_, s)| m.collides(s))
                .map(|(i, _)| i)
                .collect::<Vec<_>>();
            for i in collidable_indices {
                let star = self.stars.swap_remove(i);
                self.animations.push(star.explode());
            }
        }

        self.update_animations();
    }

    pub fn drawables(&self) -> Vec<Drawable> {
        let planets = self.planets.iter().map(Planet::drawables).flat_map(|v| v.into_iter());
        planets.chain(self.stars.iter().map(Star::drawables).flat_map(|v| v.into_iter()))
            .chain(self.animations.iter().map(|a| &a.asset).map(Drawable::Asset))
            .chain(self.player.drawables().into_iter())
            .collect()
    }

    fn update_animations(&mut self) {
        for star in &mut self.stars {
            star.update();
        }

        for animation in &mut self.animations {
            animation.update();
        }

        self.animations.retain(Animation::is_active);
    }
}
