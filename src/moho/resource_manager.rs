use std::path::Path;
use std::collections::HashMap;
use std::cell::RefCell;

use glm;
use sdl2::rect;

use renderer::{ImageDims, Renderer};
use window_wrapper::*;
use errors::*;

#[derive(Copy,Clone)]
pub struct Texture {
    pub id: usize,
    pub dims: glm::UVec2,
}

pub struct ResourceManager<R: Renderer> {
    pub wrap_coords: Option<glm::UVec2>,
    texture_cache: RefCell<HashMap<&'static str, Texture>>,
    data_cache: RefCell<HashMap<usize, R::Texture>>,
    renderer: R,
}

impl<R: Renderer> ResourceManager<R> {
    pub fn new(renderer: R) -> Self {
        ResourceManager {
            wrap_coords: None,
            texture_cache: RefCell::new(HashMap::new()),
            data_cache: RefCell::new(HashMap::new()),
            renderer: renderer,
        }
    }

    pub fn load_texture(&self, path: &'static str) -> Result<Texture> {
        self.load_cached_texture(path).map_or_else(|| self.load_new_texture(path), Ok)
    }

    pub fn draw(&mut self,
                id: usize,
                dst: Option<glm::IVec4>,
                src: Option<glm::DVec4>)
                -> Result<()> {
        match (dst, self.wrap_coords) {
            (Some(d), Some(w)) => self.draw_and_wrap(id, d, src, w),
            _ => self.draw_raw(id, dst, src),
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

    fn load_cached_texture(&self, path: &'static str) -> Option<Texture> {
        let cache = self.texture_cache.borrow();
        cache.get(path).cloned()
    }

    fn load_new_texture(&self, path: &'static str) -> Result<Texture> {
        let mut cache = self.texture_cache.borrow_mut();
        let mut data_cache = self.data_cache.borrow_mut();
        let id = data_cache.len();
        let texture_path = Path::new(path);
        let texture_data = self.renderer.load_texture(texture_path)?;
        let texture = Texture {
            id: id,
            dims: texture_data.dims(),
        };
        cache.insert(path, texture);
        data_cache.insert(id, texture_data);
        Ok(texture)
    }

    fn draw_and_wrap(&mut self,
                     id: usize,
                     dst: glm::IVec4,
                     src: Option<glm::DVec4>,
                     wrapping_coords: glm::UVec2)
                     -> Result<()> {
        wrap_rects(dst, wrapping_coords)
            .iter()
            .filter_map(|&r| r)
            .map(|r| self.draw_raw(id, Some(r), src))
            .fold(Ok(()), |res, x| if res.is_err() { res } else { x })
    }

    fn draw_raw(&mut self,
                id: usize,
                dst: Option<glm::IVec4>,
                src: Option<glm::DVec4>)
                -> Result<()> {
        let cache = self.data_cache.borrow();
        let texture = cache.get(&id).ok_or("texture not loaded")?;
        let src = src.map(|r| {
            let dims = glm::to_dvec2(texture.dims());
            glm::to_ivec4(glm::dvec4(r.x * dims.x, r.y * dims.y, r.z * dims.x, r.w * dims.y))
        });
        let src = src.map(Self::get_rect);
        let dst = dst.map(Self::get_rect);
        self.renderer.copy(texture, src, dst)
    }

    fn get_rect(rect: glm::IVec4) -> rect::Rect {
        rect::Rect::new(rect.x, rect.y, rect.z as u32, rect.w as u32)
    }
}
