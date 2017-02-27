use glm;
use master_smasher::shape::Circle;

pub trait Interpolate {
    fn interpolate(&self, next: Self, interpolation: f64) -> Self;
}

impl Interpolate for glm::IVec2 {
    fn interpolate(&self, next: glm::IVec2, interpolation: f64) -> glm::IVec2 {
        let delta = next - *self;
        *self + glm::to_ivec2(glm::to_dvec2(delta) * interpolation)
    }
}

impl Interpolate for Circle {
    fn interpolate(&self, next: Circle, interpolation: f64) -> Circle {
        let delta_radius = next.radius - self.radius;
        let delta_center = next.center - self.center;
        Circle {
            radius: self.radius + delta_radius * interpolation,
            center: self.center + delta_center * interpolation,
        }
    }
}

pub struct State<T> {
    pub old: T,
    pub current: T,
}

impl<T: Copy + Interpolate> State<T> {
    pub fn new(state: T) -> Self {
        State {
            old: state,
            current: state,
        }
    }

    pub fn update(&mut self, new: T) {
        self.old = self.current;
        self.current = new;
    }

    pub fn interpolated(&self, interpolation: f64) -> T {
        self.old.interpolate(self.current, interpolation)
    }
}
