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
    let background_path = Path::new("resources/background_game.png");
    let meteor_path = Path::new("resources/meteor.png");
    let background_texture = try!(renderer.load_texture(background_path));
    let meteor_texture = try!(renderer.load_texture(meteor_path));
    let query = meteor_texture.query();
    let mut dst = rect::Rect::new(0, 0, query.width, query.height);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    dst.offset(0, 1);
                }
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    dst.offset(0, -1);
                }
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    dst.offset(-1, 0);
                }
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    dst.offset(1, 0);
                }
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        renderer.clear();
        try!(renderer.copy(&background_texture, None, None));
        try!(renderer.copy(&meteor_texture, None, Some(dst)));
        renderer.present();
    }

    Ok(())
}
