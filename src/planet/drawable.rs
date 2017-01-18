use glm;
use moho::errors::*;
use moho::resource_manager::{Renderer, ResourceManager, TextureData};

use std::rc::Rc;

pub enum PlanetKind {
    RED,
    BLUE,
    WHITE,
}

pub struct Drawable<R: Renderer> {
    planet: Rc<R::Texture>,
    gravity: Rc<R::Texture>,
    planet_dims: glm::UVec2,
    gravity_dims: glm::UVec2,
    center: glm::IVec2,
}

impl<R: Renderer> Drawable<R> {
    pub fn new(center: glm::IVec2,
               gravity_radius: u32,
               kind: PlanetKind,
               resource_manager: &mut ResourceManager<R>)
               -> Result<Self> {
        let (planet, gravity) = Self::load_textures(kind, resource_manager)?;
        let gravity_dims = glm::uvec2(gravity_radius * 2, gravity_radius * 2);
        let drawable = Drawable {
            planet: planet.texture,
            gravity: gravity.texture,
            planet_dims: planet.dims,
            gravity_dims: gravity_dims,
            center: center,
        };

        Ok(drawable)
    }

    pub fn planet_dims(&self) -> glm::UVec2 {
        self.planet_dims
    }

    pub fn draw(&self, renderer: &mut ResourceManager<R>) -> Result<()> {
        renderer.draw_from_center(&*self.gravity, None, self.center, self.gravity_dims, None)?;
        renderer.draw_from_center(&*self.planet, None, self.center, self.planet_dims, None)
    }

    fn load_textures(kind: PlanetKind,
                     resource_manager: &mut ResourceManager<R>)
                     -> Result<(TextureData<R::Texture>, TextureData<R::Texture>)> {
        let (planet, gravity) = match kind {
            PlanetKind::RED => ("resources/red_planet.png", "resources/red_ring.png"),
            PlanetKind::BLUE => ("resources/blue_planet.png", "resources/blue_ring.png"),
            PlanetKind::WHITE => ("resources/white_planet.png", "resources/white_ring.png"),
        };

        Ok((resource_manager.load_texture(planet)?, resource_manager.load_texture(gravity)?))
    }
}
