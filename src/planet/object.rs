use circle::Circle;
use shape::Intersect;
use num_traits::Zero;
use glm;
use glm::ext::normalize_to;

pub struct Object {
    center: glm::DVec2,
    strength: f64,
    planet_radius: f64,
    gravity_radius: f64,
}

impl Object {
    pub fn new(center: glm::DVec2, strength: f64, planet_radius: f64, gravity_radius: f64) -> Self {
        Object {
            center: center,
            strength: strength,
            planet_radius: planet_radius,
            gravity_radius: gravity_radius,
        }
    }

    pub fn pull_vector(&self, point: glm::DVec2, radius: f64) -> glm::DVec2 {
        let dist = self.center - point;
        let len = glm::length(dist);
        if len > (self.gravity_radius + radius) {
            glm::DVec2::zero()
        } else {
            let force = self.strength / (len.powf(0.8));
            normalize_to(dist, force)
        }
    }

    pub fn collides_with<S: Intersect<Circle>>(&self, shape: &S) -> bool {
        shape.intersects(&self.collision_body())
    }

    fn collision_body(&self) -> Circle {
        Circle {
            center: self.center,
            radius: self.planet_radius,
        }
    }
}
