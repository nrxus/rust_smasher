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

pub trait Renderer {
    type Texture;
    fn load_texture(&self, path: &Path) -> Result<Self::Texture, String>;
    fn output_size(&self) -> Result<(u32, u32), String>;

    fn clear(&mut self);
    fn present(&mut self);
    fn copy(&mut self,
            texture: Rc<Self::Texture>,
            src: Option<rect::Rect>,
            dst: Option<rect::Rect>)
            -> Result<(), String>;
}

struct TextureData<T> {
    texture: T,
    size: glm::IVec2,
}

impl<'a> Renderer for SdlRenderer<'a> {
    type Texture = SdlTexture;

    fn load_texture(&self, path: &Path) -> Result<SdlTexture, String> {
        LoadTexture::load_texture(self, path)
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

pub struct ResourceManager<'a, I: Renderer> {
    texture_cache: RefCell<HashMap<&'a str, Rc<I::Texture>>>,
    renderer: I,
}

impl<'a, I: Renderer> ResourceManager<'a, I> {
    pub fn new(renderer: I) -> Self {
        ResourceManager {
            texture_cache: RefCell::new(HashMap::new()),
            renderer: renderer,
        }
    }

    pub fn load_texture(&self, path: &'a str) -> Result<Rc<I::Texture>, String> {
        {
            let cache = self.texture_cache.borrow();
            let image = cache.get(path);
            if let Some(x) = image {
                return Ok(x.clone());
            }
        }
        let mut cache = self.texture_cache.borrow_mut();
        let image_path = Path::new(path);
        let image = Rc::new(try!(self.renderer.load_texture(image_path)));
        cache.insert(path, image.clone());
        Ok(image.clone())
    }

    pub fn clear(&mut self) {
        self.renderer.clear();
    }

    pub fn present(&mut self) {
        self.renderer.present();
    }
}
