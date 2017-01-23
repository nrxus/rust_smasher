use glm;
use moho::resource_manager::{ResourceManager, Texture};
use moho::renderer::Renderer;
use moho::errors::*;

pub enum PlanetKind {
    RED,
    BLUE,
    WHITE,
}

pub struct Drawable {
    planet: Texture,
    gravity: Texture,
    center: glm::IVec2,
}

impl Drawable {
    pub fn new<R>(center: glm::IVec2,
                  gravity_radius: u32,
                  kind: PlanetKind,
                  resource_manager: &ResourceManager<R>)
                  -> Result<Self>
        where R: Renderer
    {
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

    pub fn draw<R: Renderer>(&self, renderer: &mut ResourceManager<R>) -> Result<()> {
        renderer.draw_from_center(&self.gravity, self.center, None, None)?;
        renderer.draw_from_center(&self.planet, self.center, None, None)
    }

    fn load_textures<R>(kind: PlanetKind,
                        resource_manager: &ResourceManager<R>)
                        -> Result<(Texture, Texture)>
        where R: Renderer
    {
        let (planet, gravity) = match kind {
            PlanetKind::RED => ("resources/red_planet.png", "resources/red_ring.png"),
            PlanetKind::BLUE => ("resources/blue_planet.png", "resources/blue_ring.png"),
            PlanetKind::WHITE => ("resources/white_planet.png", "resources/white_ring.png"),
        };

        Ok((resource_manager.load_texture(planet)?, resource_manager.load_texture(gravity)?))
    }
}
