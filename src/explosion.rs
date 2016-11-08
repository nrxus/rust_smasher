extern crate sdl2;
extern crate glm;

use animation::Animation;
use self::sdl2::render::Renderer;

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

    pub fn draw(&self, renderer: &mut Renderer) -> Result<(), String> {
        self.animation.draw(renderer, self.center)
    }
}
