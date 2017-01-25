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
    animation: Animation,
    explosion: Animation,
}

impl Star {
    pub fn new<R: Renderer>(center: glm::IVec2,
                            resource_manager: &ResourceManager<R>)
                            -> Result<Self> {
        let texture = resource_manager.load_texture("resources/star.png")?;
        let explosion_texture = resource_manager.load_texture("resources/explosion_small.png")?;

        let star_dims = glm::ivec2(texture.dims.x as i32 / 2, texture.dims.y as i32);
        let mut asset = Asset::new(texture.id, glm::ivec4(0, 0, star_dims.x, star_dims.y));
        asset.set_center(center);
        let star_sheet = TileSheet::new(glm::uvec2(2, 1));
        let star_duration = Duration::from_millis(150);
        let star_animator = FrameAnimator::new(2, star_duration, true);
        let animation = Animation::new(asset, star_sheet, star_animator);

        let explosion_rect = glm::ivec4(0,
                                        0,
                                        explosion_texture.dims.x as i32 / 10,
                                        explosion_texture.dims.y as i32);
        let mut explosion_asset = Asset::new(explosion_texture.id, explosion_rect);
        explosion_asset.set_center(center);
        let explosion_sheet = TileSheet::new(glm::uvec2(10, 1));
        let explosion_duration = Duration::from_millis(100);
        let explosion_animator = FrameAnimator::new(10, explosion_duration, false);
        let explosion = Animation::new(explosion_asset, explosion_sheet, explosion_animator);

        let body = Rectangle {
            center: glm::to_dvec2(center),
            dims: glm::to_dvec2(star_dims),
        };

        let star = Star {
            state: State::ACTIVE,
            body: body,
            explosion: explosion,
            animation: animation,
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
                self.explosion.update();
                if !self.explosion.is_active() {
                    self.state = State::INACTIVE;
                }
            }
        }
    }

    pub fn draw<R: Renderer>(&self, renderer: &mut ResourceManager<R>) -> Result<()> {
        match self.state {
            State::INACTIVE => Ok(()),
            State::ACTIVE => self.draw_on_center(&self.animation, renderer),
            State::EXPLODED => self.draw_on_center(&self.explosion, renderer),
        }
    }

    fn draw_on_center<R>(&self,
                         animation: &Animation,
                         renderer: &mut ResourceManager<R>)
                         -> Result<()>
        where R: Renderer
    {
        animation.draw(None, renderer)
    }
}

impl<I: Intersect<Rectangle>> Collidable<Rectangle, I> for Star {
    fn collides(&self, collision: &I) -> bool {
        collision.intersects(&self.body)
    }
}
