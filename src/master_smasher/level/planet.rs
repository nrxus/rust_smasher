use master_smasher::drawable::{Asset, AssetManager, Drawable, TextureAsset};
use master_smasher::shape::{Circle, Intersect};
use super::collidable::Collidable;
use super::level_data::{PlanetData, PlanetKind};

use glm;
use glm::GenNum;
use glm::ext::normalize_to;
use num_traits::Zero;

use std::cmp;

struct Ring {
    radius: f64,
    strength: f64,
    asset: Asset,
}

pub struct Planet {
    body: Circle,
    asset: Asset,
    ring: Option<Ring>,
}

impl Planet {
    pub fn new(data: &PlanetData, asset_manager: &AssetManager) -> Self {
        let (asset, ring) = Self::load_assets(data, asset_manager);
        let dims = asset.dims();
        let radius = cmp::min(dims.x, dims.y) as f64 / 2.;
        let center = glm::dvec2(data.x as f64, data.y as f64);
        let body = Circle {
            center: center,
            radius: radius,
        };

        Planet {
            body: body,
            asset: asset,
            ring: ring,
        }
    }

    pub fn pull_vector(&self, point: glm::DVec2, radius: f64) -> glm::DVec2 {
        match self.ring {
            Some(ref r) => {
                let dist = self.body.center - point;
                let len = glm::length(dist);
                if len > (r.radius + radius) {
                    glm::DVec2::zero()
                } else {
                    let force = r.strength / (len.powf(0.8));
                    normalize_to(dist, force)
                }
            }
            None => glm::DVec2::zero(),
        }
    }

    pub fn drawables(&self) -> Vec<Drawable> {
        let mut drawables = vec![];
        match self.ring {
            Some(ref r) => drawables.push(Drawable::Asset(&r.asset)),
            None => {}
        }
        drawables.push(Drawable::Asset(&self.asset));
        drawables
    }

    fn load_assets(data: &PlanetData, asset_manager: &AssetManager) -> (Asset, Option<Ring>) {
        let center = glm::ivec2(data.x, data.y);
        let (planet, ring) = match data.kind {
            PlanetKind::RED { ring, strength } => {
                let mut asset = asset_manager.get_asset(TextureAsset::RedRing, center);
                asset.resize(glm::UVec2::from_s((ring * 2.) as u32));
                let ring = Some(Ring {
                    radius: ring,
                    strength: strength,
                    asset: asset,
                });
                (TextureAsset::RedPlanet, ring)
            }
            PlanetKind::BLUE { ring, strength } => {
                let mut asset = asset_manager.get_asset(TextureAsset::BlueRing, center);
                asset.resize(glm::UVec2::from_s((ring * 2.) as u32));
                let ring = Some(Ring {
                    radius: ring,
                    strength: strength,
                    asset: asset,
                });
                (TextureAsset::BluePlanet, ring)
            }
            PlanetKind::WHITE { ring, strength } => {
                let mut asset = asset_manager.get_asset(TextureAsset::WhiteRing, center);
                asset.resize(glm::UVec2::from_s((ring * 2.) as u32));
                let ring = Some(Ring {
                    radius: ring,
                    strength: strength,
                    asset: asset,
                });
                (TextureAsset::WhitePlanet, ring)
            }
            PlanetKind::DEAD => (TextureAsset::DeadPlanet, None),
        };
        let planet = asset_manager.get_asset(planet, center);

        (planet, ring)
    }
}

impl<I: Intersect<Circle>> Collidable<Circle, I> for Planet {
    fn collides(&self, shape: &I) -> bool {
        shape.intersects(&self.body)
    }
}
