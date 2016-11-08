extern crate sdl2;

use sdl2::render::{Renderer, Texture};
use sdl2_image::LoadTexture;
use std::path::Path;
use std::collections::HashMap;

struct ResourceManager<'a> {
    texture_cache: HashMap<&'a str, Texture>,
    renderer: Renderer<'a>,
}

impl<'a> ResourceManager<'a> {
    // pub fn new() -> Self {}
    pub fn getTexture(&mut self, path: &'a str) -> Result<&Texture, String> {
        if !self.texture_cache.contains_key(path) {
            let image_path = Path::new(path);
            let texture = try!(self.renderer.load_texture(image_path));
            self.texture_cache.insert(path, texture);
        }
        self.texture_cache.get(path).ok_or("Texture Cache Error: No such path".into())
    }
}
