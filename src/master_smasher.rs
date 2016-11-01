extern crate moho;
extern crate sdl2_image;
extern crate sdl2;

use self::sdl2::event::Event;
use self::sdl2::keyboard::Keycode;
use self::sdl2::rect;

use self::sdl2_image::{LoadTexture, INIT_PNG, INIT_JPG};
use std::error::Error;
use std::path::Path;

pub fn run() -> Result<(), Box<Error>> {
    let (mut renderer, mut event_pump) = try!(moho::init("Master Smasher", 800, 600));
    let _image_context = try!(sdl2_image::init(INIT_PNG | INIT_JPG));
    let mut input_manager = moho::input_manager::InputManager::new();
    let background_path = Path::new("resources/background_game.png");
    let meteor_path = Path::new("resources/meteor.png");
    let background_texture = try!(renderer.load_texture(background_path));
    let meteor_texture = try!(renderer.load_texture(meteor_path));
    let query = meteor_texture.query();
    let mut dst = rect::Rect::new(0, 0, query.width, query.height);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown { keycode: Some(key), .. } => {
                    input_manager.press_key(key);
                }
                Event::KeyUp { keycode: Some(key), .. } => {
                    input_manager.release_key(key);
                }
                _ => {}
            }
        }

        if input_manager.is_key_down(Keycode::Escape) {
            break;
        }

        if input_manager.is_key_down(Keycode::Down) {
            dst.offset(0, 1);
        }

        if input_manager.is_key_down(Keycode::Up) {
            dst.offset(0, -1);
        }

        if input_manager.is_key_down(Keycode::Left) {
            dst.offset(-1, 0);
        }

        if input_manager.is_key_down(Keycode::Right) {
            dst.offset(1, 0);
        }

        // The rest of the game loop goes here...
        renderer.clear();
        try!(renderer.copy(&background_texture, None, None));
        try!(renderer.copy(&meteor_texture, None, Some(dst)));
        renderer.present();
    }

    Ok(())
}
