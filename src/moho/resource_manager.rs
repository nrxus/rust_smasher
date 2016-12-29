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

use window_wrapper::*;

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

    // Drawing methods
    fn clear(&mut self);
    fn present(&mut self);
    fn fill_rects(&mut self, rects: &[rect::Rect]) -> Result<(), String>;
    fn copy(&mut self,
            texture: &Self::Texture,
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

    fn fill_rects(&mut self, rects: &[rect::Rect]) -> Result<(), String> {
        self.fill_rects(rects)
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
        match self.load_cached_texture(path) {
            Some(texture) => Ok(texture),
            None => self.load_new_texture(path),
        }
    }

    pub fn draw(&mut self,
                texture: &R::Texture,
                src: Option<rect::Rect>,
                dst: Option<rect::Rect>,
                wrapping_coords: Option<glm::UVec2>)
                -> Result<(), String> {
        match (wrapping_coords, dst) {
            (Some(coords), Some(rect)) => {
                let dims = glm::uvec2(rect.width(), rect.height());
                let center = rect.center();
                let center = glm::uvec2(center.x() as u32, center.y() as u32);

                get_wrapped_centers(center, dims, coords)
                    .iter()
                    .filter_map(|&c| c)
                    .map(|c| {
                        let left = c.x - dims.x as i32 / 2;
                        let top = c.y - dims.y as i32 / 2;
                        rect::Rect::new(left, top, dims.x, dims.y)
                    })
                    .map(|r| self.renderer.copy(texture, src, Some(r)))
                    .fold(Ok(()), |res, x| { if res.is_err() { res } else { x } })
            }
            _ => self.renderer.copy(texture, src, dst),
        }
    }

    pub fn fill_rects(&mut self, rects: &[rect::Rect]) -> Result<(), String> {
        self.renderer.fill_rects(rects)
    }

    pub fn clear(&mut self) {
        self.renderer.clear();
    }

    pub fn present(&mut self) {
        self.renderer.present();
    }

    pub fn output_size(&self) -> Result<(u32, u32), String> {
        self.renderer.output_size()
    }

    fn load_cached_texture(&self, path: &'static str) -> Option<TextureData<R::Texture>> {
        let cache = self.texture_cache.borrow();
        match cache.get(path) {
            Some(texture) => Some(texture.clone()),
            None => None,
        }
    }

    fn load_new_texture(&self, path: &'static str) -> Result<TextureData<R::Texture>, String> {
        let mut cache = self.texture_cache.borrow_mut();
        let texture_path = Path::new(path);
        let texture_data = self.renderer.load_texture(texture_path)?;
        cache.insert(path, texture_data.clone());
        Ok(texture_data)
    }
}
