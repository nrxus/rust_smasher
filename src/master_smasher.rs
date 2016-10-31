extern crate moho;
extern crate sdl2_image;

use self::sdl2_image::{LoadTexture, INIT_PNG, INIT_JPG};
use std::error::Error;
use std::path::Path;

pub fn run() -> Result<(), Box<Error>> {
    let (mut renderer, event_pump) = try!(moho::init("Master Smasher", 800, 600));
    let _image_context = try!(sdl2_image::init(INIT_PNG | INIT_JPG));
    let background_path = Path::new("resources/background_menu.png");
    let texture = try!(renderer.load_texture(background_path));
    renderer.copy(&texture, None, None);
    renderer.present();
    Ok(())
}
