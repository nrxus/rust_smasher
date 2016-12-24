extern crate sdl2;
extern crate glm;

use sdl2::render::Renderer as SdlRenderer;
use sdl2::render::Texture as SdlTexture;
use sdl2::image::LoadTexture;
use std::path::Path;
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;
use sdl2::rect;

pub struct TextureData<T> {
    pub texture: Rc<T>,
    pub width: u32,
    pub height: u32,
}

impl<T> Clone for TextureData<T> {
    fn clone(&self) -> TextureData<T> {
        TextureData {
            texture: self.texture.clone(),
            width: self.width,
            height: self.height,
        }
    }
}

pub trait Renderer {
    type Texture;
    fn load_texture(&self, path: &Path) -> Result<TextureData<Self::Texture>, String>;
    fn output_size(&self) -> Result<(u32, u32), String>;

    fn clear(&mut self);
    fn present(&mut self);
    fn copy(&mut self,
            texture: Rc<Self::Texture>,
            src: Option<rect::Rect>,
            dst: Option<rect::Rect>)
            -> Result<(), String>;
}

impl Renderer for SdlRenderer<'static> {
    type Texture = SdlTexture;

    fn load_texture(&self, path: &Path) -> Result<TextureData<SdlTexture>, String> {
        let texture = LoadTexture::load_texture(self, path)?;
        let query = texture.query();
        Ok(TextureData {
            texture: Rc::new(texture),
            width: query.width,
            height: query.height,
        })
    }

    fn output_size(&self) -> Result<(u32, u32), String> {
        self.output_size()
    }

    fn copy(&mut self,
            texture: Rc<SdlTexture>,
            src: Option<rect::Rect>,
            dst: Option<rect::Rect>)
            -> Result<(), String> {
        self.copy(&*texture, src, dst)
    }

    fn clear(&mut self) {
        self.clear();
    }

    fn present(&mut self) {
        self.present();
    }
}

pub struct ResourceManager<R: Renderer> {
    texture_cache: RefCell<HashMap<&'static str, TextureData<R::Texture>>>,
    renderer: R,
}

impl<R: Renderer> ResourceManager<R> {
    pub fn new(renderer: R) -> Self {
        ResourceManager {
            texture_cache: RefCell::new(HashMap::new()),
            renderer: renderer,
        }
    }

    pub fn load_texture(&self, path: &'static str) -> Result<TextureData<R::Texture>, String> {
        {
            let cache = self.texture_cache.borrow();
            let texture = cache.get(path);
            if let Some(x) = texture {
                return Ok(x.clone());
            }
        }
        let mut cache = self.texture_cache.borrow_mut();
        let texture_path = Path::new(path);
        let texture_data = self.renderer.load_texture(texture_path)?;
        cache.insert(path, texture_data.clone());
        Ok(texture_data)
    }

    pub fn draw(&mut self,
                texture: Rc<R::Texture>,
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
