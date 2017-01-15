mod drawable;
mod object;

use self::drawable::Drawable;
use self::object::Object;

use glm;
use moho::resource_manager::{Renderer, ResourceManager};
use planet::Planet;

use std::cmp;

#[derive(Copy, Clone)]
pub enum MeteorState {
    UNLAUNCHED,
    LAUNCHED,
    EXPLODED,
}

pub struct Meteor<R: Renderer> {
    drawable: Drawable<R>,
    object: Object,
    state: MeteorState,
}

impl<R: Renderer> Meteor<R> {
    pub fn new(center: glm::IVec2,
               resource_manager: &mut ResourceManager<R>)
               -> Result<Self, String> {
        let max_coords = resource_manager.output_size()?;
        let drawable = Drawable::new(center, max_coords, resource_manager)?;
        let dims = drawable.meteor_dims();
        let radius = cmp::min(dims.x, dims.y) as f64 / 2.;
        let object = Object::new(glm::to_dvec2(center), radius, glm::to_dvec2(max_coords));
        let meteor = Meteor {
            drawable: drawable,
            object: object,
            state: MeteorState::UNLAUNCHED,
        };

        Ok(meteor)
    }

    pub fn restart(&mut self) {
        self.object.restart();
        self.state = MeteorState::UNLAUNCHED;
    }

    pub fn launch(&mut self, target: glm::IVec2) {
        self.object.launch(target);
        self.state = MeteorState::LAUNCHED;
    }

    pub fn update(&mut self, planets: &[Planet<R>]) {
        match self.state {
            MeteorState::UNLAUNCHED => {}
            MeteorState::LAUNCHED => {
                self.object.update(planets);
                self.drawable.center = glm::to_ivec2(self.center());
                if self.object.collides_with(planets) {
                    self.state = MeteorState::EXPLODED;
                }
            }
            MeteorState::EXPLODED => {
                if !self.drawable.animate_explosion() {
                    self.restart();
                    self.drawable.center = glm::to_ivec2(self.center());
                }
            }
        }
    }

    pub fn draw(&self, renderer: &mut ResourceManager<R>) -> Result<(), String> {
        match self.state {
            MeteorState::EXPLODED => self.drawable.draw_explosion(renderer),
            _ => self.drawable.draw_meteor(renderer),
        }
    }

    pub fn state(&self) -> MeteorState {
        self.state
    }

    pub fn radius(&self) -> f64 {
        self.object.radius()
    }

    pub fn center(&self) -> glm::DVec2 {
        self.object.center()
    }
}
