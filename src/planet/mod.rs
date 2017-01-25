mod drawable;
mod object;

pub use self::drawable::PlanetKind;
use self::object::Object;
use self::drawable::Drawable;

use std::cmp;

use asset_manager::AssetManager;
use circle::Circle;
use collidable::Collidable;
use shape::Intersect;
use glm;
use moho::errors::*;
use moho::renderer::Renderer;
use moho::resource_manager::ResourceManager;

pub struct Planet {
    object: Object,
    drawable: Drawable,
}

impl Planet {
    pub fn new(center: glm::IVec2,
               strength: f64,
               gravity_radius: f64,
               kind: PlanetKind,
               asset_manager: &AssetManager)
               -> Self {
        let drawable = Drawable::new(center, gravity_radius as u32, kind, asset_manager);
        let dims = drawable.planet_dims();
        let planet_radius = cmp::min(dims.x, dims.y) as f64 / 2.;
        let center = glm::to_dvec2(center);
        let object = Object::new(center, strength, planet_radius, gravity_radius);

        Planet {
            object: object,
            drawable: drawable,
        }
    }

    pub fn pull_vector(&self, point: glm::DVec2, radius: f64) -> glm::DVec2 {
        self.object.pull_vector(point, radius)
    }

    pub fn draw<R: Renderer>(&self, renderer: &mut ResourceManager<R>) -> Result<()> {
        self.drawable.draw(renderer)
    }
}

impl<I: Intersect<Circle>> Collidable<Circle, I> for Planet {
    fn collides(&self, collision: &I) -> bool {
        self.object.collides_with(collision)
    }
}
