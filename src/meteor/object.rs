use circle::Circle;
use planet::Planet;

use glm;
use moho::resource_manager::Renderer;

pub struct Object {
    initial_center: glm::DVec2,
    center: glm::DVec2,
    radius: f64,
    max_coords: glm::DVec2,
    velocity: glm::DVec2,
}

impl Object {
    pub fn new(center: glm::DVec2, radius: f64, max_coords: glm::DVec2) -> Self {
        Object {
            initial_center: center,
            center: center,
            radius: radius,
            max_coords: max_coords,
            velocity: glm::dvec2(0., 0.),
        }
    }

    pub fn restart(&mut self) {
        self.center = self.initial_center;
        self.velocity = glm::dvec2(0., 0.);
    }

    pub fn launch(&mut self, target: glm::IVec2) {
        const FACTOR: f64 = 50.;
        let offset = target - glm::to_ivec2(self.center);
        self.velocity = glm::to_dvec2(offset) / FACTOR;
    }

    pub fn update<R: Renderer>(&mut self, planets: &[Planet<R>]) {
        self.pull(planets);
        self.displace();
    }

    pub fn center(&self) -> glm::DVec2 {
        self.center
    }

    pub fn collides_with<R: Renderer>(&self, planets: &[Planet<R>]) -> bool {
        let body = self.collision_body();
        planets.iter().any(|p| p.collides_with(&body))
    }

    fn pull<R: Renderer>(&mut self, planets: &[Planet<R>]) {
        for planet in planets {
            let acceleration = planet.pull_vector(self.center, self.radius);
            self.velocity = self.velocity + acceleration / 50.;
        }
    }

    fn displace(&mut self) {
        self.center.x += self.velocity.x;
        self.center.y += self.velocity.y;

        self.center.x = (self.center.x + self.max_coords.x) % self.max_coords.x;
        self.center.y = (self.center.y + self.max_coords.y) % self.max_coords.y;
    }

    fn collision_body(&self) -> Circle {
        Circle {
            center: self.center,
            radius: self.radius,
        }
    }
}
