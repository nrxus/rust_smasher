use std::path::Path;
use std::collections::HashMap;
use std::cell::RefCell;

use glm;
use sdl2::rect;
use sdl2::render::Renderer as SdlRenderer;
use sdl2::render::Texture as SdlTexture;
use sdl2::image::LoadTexture;

use window_wrapper::*;
use errors::*;

#[derive(Copy,Clone)]
pub struct Texture {
    pub id: usize,
    pub dims: glm::UVec2,
}

pub struct TextureData<R: Renderer> {
    pub texture: R::Texture,
    pub dims: glm::UVec2,
}

pub trait Renderer {
    type Texture;

    fn load_texture(&self, path: &Path) -> Result<TextureData<Self>> where Self: Sized;
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

    fn load_texture(&self, path: &Path) -> Result<TextureData<Self>> {
        let texture = LoadTexture::load_texture(self, path)?;
        let query = texture.query();
        Ok(TextureData {
            texture: texture,
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
    texture_cache: RefCell<HashMap<&'static str, Texture>>,
    id_cache: RefCell<HashMap<usize, TextureData<R>>>,
    renderer: R,
}

impl<R: Renderer> ResourceManager<R> {
    pub fn new(renderer: R) -> Self {
        ResourceManager {
            texture_cache: RefCell::new(HashMap::new()),
            id_cache: RefCell::new(HashMap::new()),
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
        let mut id_cache = self.id_cache.borrow_mut();
        let id = id_cache.len();
        let texture_path = Path::new(path);
        let texture_data = self.renderer.load_texture(texture_path)?;
        let texture = Texture {
            id: id,
            dims: texture_data.dims,
        };
        cache.insert(path, texture);
        id_cache.insert(id, texture_data);
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
        let cache = self.id_cache.borrow();
        let data = cache.get(&id).ok_or("texture not loaded")?;
        let src = match src {
            None => None,
            Some(r) => {
                Some(rect::Rect::new((r.x * data.dims.x as f64) as i32,
                                     (r.y * data.dims.y as f64) as i32,
                                     (r.z * data.dims.x as f64) as u32,
                                     (r.w * data.dims.y as f64) as u32))
            }
        };
        self.renderer.copy(&data.texture, src, Self::get_rect(dst))
    }

    fn get_rect(rect: Option<glm::IVec4>) -> Option<rect::Rect> {
        match rect {
            Some(r) => Some(rect::Rect::new(r.x, r.y, r.z as u32, r.w as u32)),
            None => None,
        }
    }
}
