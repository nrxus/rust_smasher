mod unlaunched_meteor;
mod launched_meteor;
mod star;
mod planet;
mod level_data;
mod collidable;
mod player;
mod player_assets;
mod world;
mod world_assets;
mod interpolate;

use self::level_data::LevelData;
use self::player::{MeteorState, Player};
use self::player_assets::PlayerAssets;
use self::world::World;
use self::world_assets::WorldAssets;
use errors::*;

use glm;
use moho::input_manager::{EventPump, InputManager};
use moho::resource_manager::{Renderer, ResourceLoader};

use std::time::Duration;

pub struct Level {
    world: World,
    player: Player,
}

impl Level {
    pub fn load<L>(path: &'static str, size: glm::UVec2, resource_loader: &L) -> Result<Level>
        where L: ResourceLoader
    {
        let data = LevelData::load(path)?;
        let player_assets = PlayerAssets::new(resource_loader)?;
        let world_assets = WorldAssets::new(resource_loader)?;
        Ok(Level::new(data, size, player_assets, world_assets))
    }

    pub fn new(data: LevelData,
               window_size: glm::UVec2,
               player_assets: PlayerAssets,
               world_assets: WorldAssets)
               -> Level {
        let world = World::new(&data, world_assets);
        let player = Player::new(player_assets, (&data.meteor).into(), window_size);

        Level {
            world: world,
            player: player,
        }
    }

    pub fn update<E: EventPump>(&mut self, input_manager: &InputManager<E>) {
        self.player.update(&self.world.planets, input_manager);

        if let MeteorState::LAUNCHED(ref m) = self.player.state {
            self.world.collide(m);
        }
    }

    pub fn animate(&mut self, delta: Duration) {
        self.player.animate(delta);
        self.world.animate(delta);
    }

    pub fn draw<R>(&self, interpolation: f64, renderer: &mut R) -> Result<()>
        where R: Renderer
    {
        renderer.show(&self.world)?;
        self.player.draw(interpolation, renderer)
    }
}
