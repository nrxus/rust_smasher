extern crate glm;
extern crate moho;
extern crate num_traits;
extern crate sdl2;

mod animation;
mod asset;
mod asset_manager;
mod circle;
mod collidable;
mod master_smasher;
mod meteor;
mod shape;
mod star;
mod rectangle;
mod planet;

fn main() {
    const WINDOW_WIDTH: u32 = 1280;
    const WINDOW_HEIGHT: u32 = 720;
    let (renderer, input_manager) = moho::init("Master Smasher", WINDOW_WIDTH, WINDOW_HEIGHT)
        .unwrap();
    let mut game =
        master_smasher::MasterSmasher::<moho::SdlMohoEngine>::new(renderer, input_manager).unwrap();
    game.run().unwrap();
}
