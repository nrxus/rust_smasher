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
        let drawable = Drawable::new(max_coords, resource_manager)?;
        let dims = drawable.meteor_dims();
        let radius = cmp::min(dims.x, dims.y) as f64 / 2.;
        let center = glm::dvec2(center.x as f64, center.y as f64);
        let max_coords = glm::dvec2(max_coords.x as f64, max_coords.y as f64);
        let object = Object::new(center, radius, max_coords);
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
                if self.object.collides_with(planets) {
                    self.state = MeteorState::EXPLODED;
                }
            }
            MeteorState::EXPLODED => {
                if !self.drawable.animate_explosion() {
                    self.restart();
                }
            }
        }
    }

    pub fn draw(&self, renderer: &mut ResourceManager<R>) -> Result<(), String> {
        let center = self.object.center();
        let center = glm::ivec2(center.x as i32, center.y as i32);
        match self.state {
            MeteorState::EXPLODED => self.drawable.draw_explosion(center, renderer),
            _ => self.drawable.draw_meteor(center, renderer),
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
