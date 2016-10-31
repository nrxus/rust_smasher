extern crate moho;
extern crate sdl2_image;
extern crate sdl2;

use self::sdl2::event::Event;
use self::sdl2::keyboard::Keycode;

use self::sdl2_image::{LoadTexture, INIT_PNG, INIT_JPG};
use std::error::Error;
use std::path::Path;

pub fn run() -> Result<(), Box<Error>> {
    let (mut renderer, mut event_pump) = try!(moho::init("Master Smasher", 800, 600));
    let _image_context = try!(sdl2_image::init(INIT_PNG | INIT_JPG));
    let background_path = Path::new("resources/background_menu.png");
    let texture = try!(renderer.load_texture(background_path));
    try!(renderer.copy(&texture, None, None));
    renderer.present();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                _ => {}
            }
        }
        // The rest of the game loop goes here...
    }

    Ok(())
}
