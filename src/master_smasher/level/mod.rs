mod unlaunched_meteor;
mod launched_meteor;
mod star;
mod planet;
mod level_data;
mod collidable;
mod player;
mod player_assets;
mod world;

use self::level_data::LevelData;
use self::player::{MeteorState, Player};
use self::player_assets::PlayerAssets;
use self::world::{World, WorldAssets};
use super::drawable::Drawable;
use errors::*;

use glm;
use moho::input_manager::{EventPump, InputManager};
use moho::renderer::Renderer;
use moho::resource_manager::ResourceManager;

pub struct Level {
    world: World,
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
        let player_assets = PlayerAssets::new(resource_manager)?;
        let world_assets = WorldAssets::new(resource_manager)?;
        Ok(Level::new(data, size, player_assets, world_assets))
    }

    pub fn new(data: LevelData,
               window_size: glm::UVec2,
               player_assets: PlayerAssets,
               world_assets: WorldAssets)
               -> Level {
        let world = World::new(&data, world_assets);
        let meteor_center = glm::ivec2(data.meteor.x, data.meteor.y);
        let player = Player::new(player_assets, meteor_center, window_size);

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

        self.world.update();
    }

    pub fn drawables(&self) -> Vec<Drawable> {
        let world = self.world.drawables().into_iter();
        world.chain(self.player.drawables().into_iter()).collect()
    }
}
