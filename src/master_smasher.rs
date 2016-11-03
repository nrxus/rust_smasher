extern crate moho;
extern crate sdl2_image;
extern crate sdl2;
extern crate glm;

use self::sdl2::event::Event;
use self::sdl2::keyboard::Keycode;
use self::sdl2::render::{Renderer, Texture};

use self::sdl2_image::{LoadTexture, INIT_PNG, INIT_JPG};
use std::error::Error;
use std::path::Path;
use meteor;

pub fn run() -> Result<(), Box<Error>> {
    const WINDOW_HEIGHT: u32 = 600;
    const WINDOW_WIDTH: u32 = 800;

    let (mut renderer, mut event_pump) =
        try!(moho::init("Master Smasher", WINDOW_WIDTH, WINDOW_HEIGHT));
    let _image_context = try!(sdl2_image::init(INIT_PNG | INIT_JPG));
    let mut input_manager = moho::input_manager::InputManager::new();
    let background_path = Path::new("resources/background_game.png");
    let meteor_path = Path::new("resources/meteor.png");

    let background = Background { texture: try!(renderer.load_texture(background_path)) };
    let mut meteor = meteor::Meteor::new(try!(renderer.load_texture(meteor_path)),
                                         glm::uvec2(WINDOW_WIDTH, WINDOW_HEIGHT));

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
                Event::MouseButtonDown { x, y, .. } => {
                    if !meteor.is_launched() || true {
                        meteor.launch(glm::dvec2(x as f64, y as f64));
                    }
                }
                _ => {}
            }
        }
        if input_manager.is_key_down(Keycode::Escape) {
            break;
        }
        meteor.update();

        renderer.clear();
        try!(background.draw(&mut renderer));
        try!(meteor.draw(&mut renderer));
        renderer.present();
    }

    Ok(())
}

struct Background {
    texture: Texture,
}

impl Background {
    fn draw(&self, renderer: &mut Renderer) -> Result<(), Box<Error>> {
        try!(renderer.copy(&self.texture, None, None));

        Ok(())
    }
}
