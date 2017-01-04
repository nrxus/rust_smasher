extern crate moho;
extern crate glm;
extern crate sdl2;

mod master_smasher;
mod meteor;
mod shape;
mod rectangle;
mod circle;
mod planet;
mod animation;
mod explosion;

fn main() {
    const WINDOW_WIDTH: u32 = 1280;
    const WINDOW_HEIGHT: u32 = 720;
    let (renderer, input_manager) = moho::init("Master Smasher", WINDOW_WIDTH, WINDOW_HEIGHT)
        .unwrap();
    let mut game =
        master_smasher::MasterSmasher::<moho::SdlMohoEngine>::new(renderer, input_manager).unwrap();
    game.run().unwrap();
}
