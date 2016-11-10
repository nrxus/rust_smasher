extern crate glm;
extern crate sdl2;
extern crate moho;

use std::rc::Rc;
use self::sdl2::rect;
use self::moho::resource_manager::ResourceManager;
use self::moho::resource_manager::Renderer;
use self::sdl2::render::Texture;
use self::moho::window_wrapper::*;

pub struct SpriteStrip {
    texture: Rc<Texture>,
    dims: glm::UVec2,
    wrapping_coords: Option<glm::UVec2>,
}

impl SpriteStrip {
    pub fn new(texture: Rc<Texture>, num_frames: u32, wrapping_coords: Option<glm::UVec2>) -> Self {
        let query = texture.query();
        let dims = glm::uvec2(query.width / num_frames, query.height);

        SpriteStrip {
            texture: texture,
            dims: dims,
            wrapping_coords: wrapping_coords,
        }
    }

    pub fn draw<I>(&self,
                   renderer: &mut ResourceManager<I>,
                   center: glm::IVec2,
                   frame_num: u32)
                   -> Result<(), String>
        where I: Renderer<Texture = Texture>
    {

        let source_rect = rect::Rect::new((self.dims.x * frame_num) as i32,
                                          0,
                                          self.dims.x,
                                          self.dims.y);

        match self.wrapping_coords {
            Some(coords) => {
                let center = glm::uvec2(center.x as u32, center.y as u32);
                get_wrapped_centers(center, self.dims, coords)
                    .iter()
                    .filter_map(|&c| c)
                    .map(|c| {
                        rect::Rect::new(c.x - self.dims.x as i32 / 2,
                                        c.y - self.dims.y as i32 / 2,
                                        self.dims.x,
                                        self.dims.y)
                    })
                    .map(|r| renderer.draw(self.texture.clone(), Some(source_rect), Some(r)))
                    .fold(Ok(()), |res, x| { if res.is_err() { res } else { x } })
            }
            None => {
                let rect = rect::Rect::new(center.x - self.dims.x as i32 / 2,
                                           center.y - self.dims.y as i32 / 2,
                                           self.dims.x,
                                           self.dims.y);
                renderer.draw(self.texture.clone(), Some(source_rect), Some(rect))
            }
        }
    }

    pub fn get_dims(&self) -> glm::UVec2 {
        self.dims
    }
}
