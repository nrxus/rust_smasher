mod master_smasher;
mod meteor;

fn main() {
    let mut game = master_smasher::MasterSmasher::new().unwrap();
    game.run().unwrap();
}
