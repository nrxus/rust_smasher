mod drawable;
mod level;
mod shape;

use self::level::Level;

use errors::*;
use moho::input_manager::InputManager;
use moho::renderer::Renderer;
use moho::resource_manager::{ResourceManager, Texture};
use moho::timer::Timer;
use moho::MohoEngine;
use sdl2::keyboard::Keycode;

use std::time::Duration;

pub struct MasterSmasher<E: MohoEngine> {
    level: Level,
    background: Texture,
    input_manager: InputManager<E::EventPump>,
    renderer: ResourceManager<E::Renderer>,
}

impl<E: MohoEngine> MasterSmasher<E> {
    pub fn new(mut renderer: ResourceManager<E::Renderer>,
               input_manager: InputManager<E::EventPump>)
               -> Result<Self> {
        let background = renderer.load_texture("resources/background_game.png")?;
        let window_size = renderer.output_size()?;
        renderer.wrap_coords = Some(window_size);
        let level = Level::load("levels/level_1.lvl", window_size, &renderer)?;
        Ok(MasterSmasher {
            level: level,
            background: background,
            input_manager: input_manager,
            renderer: renderer,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        let mut timer = Timer::new();
        while !self.game_quit() {
            let game_time = timer.update();
            self.update();
            self.draw(game_time.since_update)?;
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

    fn draw(&mut self, delta: Duration) -> Result<()> {
        self.level.animate(delta);
        let drawables = self.level.drawables();
        self.renderer.clear();
        self.renderer.draw(self.background.id, None, None)?;
        for drawable in drawables {
            drawable.draw(&mut self.renderer)?;
        }
        self.renderer.present();
        Ok(())
    }
}
