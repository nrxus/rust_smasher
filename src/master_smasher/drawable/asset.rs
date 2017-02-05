use glm;
use moho::resource_manager::Texture;

#[derive(Clone)]
pub struct Asset {
    pub texture_id: usize,
    pub dst_rect: glm::IVec4,
    pub src_rect: Option<glm::DVec4>,
}

impl Asset {
    pub fn from_texture(texture: &Texture, center: glm::IVec2) -> Asset {
        Asset::centered_on(texture.id, center, texture.dims)
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

    pub fn center_on(&mut self, center: glm::IVec2) {
        self.dst_rect = Self::rectify(center, self.dims());
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
