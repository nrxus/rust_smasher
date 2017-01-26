use super::super::shape::Circle;
use super::super::planet::Planet;

use glm;
use num_traits::Zero;

pub struct Object {
    body: Circle,
    initial_center: glm::DVec2,
    max_coords: glm::DVec2,
    velocity: glm::DVec2,
}

impl Object {
    pub fn new(center: glm::DVec2, radius: f64, max_coords: glm::DVec2) -> Self {
        let body = Circle {
            center: center,
            radius: radius,
        };

        Object {
            body: body,
            initial_center: center,
            max_coords: max_coords,
            velocity: glm::DVec2::zero(),
        }
    }

    pub fn restart(&mut self) {
        self.body.center = self.initial_center;
        self.velocity = glm::DVec2::zero();
    }

    pub fn launch(&mut self, target: glm::IVec2) {
        const FACTOR: f64 = 50.;
        let offset = target - glm::to_ivec2(self.body.center);
        self.velocity = glm::to_dvec2(offset) / FACTOR;
    }

    pub fn update(&mut self, planets: &[Planet]) {
        self.pull(planets);
        self.displace();
    }

    pub fn body(&self) -> &Circle {
        &self.body
    }

    fn pull(&mut self, planets: &[Planet]) {
        for planet in planets {
            let acceleration = planet.pull_vector(self.body.center, self.body.radius);
            self.velocity = self.velocity + acceleration / 50.;
        }
    }

    fn displace(&mut self) {
        self.body.center = self.body.center + self.velocity;
        self.body.center = (self.body.center + self.max_coords) % self.max_coords;
    }
}
