mod master_smasher;
mod meteor;
mod drawable;

fn main() {
    let mut game = master_smasher::MasterSmasher::new().unwrap();
    game.run().unwrap();
}
