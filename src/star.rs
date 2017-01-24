use animation::Animation;
use asset::Asset;
use collidable::Collidable;
use rectangle::Rectangle;
use shape::Intersect;

use glm;
use moho::errors::*;
use moho::frame_animator::FrameAnimator;
use moho::renderer::Renderer;
use moho::resource_manager::ResourceManager;
use moho::tile_sheet::TileSheet;

use std::time::Duration;

pub enum State {
    INACTIVE,
    ACTIVE,
    EXPLODED,
}

pub struct Star {
    state: State,
    body: Rectangle,
    asset: Asset,
    explosion_asset: Asset,
    animation: Animation,
    explosion_animation: Animation,
}

impl Star {
    pub fn new<R: Renderer>(center: glm::IVec2,
                            resource_manager: &ResourceManager<R>)
                            -> Result<Self> {
        let texture = resource_manager.load_texture("resources/star.png")?;
        let explosion_texture = resource_manager.load_texture("resources/explosion_small.png")?;

        let star_dims = glm::uvec2(texture.dims.x / 2, texture.dims.y);
        let asset = Asset::new(texture.id, star_dims);
        let star_sheet = TileSheet::new(glm::uvec2(2, 1));
        let star_duration = Duration::from_millis(150);
        let star_animator = FrameAnimator::new(2, star_duration, true);
        let animation = Animation::new(star_sheet, star_animator);

        let explosion_dims = glm::uvec2(explosion_texture.dims.x / 10, explosion_texture.dims.y);
        let explosion_asset = Asset::new(explosion_texture.id, explosion_dims);
        let explosion_sheet = TileSheet::new(glm::uvec2(10, 1));
        let explosion_duration = Duration::from_millis(100);
        let explosion_animator = FrameAnimator::new(10, explosion_duration, false);
        let explosion_animation = Animation::new(explosion_sheet, explosion_animator);

        let body = Rectangle {
            center: glm::to_dvec2(center),
            dims: glm::to_dvec2(texture.dims),
        };

        let star = Star {
            state: State::ACTIVE,
            body: body,
            asset: asset,
            explosion_asset: explosion_asset,
            animation: animation,
            explosion_animation: explosion_animation,
        };

        Ok(star)
    }

    pub fn explode(&mut self) {
        self.state = State::EXPLODED;
    }

    pub fn update(&mut self) {
        match self.state {
            State::INACTIVE => {}
            State::ACTIVE => self.animation.update(),
            State::EXPLODED => {
                self.explosion_animation.update();
                if !self.explosion_animation.is_active() {
                    self.state = State::INACTIVE;
                }
            }
        }
    }

    pub fn draw<R: Renderer>(&self, renderer: &mut ResourceManager<R>) -> Result<()> {
        match self.state {
            State::INACTIVE => Ok(()),
            State::ACTIVE => {
                let src_rect = self.animation.src_rect();
                self.draw_on_center(&self.asset, src_rect, renderer)
            }
            State::EXPLODED => {
                let src_rect = self.explosion_animation.src_rect();
                self.draw_on_center(&self.explosion_asset, src_rect, renderer)
            }
        }
    }

    fn draw_on_center<R: Renderer>(&self,
                                   asset: &Asset,
                                   src_rect: glm::DVec4,
                                   renderer: &mut ResourceManager<R>)
                                   -> Result<()> {
        let rect = asset.dst_rect(glm::to_ivec2(self.body.center));
        renderer.draw(asset.texture_id, Some(rect), Some(src_rect), None)
    }
}

impl<I: Intersect<Rectangle>> Collidable<Rectangle, I> for Star {
    fn collides(&self, collision: &I) -> bool {
        collision.intersects(&self.body)
    }
}
