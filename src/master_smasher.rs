extern crate moho;
extern crate sdl2_image;
extern crate sdl2;
extern crate glm;

use self::sdl2::event::Event;
use self::sdl2::keyboard::Keycode;
use self::sdl2::render::{Renderer, Texture};
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

    let background = Background { texture: try!(renderer.load_texture(background_path)) };
    let mut meteor = Meteor::new(try!(renderer.load_texture(meteor_path)),
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
                _ => {}
            }
        }
        if input_manager.is_key_down(Keycode::Escape) {
            break;
        }
        meteor.update(&input_manager);

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

struct Meteor {
    texture: Texture,
    rect: rect::Rect,
    wrapping_rect: rect::Rect,
}

impl Meteor {
    fn new(texture: Texture, max_coords: glm::Vector2<u32>) -> Self {
        let query = texture.query();
        Meteor {
            texture: texture,
            rect: rect::Rect::new(0, 0, query.width, query.height),
            wrapping_rect: rect::Rect::new(0, 0, max_coords.x, max_coords.y),
        }
    }

    fn update(&mut self, input_manager: &moho::input_manager::InputManager) {
        let mut top = self.rect.y();
        let mut left = self.rect.x();

        if input_manager.is_key_down(Keycode::Down) {
            top += 1;
        }

        if input_manager.is_key_down(Keycode::Up) {
            top -= 1;
        }

        if input_manager.is_key_down(Keycode::Left) {
            left -= 1;
        }

        if input_manager.is_key_down(Keycode::Right) {
            left += 1;
        }

        let max_height = self.wrapping_rect.height() as i32;
        let max_width = self.wrapping_rect.width() as i32;

        self.rect.set_y((top + max_height) % max_height);
        self.rect.set_x((left + max_width) % max_width);
    }

    fn draw(&self, renderer: &mut Renderer) -> Result<(), Box<Error>> {
        try!(renderer.copy(&self.texture, None, Some(self.rect)));

        let mut optional_rect: Option<rect::Rect> = None;
        match optional_rect {
            Some(_) => {
                try!(renderer.copy(&self.texture, None, optional_rect));
            }
            None => {}
        };

        Ok(())
    }
}
