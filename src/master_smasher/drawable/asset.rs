use glm;
use moho::resource_manager::Texture;

#[derive(Clone, Copy)]
pub struct Asset {
    pub texture: Texture,
    pub dst_rect: glm::IVec4,
}

impl Asset {
    pub fn from_texture(texture: Texture, center: glm::IVec2) -> Asset {
        let dims = glm::to_ivec2(texture.dims);
        let rect = glm::ivec4(center.x - dims.x / 2, center.y - dims.y / 2, dims.x, dims.y);
        Asset::new(texture, rect)
    }

    pub fn new(texture: Texture, dst_rect: glm::IVec4) -> Asset {
        Asset {
            texture: texture,
            dst_rect: dst_rect,
        }
    }

    pub fn dims(&self) -> glm::UVec2 {
        glm::uvec2(self.dst_rect.z as u32, self.dst_rect.w as u32)
    }
}
