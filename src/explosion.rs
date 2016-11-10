extern crate sdl2;
extern crate glm;
extern crate moho;

use animation::Animation;
use self::sdl2::render::Renderer as SdlRenderer;
use self::moho::resource_manager::ResourceManager;

pub struct Explosion {
    animation: Animation,
    center: glm::IVec2,
}

impl Explosion {
    pub fn new(animation: Animation, center: glm::IVec2) -> Self {
        Explosion {
            animation: animation,
            center: center,
        }
    }

    pub fn update(&mut self) -> bool {
        self.animation.update()
    }

    pub fn draw(&self, renderer: &mut ResourceManager<SdlRenderer>) -> Result<(), String> {
        self.animation.draw(renderer, self.center)
    }
}
