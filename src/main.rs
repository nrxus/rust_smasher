mod master_smasher;
mod meteor;
mod shape;
mod rectangle;
mod circle;
mod planet;
mod animation;
mod sprite_strip;

fn main() {
    let mut game = master_smasher::MasterSmasher::new().unwrap();
    game.run().unwrap();
}
