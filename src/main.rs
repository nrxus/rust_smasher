extern crate glm;
extern crate moho;
extern crate num_traits;
extern crate sdl2;
extern crate serde_yaml;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate error_chain;

mod master_smasher;

pub mod errors {
    error_chain!{
        links {
            Moho(::moho::errors::Error, ::moho::errors::ErrorKind);
        }
        foreign_links {
            Io(::std::io::Error);
            Yaml(::serde_yaml::Error);
        }
    }
}

fn main() {
    const WINDOW_WIDTH: u32 = 1280;
    const WINDOW_HEIGHT: u32 = 720;
    let (renderer, input_manager) = moho::init("Master Smasher", WINDOW_WIDTH, WINDOW_HEIGHT)
        .unwrap();
    let mut game =
        master_smasher::MasterSmasher::<moho::SdlMohoEngine>::new(renderer, input_manager).unwrap();
    game.run().unwrap();
}
