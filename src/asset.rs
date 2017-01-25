use glm;
use moho::errors::*;
use moho::renderer::Renderer;
use moho::resource_manager::{ResourceManager, Texture};

pub struct Asset {
    texture_id: usize,
    pub dst_rect: glm::IVec4,
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
        }
    }

    pub fn set_center(&mut self, center: glm::IVec2) {
        self.dst_rect.x = center.x - self.dst_rect.z / 2;
        self.dst_rect.y = center.y - self.dst_rect.w / 2;
    }

    pub fn draw<R>(&self,
                   src: Option<glm::DVec4>,
                   wrapping: Option<glm::UVec2>,
                   renderer: &mut ResourceManager<R>)
                   -> Result<()>
        where R: Renderer
    {
        renderer.draw(self.texture_id, Some(self.dst_rect), src, wrapping)
    }
}
