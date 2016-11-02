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
                Event::MouseButtonDown { x, y, .. } => {
                    if !meteor.launched || true {
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

struct Meteor {
    texture: Texture,
    center: glm::Vector2<f64>,
    dims: glm::Vector2<u32>,
    max_coords: glm::Vector2<u32>,
    velocity: glm::Vector2<f64>,
    launched: bool,
}

impl Meteor {
    fn new(texture: Texture, max_coords: glm::Vector2<u32>) -> Self {
        let query = texture.query();
        let center = glm::dvec2(0., 0.);
        let dims = glm::uvec2(query.width, query.height);

        Meteor {
            texture: texture,
            center: center,
            dims: dims,
            max_coords: max_coords,
            velocity: glm::dvec2(0., 0.),
            launched: false,
        }
    }

    fn launch(&mut self, target: glm::Vector2<f64>) {
        const FACTOR: f64 = 85.;
        let offset = target - self.center;
        self.velocity = glm::dvec2(offset.x as f64 / FACTOR, offset.y as f64 / FACTOR);
        self.launched = true;
    }

    fn update(&mut self) {
        self.center.y += self.velocity.y;
        self.center.x += self.velocity.x;

        let max_height = self.max_coords.y as f64;
        let max_width = self.max_coords.x as f64;

        self.center.y = (self.center.y + max_height) % max_height;
        self.center.x = (self.center.x + max_width) % max_width;
    }

    fn draw(&self, renderer: &mut Renderer) -> Result<(), Box<Error>> {
        let rects = self.drawing_rectangles();
        let results = rects.iter()
            .filter(|r| r.is_some())
            .map(|r| renderer.copy(&self.texture, None, *r));

        for result in results {
            try!(result)
        }

        Ok(())
    }

    fn drawing_rectangles(&self) -> [Option<rect::Rect>; 4] {
        let top = self.center.y as i32 - self.dims.y as i32 / 2;
        let bottom = self.center.y as i32 + self.dims.y as i32 / 2;
        let left = self.center.x as i32 - self.dims.x as i32 / 2;
        let right = self.center.x as i32 + self.dims.x as i32 / 2;

        let rect = rect::Rect::new(left, top, self.dims.x, self.dims.y);
        let mut rects: [Option<rect::Rect>; 4] = [Some(rect), None, None, None];
        let mut count = 1;

        let max_height = self.max_coords.y as i32;
        let max_width = self.max_coords.x as i32;

        if top < 0 {
            let mut rect = rect.clone();
            rect.set_y(top + max_height);
            rects[count] = Some(rect);
            count += 1;

            if left < 0 {
                rect.set_x(left + max_width);
                rects[count] = Some(rect);
                count += 1;
            } else if right > max_width {
                rect.set_right(right % max_width);
                rects[count] = Some(rect);
                count += 1;
            }
        } else if bottom > max_height {
            let mut rect = rect.clone();
            rect.set_bottom(bottom % max_height);
            rects[count] = Some(rect);
            count += 1;

            if left < 0 {
                rect.set_x(left + max_width);
                rects[count] = Some(rect);
                count += 1;
            } else if right > max_width {
                rect.set_right(right % max_width);
                rects[count] = Some(rect);
                count += 1;
            }
        }

        if left < 0 {
            let mut rect = rect.clone();
            rect.set_x(left + max_width);
            rects[count] = Some(rect);
        } else if right > max_width {
            let mut rect = rect.clone();
            rect.set_right(right % max_width);
            rects[count] = Some(rect);
        }

        rects
    }
}
