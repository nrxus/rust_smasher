use errors::*;
use super::{GameRenderer, Scene};

use glm;
use moho::resource_manager::{ResourceManager, Texture};
use moho::renderer::Renderer;

#[derive(Clone, Copy)]
pub struct Asset {
    pub texture: Texture,
    pub dst_rect: glm::IVec4,
}

impl Asset {
    pub fn from_texture(texture: Texture, center: glm::IVec2) -> Asset {
        Asset::centered_on(texture, center, texture.dims)
    }

    pub fn centered_on(texture: Texture, center: glm::IVec2, dims: glm::UVec2) -> Asset {
        let rect = Self::rectify(center, dims);
        Asset::new(texture, rect)
    }

    pub fn new(texture: Texture, dst_rect: glm::IVec4) -> Asset {
        Asset {
            texture: texture,
            dst_rect: dst_rect,
        }
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
        renderer.render(&self.texture, self.dst_rect)
    }
}
