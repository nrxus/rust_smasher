use glm;
use moho::errors::*;
use moho::resource_manager::{Renderer, ResourceManager, TextureData};

pub enum PlanetKind {
    RED,
    BLUE,
    WHITE,
}

pub struct Drawable<R: Renderer> {
    planet: TextureData<R::Texture>,
    gravity: TextureData<R::Texture>,
    center: glm::IVec2,
}

impl<R: Renderer> Drawable<R> {
    pub fn new(center: glm::IVec2,
               gravity_radius: u32,
               kind: PlanetKind,
               resource_manager: &ResourceManager<R>)
               -> Result<Self> {
        let (planet, mut gravity) = Self::load_textures(kind, resource_manager)?;
        gravity.dims = glm::uvec2(gravity_radius * 2, gravity_radius * 2);
        let drawable = Drawable {
            planet: planet,
            gravity: gravity,
            center: center,
        };

        Ok(drawable)
    }

    pub fn planet_dims(&self) -> glm::UVec2 {
        self.planet.dims
    }

    pub fn draw(&self, renderer: &mut ResourceManager<R>) -> Result<()> {
        renderer.draw_from_center(&self.gravity, self.center, None, None)?;
        renderer.draw_from_center(&self.planet, self.center, None, None)
    }

    fn load_textures(kind: PlanetKind,
                     resource_manager: &ResourceManager<R>)
                     -> Result<(TextureData<R::Texture>, TextureData<R::Texture>)> {
        let (planet, gravity) = match kind {
            PlanetKind::RED => ("resources/red_planet.png", "resources/red_ring.png"),
            PlanetKind::BLUE => ("resources/blue_planet.png", "resources/blue_ring.png"),
            PlanetKind::WHITE => ("resources/white_planet.png", "resources/white_ring.png"),
        };

        Ok((resource_manager.load_texture(planet)?, resource_manager.load_texture(gravity)?))
    }
}
