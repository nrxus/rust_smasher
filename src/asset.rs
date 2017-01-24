use glm;
use moho::errors::*;
use moho::renderer::Renderer;
use moho::resource_manager::{ResourceManager, Texture};

pub struct Asset {
    texture_id: usize,
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

    pub fn draw<R>(&self,
                   center: glm::IVec2,
                   src: Option<glm::DVec4>,
                   wrapping: Option<glm::UVec2>,
                   renderer: &mut ResourceManager<R>)
                   -> Result<()>
        where R: Renderer
    {
        let dst = Some(self.dst_rect(center));
        renderer.draw(self.texture_id, dst, src, wrapping)
    }

    fn dst_rect(&self, center: glm::IVec2) -> glm::IVec4 {
        let dimensions = glm::to_ivec2(self.dimensions);
        (center - dimensions / 2).extend(dimensions.x).extend(dimensions.y)
    }
}
