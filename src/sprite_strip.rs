extern crate glm;
extern crate sdl2;
extern crate moho;

use self::sdl2::rect;

use self::moho::resource_manager::*;

pub struct SpriteStrip<R: Renderer> {
    texture: TextureData<R::Texture>,
    dims: glm::UVec2,
    wrapping_coords: Option<glm::UVec2>,
}

impl<R: Renderer> SpriteStrip<R> {
    pub fn new(texture: TextureData<R::Texture>,
               num_frames: u32,
               wrapping_coords: Option<glm::UVec2>)
               -> Self {
        let dims = glm::uvec2(texture.width / num_frames, texture.height);

        SpriteStrip {
            texture: texture,
            dims: dims,
            wrapping_coords: wrapping_coords,
        }
    }

    pub fn draw(&self,
                renderer: &mut ResourceManager<R>,
                center: glm::IVec2,
                frame_num: u32)
                -> Result<(), String> {
        let src_rect = rect::Rect::new((self.dims.x * frame_num) as i32,
                                       0,
                                       self.dims.x,
                                       self.dims.y);

        let dst_rect = rect::Rect::new(center.x - self.dims.x as i32 / 2,
                                       center.y - self.dims.y as i32 / 2,
                                       self.dims.x,
                                       self.dims.y);

        renderer.draw(&*self.texture.texture,
                      Some(src_rect),
                      Some(dst_rect),
                      self.wrapping_coords)
    }

    pub fn get_dims(&self) -> glm::UVec2 {
        self.dims
    }
}
