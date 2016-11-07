mod master_smasher;
mod meteor;
mod drawable;
mod shape;
mod rectangle;
mod circle;
mod planet;

fn main() {
    let mut game = master_smasher::MasterSmasher::new().unwrap();
    game.run().unwrap();
}
