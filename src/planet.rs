extern crate sdl2;
extern crate glm;
extern crate moho;

use std::cmp;
use std::rc::Rc;

use self::sdl2::render::Texture;
use self::sdl2::render::Renderer as SdlRenderer;

use self::moho::resource_manager::ResourceManager;

use circle::Circle;
use sprite_strip::SpriteStrip;

pub struct Planet {
    sprite: SpriteStrip,
    center: glm::Vector2<f64>,
}

impl Planet {
    pub fn new(texture: Rc<Texture>, center: glm::IVec2) -> Self {
        let center = glm::dvec2(center.x as f64, center.y as f64);
        let sprite = SpriteStrip::new(texture, 1, None);

        Planet {
            sprite: sprite,
            center: center,
        }
    }

    pub fn draw(&self, renderer: &mut ResourceManager<SdlRenderer>) -> Result<(), String> {
        let center = glm::ivec2(self.center.x as i32, self.center.y as i32);
        self.sprite.draw(renderer, center, 0)
    }

    pub fn collision_body(&self) -> Circle {
        let dims = self.sprite.get_dims();
        let diameter = cmp::min(dims.x, dims.y) as f64;

        Circle {
            center: self.center,
            radius: diameter / 2.,
        }
    }
}
