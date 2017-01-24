use glm;
use moho::resource_manager::Texture;

pub struct Asset {
    pub texture_id: usize,
    pub dimensions: glm::UVec2,
}

impl Asset {
    pub fn from_texture(texture: &Texture) -> Asset {
        Asset::new(texture.id, texture.dims)
    }

    pub fn new(texture_id: usize, dimensions: glm::UVec2) -> Asset {
        Asset {
            texture_id: texture_id,
            dimensions: dimensions,
        }
    }

    pub fn dst_rect(&self, center: glm::IVec2) -> glm::IVec4 {
        let dimensions = glm::to_ivec2(self.dimensions);
        (center - dimensions / 2).extend(dimensions.x).extend(dimensions.y)
    }
}
