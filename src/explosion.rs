extern crate sdl2;
extern crate glm;
extern crate moho;

use animation::Animation;
use self::moho::resource_manager::Renderer;
use self::moho::resource_manager::ResourceManager;

pub struct Explosion<R: Renderer> {
    animation: Animation<R>,
    center: glm::IVec2,
}

impl<R: Renderer> Explosion<R> {
    pub fn new(animation: Animation<R>, center: glm::IVec2) -> Self {
        Explosion {
            animation: animation,
            center: center,
        }
    }

    pub fn update(&mut self) -> bool {
        self.animation.update()
    }

    pub fn draw(&self, renderer: &mut ResourceManager<R>) -> Result<(), String> {
        self.animation.draw(renderer, self.center)
    }
}
