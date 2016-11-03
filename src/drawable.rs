extern crate sdl2;

use std::error::Error;
use self::sdl2::render::{Renderer, Texture};

pub struct Drawable {
    texture: Texture,
}

impl Drawable {
    pub fn new(texture: Texture) -> Self {
        Drawable { texture: texture }
    }

    pub fn draw(&self, renderer: &mut Renderer) -> Result<(), Box<Error>> {
        try!(renderer.copy(&self.texture, None, None));
        Ok(())
    }
}
