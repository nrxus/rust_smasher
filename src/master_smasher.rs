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
    const WINDOW_HEIGHT: u32 = 600;
    const WINDOW_WIDTH: u32 = 800;
    let (mut renderer, mut event_pump) =
        try!(moho::init("Master Smasher", WINDOW_WIDTH, WINDOW_HEIGHT));
    let _image_context = try!(sdl2_image::init(INIT_PNG | INIT_JPG));
    let mut input_manager = moho::input_manager::InputManager::new();
    let background_path = Path::new("resources/background_game.png");
    let meteor_path = Path::new("resources/meteor.png");
    let background_texture = try!(renderer.load_texture(background_path));
    let meteor_texture = try!(renderer.load_texture(meteor_path));
    let query = meteor_texture.query();
    let mut dst = rect::Rect::new(0, 0, query.width, query.height);
    let mut dst2 = None;

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

            if (dst.top() > WINDOW_HEIGHT as i32) {
                let top = dst.top() % WINDOW_HEIGHT as i32;
                dst.set_y(top);
                dst2 = None
            }

            if (dst.bottom() > WINDOW_HEIGHT as i32) {
                let mut clone_rect = dst.clone();
                clone_rect.set_bottom(dst.bottom() % WINDOW_HEIGHT as i32);
                dst2 = Some(clone_rect);
            }
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

        match dst2 {
            Some(_) => {
                try!(renderer.copy(&meteor_texture, None, dst2));
            }
            None => {}
        }

        renderer.present();
    }

    Ok(())
}
