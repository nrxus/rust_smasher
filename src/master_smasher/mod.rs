mod drawable;
mod level;
mod shape;

use self::level::Level;

use errors::*;
use moho::input_manager::InputManager;
use moho::resource_manager::{ResourceManager, TextureId};
use moho::timer::Timer;
use moho::MohoEngine;
use sdl2::keyboard::Keycode;

use std::time::Duration;

pub struct MasterSmasher<E: MohoEngine> {
    level: Level,
    background: TextureId,
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
               background: background.id,
               input_manager: input_manager,
               renderer: renderer,
           })
    }

    pub fn run(&mut self) -> Result<()> {
        const GAME_SPEED: u32 = 60;
        const MAX_SKIP: u32 = 10;
        let update_duration = Duration::new(0, 1000000000 / GAME_SPEED);
        let mut timer = Timer::new();
        let mut delta: Duration = Default::default();
        while !self.game_quit() {
            let game_time = timer.update();
            delta += game_time.since_update;
            let mut loops: u32 = 0;
            while delta >= update_duration && loops < MAX_SKIP {
                self.input_manager.update();
                if self.game_quit() {
                    break;
                }
                self.update();
                delta -= update_duration;
                loops += 1;
            }
            if self.game_quit() {
                break;
            }
            self.level.animate(game_time.since_update);
            let interpolation = delta.subsec_nanos() as f64 / update_duration.subsec_nanos() as f64;
            self.draw(interpolation)?;
        }
        Ok(())
    }

    fn update(&mut self) {
        self.level.update(&self.input_manager);
    }

    fn draw(&mut self, interpolation: f64) -> Result<()> {
        self.renderer.clear();
        self.renderer.show(&self.background)?;
        self.level.draw(interpolation, &mut self.renderer)?;
        self.renderer.present();
        Ok(())
    }

    fn game_quit(&self) -> bool {
        self.input_manager.game_quit() || self.input_manager.is_key_down(Keycode::Escape)
    }
}
