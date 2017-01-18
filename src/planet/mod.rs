mod drawable;
mod object;

pub use self::drawable::PlanetKind;
use self::object::Object;
use self::drawable::Drawable;

use std::cmp;

use circle::Circle;
use shape::Intersect;
use glm;
use moho::errors::*;
use moho::resource_manager::{Renderer, ResourceManager};

pub struct Planet<R: Renderer> {
    object: Object,
    drawable: Drawable<R>,
}

impl<R: Renderer> Planet<R> {
    pub fn new(center: glm::IVec2,
               strength: f64,
               gravity_radius: f64,
               kind: PlanetKind,
               resource_manager: &mut ResourceManager<R>)
               -> Result<Self> {
        let drawable = Drawable::new(center, gravity_radius as u32, kind, resource_manager)?;
        let dims = drawable.planet_dims();
        let planet_radius = cmp::min(dims.x, dims.y) as f64 / 2.;
        let center = glm::to_dvec2(center);
        let object = Object::new(center, strength, planet_radius, gravity_radius);
        let planet = Planet {
            object: object,
            drawable: drawable,
        };

        Ok(planet)
    }

    pub fn pull_vector(&self, point: glm::DVec2, radius: f64) -> glm::DVec2 {
        self.object.pull_vector(point, radius)
    }

    pub fn collides_with<S: Intersect<Circle>>(&self, shape: &S) -> bool {
        self.object.collides_with(shape)
    }

    pub fn draw(&self, renderer: &mut ResourceManager<R>) -> Result<()> {
        self.drawable.draw(renderer)
    }
}
