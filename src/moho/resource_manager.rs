use std::path::Path;
use std::collections::HashMap;
use std::cell::RefCell;

use glm;
use sdl2::rect;

use renderer::{Renderer, TextureData};
use window_wrapper::*;
use errors::*;

#[derive(Copy,Clone)]
pub struct Texture {
    pub id: usize,
    pub dims: glm::UVec2,
}

pub struct ResourceManager<R: Renderer> {
    texture_cache: RefCell<HashMap<&'static str, Texture>>,
    data_cache: RefCell<HashMap<usize, TextureData<R>>>,
    renderer: R,
}

impl<R: Renderer> ResourceManager<R> {
    pub fn new(renderer: R) -> Self {
        ResourceManager {
            texture_cache: RefCell::new(HashMap::new()),
            data_cache: RefCell::new(HashMap::new()),
            renderer: renderer,
        }
    }

    pub fn load_texture(&self, path: &'static str) -> Result<Texture> {
        match self.load_cached_texture(path) {
            Some(texture) => Ok(texture),
            None => self.load_new_texture(path),
        }
    }

    pub fn draw_from_center(&mut self,
                            texture: &Texture,
                            center: glm::IVec2,
                            src: Option<glm::DVec4>,
                            wrapping_coords: Option<glm::UVec2>)
                            -> Result<()> {
        let width = texture.dims.x as i32;
        let height = texture.dims.y as i32;
        let dst = glm::ivec4(center.x - width / 2, center.y - height / 2, width, height);
        self.draw(texture.id, Some(dst), src, wrapping_coords)
    }

    pub fn draw(&mut self,
                id: usize,
                dst: Option<glm::IVec4>,
                src: Option<glm::DVec4>,
                wrapping_coords: Option<glm::UVec2>)
                -> Result<()> {
        match (dst, wrapping_coords) {
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
        match cache.get(path) {
            Some(texture) => Some(*texture),
            None => None,
        }
    }

    fn load_new_texture(&self, path: &'static str) -> Result<Texture> {
        let mut cache = self.texture_cache.borrow_mut();
        let mut data_cache = self.data_cache.borrow_mut();
        let id = data_cache.len();
        let texture_path = Path::new(path);
        let texture_data = self.renderer.load_texture(texture_path)?;
        let texture = Texture {
            id: id,
            dims: texture_data.dims,
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
            .fold(Ok(()), |res, x| { if res.is_err() { res } else { x } })
    }

    fn draw_raw(&mut self,
                id: usize,
                dst: Option<glm::IVec4>,
                src: Option<glm::DVec4>)
                -> Result<()> {
        let cache = self.data_cache.borrow();
        let data = cache.get(&id).ok_or("texture not loaded")?;
        let src = match src {
            None => None,
            Some(r) => {
                let dims = glm::to_dvec2(data.dims);
                let rect = glm::dvec4(r.x * dims.x, r.y * dims.y, r.z * dims.x, r.w * dims.y);
                Some(Self::get_rect(glm::to_ivec4(rect)))
            }
        };
        let dst = match dst {
            None => None,
            Some(r) => Some(Self::get_rect(r)),
        };
        self.renderer.copy(&data.texture, src, dst)
    }

    fn get_rect(rect: glm::IVec4) -> rect::Rect {
        rect::Rect::new(rect.x, rect.y, rect.z as u32, rect.w as u32)
    }
}
