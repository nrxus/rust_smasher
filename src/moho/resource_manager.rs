extern crate sdl2;
extern crate glm;

use sdl2::render::Renderer as SdlRenderer;
use sdl2::render::Texture as SdlTexture;
use sdl2_image::LoadTexture;
use std::path::Path;
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;
use sdl2::rect;

pub struct TextureData<T> {
    pub texture: T,
    pub width: u32,
    pub height: u32,
}

pub trait Renderer {
    type Texture;
    fn load_texture(&self, path: &Path) -> Result<TextureData<Self::Texture>, String>;
    fn output_size(&self) -> Result<(u32, u32), String>;

    fn clear(&mut self);
    fn present(&mut self);
    fn copy(&mut self,
            texture: &Self::Texture,
            src: Option<rect::Rect>,
            dst: Option<rect::Rect>)
            -> Result<(), String>;
}

impl<'a> Renderer for SdlRenderer<'a> {
    type Texture = SdlTexture;

    fn load_texture(&self, path: &Path) -> Result<TextureData<SdlTexture>, String> {
        let texture = LoadTexture::load_texture(self, path)?;
        let query = texture.query();
        Ok(TextureData {
            texture: texture,
            width: query.width,
            height: query.height,
        })
    }

    fn output_size(&self) -> Result<(u32, u32), String> {
        self.output_size()
    }

    fn copy(&mut self,
            texture: &SdlTexture,
            src: Option<rect::Rect>,
            dst: Option<rect::Rect>)
            -> Result<(), String> {
        self.copy(texture, src, dst)
    }

    fn clear(&mut self) {
        self.clear();
    }

    fn present(&mut self) {
        self.present();
    }
}

pub struct ResourceManager<'a, R: Renderer> {
    texture_cache: RefCell<HashMap<&'a str, Rc<TextureData<R::Texture>>>>,
    renderer: R,
}

impl<'a, R: Renderer> ResourceManager<'a, R> {
    pub fn new(renderer: R) -> Self {
        ResourceManager {
            texture_cache: RefCell::new(HashMap::new()),
            renderer: renderer,
        }
    }

    pub fn load_texture(&self, path: &'a str) -> Result<Rc<TextureData<R::Texture>>, String> {
        {
            let cache = self.texture_cache.borrow();
            let texture = cache.get(path);
            if let Some(x) = texture {
                return Ok(x.clone());
            }
        }
        let mut cache = self.texture_cache.borrow_mut();
        let texture_path = Path::new(path);
        let texture = Rc::new(self.renderer.load_texture(texture_path)?);
        cache.insert(path, texture.clone());
        Ok(texture.clone())
    }

    pub fn draw(&mut self,
                texture: &R::Texture,
                src: Option<rect::Rect>,
                dst: Option<rect::Rect>)
                -> Result<(), String> {
        self.renderer.copy(texture, src, dst)
    }

    pub fn clear(&mut self) {
        self.renderer.clear();
    }

    pub fn present(&mut self) {
        self.renderer.present();
    }
}
