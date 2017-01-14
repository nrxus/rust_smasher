use circle::Circle;
use planet::Planet;

use glm;
use moho::resource_manager::Renderer;

pub struct WorldMeteor {
    initial_center: glm::DVec2,
    center: glm::DVec2,
    radius: f64,
    max_coords: glm::UVec2,
    velocity: glm::DVec2,
}

impl WorldMeteor {
    pub fn new(center: glm::IVec2, radius: f64, max_coords: glm::UVec2) -> Self {
        let center = glm::dvec2(center.x as f64, center.y as f64);

        WorldMeteor {
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

    pub fn launch(&mut self, target: glm::Vector2<i32>) {
        const FACTOR: f64 = 50.;
        let offset = glm::ivec2(target.x - self.center.x as i32,
                                target.y - self.center.y as i32);
        self.velocity = glm::dvec2(offset.x as f64 / FACTOR, offset.y as f64 / FACTOR);
    }

    pub fn update<R: Renderer>(&mut self, planets: &[Planet<R>]) {
        self.pull(planets);
        self.displace();
    }

    pub fn radius(&self) -> f64 {
        self.radius
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
        self.center.y += self.velocity.y;
        self.center.x += self.velocity.x;

        let max_height = self.max_coords.y as f64;
        let max_width = self.max_coords.x as f64;

        self.center.y = (self.center.y + max_height) % max_height;
        self.center.x = (self.center.x + max_width) % max_width;
    }

    fn collision_body(&self) -> Circle {
        Circle {
            center: self.center,
            radius: self.radius as f64,
        }
    }
}
