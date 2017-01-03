extern crate sdl2;
extern crate glm;
extern crate moho;

use std::rc::Rc;

use animation::Animation;
use self::moho::resource_manager::Renderer;
use self::moho::resource_manager::ResourceManager;

pub struct Explosion<R: Renderer> {
    center: glm::IVec2,
    dims: glm::UVec2,
    texture: Rc<R::Texture>,
    animation: Animation,
}

impl<R: Renderer> Explosion<R> {
    pub fn new(center: glm::IVec2,
               dims: glm::UVec2,
               animation: Animation,
               texture: Rc<R::Texture>)
               -> Self {
        Explosion {
            center: center,
            dims: dims,
            texture: texture,
            animation: animation,
        }
    }

    pub fn update(&mut self) -> bool {
        self.animation.update()
    }

    pub fn draw(&self, renderer: &mut ResourceManager<R>) -> Result<(), String> {
        let src_rect = Some(self.animation.src_rect());
        renderer.draw_from_center(&*self.texture, src_rect, self.center, self.dims, None)
    }
}
