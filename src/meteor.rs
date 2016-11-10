extern crate glm;
extern crate sdl2;
extern crate moho;

use std::cmp;
use std::rc::Rc;

use self::sdl2::render::Texture;

use self::moho::resource_manager::ResourceManager;
use self::moho::resource_manager::Renderer;

use circle::Circle;
use sprite_strip::SpriteStrip;

pub struct Meteor {
    sprite: SpriteStrip,
    center: glm::DVec2,
    max_coords: glm::UVec2,
    velocity: glm::DVec2,
    launched: bool,
}

impl Meteor {
    pub fn new(texture: Rc<Texture>, center: glm::IVec2, max_coords: glm::Vector2<u32>) -> Self {
        let center = glm::dvec2(center.x as f64, center.y as f64);
        let sprite = SpriteStrip::new(texture, 1, Some(max_coords));

        Meteor {
            sprite: sprite,
            center: center,
            max_coords: max_coords,
            velocity: glm::dvec2(0., 0.),
            launched: false,
        }
    }

    pub fn restart_at(&mut self, center: glm::IVec2) {
        self.center = glm::dvec2(center.x as f64, center.y as f64);
        self.launched = false;
        self.velocity = glm::dvec2(0., 0.);
    }

    pub fn launch(&mut self, target: glm::Vector2<i32>) {
        const FACTOR: f64 = 85.;
        let offset = glm::ivec2(target.x - self.center.x as i32,
                                target.y - self.center.y as i32);
        self.velocity = glm::dvec2(offset.x as f64 / FACTOR, offset.y as f64 / FACTOR);
        self.launched = true;
    }

    pub fn update(&mut self) {
        self.center.y += self.velocity.y;
        self.center.x += self.velocity.x;

        let max_height = self.max_coords.y as f64;
        let max_width = self.max_coords.x as f64;

        self.center.y = (self.center.y + max_height) % max_height;
        self.center.x = (self.center.x + max_width) % max_width;
    }

    pub fn is_launched(&self) -> bool {
        self.launched
    }

    pub fn draw<I>(&self, renderer: &mut ResourceManager<I>) -> Result<(), String>
        where I: Renderer<Texture = Texture>
    {
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
