use glm;
use moho::errors::*;
use moho::renderer::Renderer;
use moho::resource_manager::{ResourceManager, Texture};
use sdl2::rect;

pub enum Drawable<'a> {
    Asset(&'a Asset),
    Rectangles(&'a [rect::Rect]),
}

impl<'a> Drawable<'a> {
    pub fn draw<R: Renderer>(&self, renderer: &mut ResourceManager<R>) -> Result<()> {
        match *self {
            Drawable::Asset(ref a) => {
                let max = Some(renderer.output_size()?);
                renderer.draw(a.texture_id, Some(a.dst_rect), a.src_rect, max)
            }
            Drawable::Rectangles(ref r) => renderer.fill_rects(r),
        }
    }
}

#[derive(Clone)]
pub struct Asset {
    pub texture_id: usize,
    pub dst_rect: glm::IVec4,
    pub src_rect: Option<glm::DVec4>,
}

impl Asset {
    pub fn from_texture(texture: &Texture) -> Asset {
        let rect = glm::ivec4(0, 0, texture.dims.x as i32, texture.dims.y as i32);
        Asset::new(texture.id, rect)
    }

    pub fn new(texture_id: usize, dst_rect: glm::IVec4) -> Asset {
        Asset {
            texture_id: texture_id,
            dst_rect: dst_rect,
            src_rect: None,
        }
    }

    pub fn set_center(&mut self, center: glm::IVec2) {
        self.dst_rect.x = center.x - self.dst_rect.z / 2;
        self.dst_rect.y = center.y - self.dst_rect.w / 2;
    }

    pub fn center(&self) -> glm::IVec2 {
        glm::ivec2(self.dst_rect.x + self.dst_rect.z / 2,
                   self.dst_rect.y + self.dst_rect.w / 2)
    }
}
