extern crate sdl2;
extern crate glm;
extern crate moho;

use animation::Animation;
use self::sdl2::render::Texture;
use self::moho::resource_manager::Renderer;
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

    pub fn draw<I>(&self, renderer: &mut ResourceManager<I>) -> Result<(), String>
        where I: Renderer<Texture = Texture>
    {
        self.animation.draw(renderer, self.center)
    }
}
