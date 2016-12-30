extern crate glm;
extern crate sdl2;
extern crate moho;

use self::sdl2::rect;

use self::moho::resource_manager::*;

pub struct SpriteStrip<R: Renderer> {
    texture: TextureData<R::Texture>,
    wrapping_coords: Option<glm::UVec2>,
}

impl<R: Renderer> SpriteStrip<R> {
    pub fn new(texture: TextureData<R::Texture>,
               wrapping_coords: Option<glm::UVec2>)
               -> Self {
        SpriteStrip {
            texture: texture,
            wrapping_coords: wrapping_coords,
        }
    }

    pub fn draw(&self,
                center: glm::IVec2,
                dims: glm::UVec2,
                frame_num: u32,
                renderer: &mut ResourceManager<R>)
                -> Result<(), String> {
        let src_rect = rect::Rect::new((dims.x * frame_num) as i32, 0, dims.x, dims.y);

        let dst_rect = rect::Rect::from_center((center.x, center.y), dims.x, dims.y);

        renderer.draw(&*self.texture.texture,
                      Some(src_rect),
                      Some(dst_rect),
                      self.wrapping_coords)
    }
}
