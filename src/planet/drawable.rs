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
        let mut planet = Asset::from_texture(&planet);
        planet.set_center(center);
        let rect = glm::ivec4(0, 0, gravity_radius as i32 * 2, gravity_radius as i32 * 2);
        let mut gravity = Asset::new(gravity.id, rect);
        gravity.set_center(center);
        let drawable = Drawable {
            planet: planet,
            gravity: gravity,
        };

        Ok(drawable)
    }

    pub fn planet_dims(&self) -> glm::UVec2 {
        glm::uvec2(self.planet.dst_rect.z as u32, self.planet.dst_rect.w as u32)
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
        asset.draw(None, None, renderer)
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
