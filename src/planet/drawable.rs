use asset::Asset;

use glm::{self, GenNum};
use moho::resource_manager::{ResourceManager, Texture};
use moho::renderer::Renderer;
use moho::errors::*;

pub enum PlanetKind {
    RED,
    BLUE,
    WHITE,
}

pub struct Drawable {
    planet: Asset,
    gravity: Asset,
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
        let (planet, gravity) = Self::load_textures(kind, resource_manager)?;
        let planet = Asset::from_texture(&planet);
        let gravity = Asset::new(gravity.id, glm::UVec2::from_s(gravity_radius * 2));
        let drawable = Drawable {
            planet: planet,
            gravity: gravity,
            center: center,
        };

        Ok(drawable)
    }

    pub fn planet_dims(&self) -> glm::UVec2 {
        self.planet.dimensions
    }

    pub fn draw<R>(&self, renderer: &mut ResourceManager<R>) -> Result<()>
        where R: Renderer
    {
        self.draw_at_center(&self.gravity, renderer)?;
        self.draw_at_center(&self.planet, renderer)
    }

    fn draw_at_center<R>(&self, asset: &Asset, renderer: &mut ResourceManager<R>) -> Result<()>
        where R: Renderer
    {
        asset.draw(self.center, None, None, renderer)
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
