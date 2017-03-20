use master_smasher::drawable::{Scene, GameRenderer, Rectifiable};
use master_smasher::shape::{Circle, Intersect, Shape};
use super::world_assets::WorldAssets;
use super::collidable::Collidable;
use super::level_data::{PlanetData, PlanetKind};
use errors::*;

use glm;
use glm::ext::normalize_to;
use moho::renderer::Renderer;
use moho::resource_manager::{ResourceManager, Texture};
use num_traits::Zero;

use std::cmp;
use std::time::Duration;

struct Ring {
    body: Circle,
    strength: f64,
    zoom: f64,
    texture: Texture,
}

impl Ring {
    pub fn new(radius: f64, strength: f64, center: glm::IVec2, texture: Texture) -> Self {
        let body = Circle {
            radius: radius,
            center: glm::to_dvec2(center),
        };
        Ring {
            body: body,
            strength: strength,
            zoom: 1.,
            texture: texture,
        }
    }

    pub fn animate(&mut self, delta: Duration) {
        const K: f64 = 0.166;
        const NANO_IN_SEC: f64 = 1000000000.;

        let animated = self.animated_body();
        let pull = self.pull(animated.radius);
        let time = delta.as_secs() as f64 + delta.subsec_nanos() as f64 / NANO_IN_SEC;
        self.zoom *= 1. / 2_f64.powf(K * pull * time);
    }

    pub fn pull_vector<B: Intersect<Circle> + Shape>(&self, body: &B) -> glm::DVec2 {
        if body.intersects(&self.body) {
            let dist = self.body.center - body.get_center();
            let force = self.pull(glm::length(dist));
            normalize_to(dist, force)
        } else {
            glm::DVec2::zero()
        }
    }

    fn pull(&self, distance: f64) -> f64 {
        self.strength / (distance.powf(0.8))
    }

    fn animated_body(&self) -> Circle {
        Circle {
            radius: self.body.radius * self.zoom,
            center: self.body.center,
        }
    }
}

pub struct Planet {
    body: Circle,
    texture: Texture,
    ring: Option<Ring>,
}

impl Planet {
    pub fn new(data: &PlanetData, textures: &WorldAssets) -> Self {
        let (texture, ring) = Self::load_assets(data, textures);
        let radius = cmp::min(texture.dims.x, texture.dims.y) as f64 / 2.;
        let center = glm::dvec2(data.x as f64, data.y as f64);
        let body = Circle {
            center: center,
            radius: radius,
        };

        Planet {
            body: body,
            texture: texture,
            ring: ring,
        }
    }

    pub fn animate(&mut self, delta: Duration) {
        if let Some(ref mut r) = self.ring {
            if r.body.radius * r.zoom < self.body.radius {
                r.zoom = 1.;
            }
            r.animate(delta)
        }
    }

    //TODO: make generic to any Intersect
    pub fn pull_vector(&self, body: &Circle) -> glm::DVec2 {
        self.ring.as_ref().map_or(glm::DVec2::zero(), |r| r.pull_vector(body))
    }

    fn load_assets(data: &PlanetData, textures: &WorldAssets) -> (Texture, Option<Ring>) {
        let center = glm::ivec2(data.x, data.y);
        let (planet, ring) = match data.kind {
            PlanetKind::RED { ring, strength } => {
                (textures.red_planet, Some((textures.red_ring, ring, strength)))
            }
            PlanetKind::BLUE { ring, strength } => {
                (textures.blue_planet, Some((textures.blue_ring, ring, strength)))
            }
            PlanetKind::WHITE { ring, strength } => {
                (textures.white_planet, Some((textures.white_ring, ring, strength)))
            }
            PlanetKind::DEAD => (textures.dead_planet, None),
        };

        let ring = ring.map(|(t, r, s)| Ring::new(r, s, center, t));

        (planet, ring)
    }
}

impl<I: Intersect<Circle>> Collidable<Circle, I> for Planet {
    fn collides(&self, shape: &I) -> bool {
        shape.intersects(&self.body)
    }
}

impl<R: Renderer> Scene<ResourceManager<R>> for Planet {
    fn show(&self, renderer: &mut ResourceManager<R>) -> Result<()> {
        if let Some(ref r) = self.ring {
            renderer.show(r)?;
        }
        renderer.render(&self.texture, self.body.rectify())
    }
}

impl<R: Renderer> Scene<ResourceManager<R>> for Ring {
    fn show(&self, renderer: &mut ResourceManager<R>) -> Result<()> {
        let dst_rect = self.body.rectify();
        let moving_rect = self.animated_body().rectify();
        renderer.render(&self.texture, dst_rect)?;
        renderer.render(&self.texture, moving_rect)
    }
}
