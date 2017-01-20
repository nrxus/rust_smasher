mod drawable;
mod object;

use self::drawable::Drawable;
use self::object::Object;

use circle::Circle;
use collidable::Collidable;
use shape::Shape;
use shape::Intersect;
use glm;
use num_traits::Zero;
use moho::errors::*;
use moho::resource_manager::{Renderer, ResourceManager};
use planet::Planet;

use std::cmp;

pub enum MeteorState {
    UNLAUNCHED,
    LAUNCHED,
    EXPLODED,
}

pub struct Meteor {
    drawable: Drawable,
    object: Object,
    state: MeteorState,
    target: glm::IVec2,
}

impl Meteor {
    pub fn new<R: Renderer>(center: glm::IVec2,
                            resource_manager: &ResourceManager<R>)
                            -> Result<Self> {
        let max_coords = resource_manager.output_size()?;
        let drawable = Drawable::new(center, max_coords, resource_manager)?;
        let dims = drawable.meteor_dims();
        let radius = cmp::min(dims.x, dims.y) as f64 / 2.;
        let object = Object::new(glm::to_dvec2(center), radius, glm::to_dvec2(max_coords));
        let meteor = Meteor {
            drawable: drawable,
            object: object,
            state: MeteorState::UNLAUNCHED,
            target: glm::IVec2::zero(),
        };

        Ok(meteor)
    }

    pub fn update(&mut self, planets: &[Planet]) {
        match self.state {
            MeteorState::UNLAUNCHED => {
                self.drawable.update_launch_vector(self.target);
            }
            MeteorState::LAUNCHED => {
                self.object.update(planets);
                self.move_drawable();
            }
            MeteorState::EXPLODED => {
                self.drawable.animate_explosion();
                if !self.drawable.is_exploding() {
                    self.object.restart();
                    self.move_drawable();
                    self.state = MeteorState::UNLAUNCHED;
                }
            }
        }
    }

    pub fn draw<R: Renderer>(&self, renderer: &mut ResourceManager<R>) -> Result<()> {
        match self.state {
            MeteorState::UNLAUNCHED => self.drawable.draw_unlaunched(renderer),
            MeteorState::LAUNCHED => self.drawable.draw_meteor(renderer),
            MeteorState::EXPLODED => self.drawable.draw_explosion(renderer),
        }
    }

    pub fn state(&self) -> &MeteorState {
        &self.state
    }

    pub fn explode(&mut self) {
        self.state = MeteorState::EXPLODED;
    }

    pub fn launch(&mut self) {
        self.object.launch(self.target);
        self.state = MeteorState::LAUNCHED;
    }

    pub fn update_target(&mut self, target: glm::IVec2) {
        self.target = target;
    }

    pub fn collides<S, C>(&self, collidable: &C) -> bool
        where S: Shape,
              C: Collidable<S, Circle>,
              Circle: Intersect<S>
    {
        collidable.collides(self.object.body())
    }

    fn move_drawable(&mut self) {
        let body = self.object.body();
        self.drawable.center = glm::to_ivec2(body.center);
    }
}
