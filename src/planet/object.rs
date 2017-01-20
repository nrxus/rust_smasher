use circle::Circle;
use shape::Intersect;
use num_traits::Zero;
use glm;
use glm::ext::normalize_to;

pub struct Object {
    body: Circle,
    strength: f64,
    gravity_radius: f64,
}

impl Object {
    pub fn new(center: glm::DVec2, strength: f64, planet_radius: f64, gravity_radius: f64) -> Self {
        let body = Circle {
            center: center,
            radius: planet_radius,
        };

        Object {
            body: body,
            strength: strength,
            gravity_radius: gravity_radius,
        }
    }

    pub fn pull_vector(&self, point: glm::DVec2, radius: f64) -> glm::DVec2 {
        let dist = self.body.center - point;
        let len = glm::length(dist);
        if len > (self.gravity_radius + radius) {
            glm::DVec2::zero()
        } else {
            let force = self.strength / (len.powf(0.8));
            normalize_to(dist, force)
        }
    }

    pub fn collides_with<S: Intersect<Circle>>(&self, shape: &S) -> bool {
        shape.intersects(&self.body)
    }
}
