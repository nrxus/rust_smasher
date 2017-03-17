use errors::*;
use super::Scene;

use glm;
use moho::resource_manager::{ResourceManager, Texture};
use moho::renderer::Renderer;

#[derive(Clone, Copy)]
pub struct Asset {
    pub texture_id: usize,
    pub dst_rect: glm::IVec4,
    pub src_rect: Option<glm::UVec4>,
}

impl Asset {
    pub fn from_texture(texture: &Texture, center: glm::IVec2) -> Asset {
        Asset::centered_on(texture.id, center, texture.dims)
    }

    pub fn scaled_texture(texture: &Texture, center: glm::IVec2, scale: glm::DVec2) -> Asset {
        let dims = glm::to_uvec2(glm::to_dvec2(texture.dims) * scale);
        Asset::centered_on(texture.id, center, dims)
    }

    pub fn centered_on(texture_id: usize, center: glm::IVec2, dims: glm::UVec2) -> Asset {
        let rect = Self::rectify(center, dims);
        Asset::new(texture_id, rect)
    }

    pub fn new(texture_id: usize, dst_rect: glm::IVec4) -> Asset {
        Asset {
            texture_id: texture_id,
            dst_rect: dst_rect,
            src_rect: None,
        }
    }

    pub fn zoom(&mut self, zoom: glm::DVec2) {
        let dims = glm::to_dvec2(self.dims()) * zoom;
        self.dst_rect = Self::rectify(self.center(), glm::to_uvec2(dims));
    }

    pub fn center(&self) -> glm::IVec2 {
        glm::ivec2(self.dst_rect.x + self.dst_rect.z / 2,
                   self.dst_rect.y + self.dst_rect.w / 2)
    }

    pub fn dims(&self) -> glm::UVec2 {
        glm::uvec2(self.dst_rect.z as u32, self.dst_rect.w as u32)
    }

    fn rectify(center: glm::IVec2, dims: glm::UVec2) -> glm::IVec4 {
        let dims = glm::to_ivec2(dims);
        glm::ivec4(center.x - dims.x / 2, center.y - dims.y / 2, dims.x, dims.y)
    }
}

impl<R: Renderer> Scene<ResourceManager<R>> for Asset {
    fn show(&self, renderer: &mut ResourceManager<R>) -> Result<()> {
        renderer.draw(self.texture_id, Some(self.dst_rect), self.src_rect).map_err(Into::into)
    }
}
