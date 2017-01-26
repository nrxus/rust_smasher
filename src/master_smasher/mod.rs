mod asset_manager;
mod collidable;
mod meteor;
mod level;
mod planet;
mod shape;
mod star;

use self::asset_manager::AssetManager;
use self::level::Level;

use moho::errors::*;
use moho::input_manager::*;
use moho::resource_manager::*;
use moho::MohoEngine;
use sdl2::keyboard::Keycode;

pub struct MasterSmasher<E: MohoEngine> {
    level: Level,
    background: Texture,
    input_manager: InputManager<E::EventPump>,
    renderer: ResourceManager<E::Renderer>,
}

impl<E: MohoEngine> MasterSmasher<E> {
    pub fn new(renderer: ResourceManager<E::Renderer>,
               input_manager: InputManager<E::EventPump>)
               -> Result<Self> {
        let asset_manager = AssetManager::new(&renderer)?;
        let background = renderer.load_texture("resources/background_game.png")?;
        let level = Level::new(renderer.output_size()?, &asset_manager);
        Ok(MasterSmasher {
            level: level,
            background: background,
            input_manager: input_manager,
            renderer: renderer,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        while !self.game_quit() {
            self.update();
            self.draw()?;
        }
        Ok(())
    }

    fn update(&mut self) {
        self.input_manager.update();
        if self.game_quit() {
            return;
        }
        self.level.update(&self.input_manager);
    }

    fn game_quit(&self) -> bool {
        self.input_manager.game_quit() || self.input_manager.is_key_down(Keycode::Escape)
    }

    fn draw(&mut self) -> Result<()> {
        self.renderer.clear();
        self.renderer.draw(self.background.id, None, None, None)?;
        self.level.draw(&mut self.renderer)?;
        self.renderer.present();
        Ok(())
    }
}
