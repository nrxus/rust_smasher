use std::path::Path;
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

use glm;
use sdl2::rect;
use sdl2::render::Renderer as SdlRenderer;
use sdl2::render::Texture as SdlTexture;
use sdl2::image::LoadTexture;

use window_wrapper::*;
use errors::*;

pub struct TextureData<T> {
    pub texture: Rc<T>,
    pub dims: glm::UVec2,
}

impl<T> Clone for TextureData<T> {
    fn clone(&self) -> TextureData<T> {
        TextureData {
            texture: self.texture.clone(),
            dims: self.dims,
        }
    }
}

pub trait Renderer {
    type Texture;

    fn load_texture(&self, path: &Path) -> Result<TextureData<Self::Texture>>;
    fn output_size(&self) -> Result<(u32, u32)>;

    // Drawing methods
    fn clear(&mut self);
    fn present(&mut self);
    fn fill_rects(&mut self, rects: &[rect::Rect]) -> Result<()>;
    fn copy(&mut self,
            texture: &Self::Texture,
            src: Option<rect::Rect>,
            dst: Option<rect::Rect>)
            -> Result<()>;
}

impl Renderer for SdlRenderer<'static> {
    type Texture = SdlTexture;

    fn load_texture(&self, path: &Path) -> Result<TextureData<SdlTexture>> {
        let texture = LoadTexture::load_texture(self, path)?;
        let query = texture.query();
        Ok(TextureData {
            texture: Rc::new(texture),
            dims: glm::uvec2(query.width, query.height),
        })
    }

    fn output_size(&self) -> Result<(u32, u32)> {
        Ok(self.output_size()?)
    }

    fn copy(&mut self,
            texture: &SdlTexture,
            src: Option<rect::Rect>,
            dst: Option<rect::Rect>)
            -> Result<()> {
        Ok(self.copy(texture, src, dst)?)
    }

    fn clear(&mut self) {
        self.clear();
    }

    fn present(&mut self) {
        self.present();
    }

    fn fill_rects(&mut self, rects: &[rect::Rect]) -> Result<()> {
        Ok(self.fill_rects(rects)?)
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

    pub fn load_texture(&self, path: &'static str) -> Result<TextureData<R::Texture>> {
        match self.load_cached_texture(path) {
            Some(texture) => Ok(texture),
            None => self.load_new_texture(path),
        }
    }

    pub fn draw_from_center(&mut self,
                            texture: &R::Texture,
                            src: Option<glm::IVec4>,
                            center: glm::IVec2,
                            dims: glm::UVec2,
                            wrapping_coords: Option<glm::UVec2>)
                            -> Result<()> {
        let width = dims.x as i32;
        let height = dims.y as i32;
        let dst = glm::ivec4(center.x - width / 2, center.y - height / 2, width, height);
        self.draw(texture, src, Some(dst), wrapping_coords)
    }

    pub fn draw(&mut self,
                texture: &R::Texture,
                src: Option<glm::IVec4>,
                dst: Option<glm::IVec4>,
                wrapping_coords: Option<glm::UVec2>)
                -> Result<()> {
        match (dst, wrapping_coords) {
            (Some(d), Some(w)) => self.draw_and_wrap(texture, src, d, w),
            _ => self.draw_raw(texture, src, dst),
        }
    }

    pub fn fill_rects(&mut self, rects: &[rect::Rect]) -> Result<()> {
        self.renderer.fill_rects(rects)
    }

    pub fn clear(&mut self) {
        self.renderer.clear();
    }

    pub fn present(&mut self) {
        self.renderer.present();
    }

    pub fn output_size(&self) -> Result<glm::UVec2> {
        let (x, y) = self.renderer.output_size()?;
        Ok(glm::uvec2(x, y))
    }

    fn load_cached_texture(&self, path: &'static str) -> Option<TextureData<R::Texture>> {
        let cache = self.texture_cache.borrow();
        match cache.get(path) {
            Some(texture) => Some(texture.clone()),
            None => None,
        }
    }

    fn load_new_texture(&self, path: &'static str) -> Result<TextureData<R::Texture>> {
        let mut cache = self.texture_cache.borrow_mut();
        let texture_path = Path::new(path);
        let texture_data = self.renderer.load_texture(texture_path)?;
        cache.insert(path, texture_data.clone());
        Ok(texture_data)
    }

    fn draw_and_wrap(&mut self,
                     texture: &R::Texture,
                     src: Option<glm::IVec4>,
                     dst: glm::IVec4,
                     wrapping_coords: glm::UVec2)
                     -> Result<()> {
        wrap_rects(dst, wrapping_coords)
            .iter()
            .filter_map(|&r| r)
            .map(|r| self.draw_raw(texture, src, Some(r)))
            .fold(Ok(()), |res, x| { if res.is_err() { res } else { x } })
    }

    fn draw_raw(&mut self,
                texture: &R::Texture,
                src: Option<glm::IVec4>,
                dst: Option<glm::IVec4>)
                -> Result<()> {
        self.renderer.copy(texture, Self::get_rect(src), Self::get_rect(dst))
    }

    fn get_rect(rect: Option<glm::IVec4>) -> Option<rect::Rect> {
        match rect {
            Some(r) => Some(rect::Rect::new(r.x, r.y, r.z as u32, r.w as u32)),
            None => None,
        }
    }
}
