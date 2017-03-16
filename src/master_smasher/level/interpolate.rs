use glm;
use master_smasher::shape::Circle;

use std::mem;

pub trait Interpolate {
    fn interpolate(&self, next: &Self, interpolation: f64) -> Self;
}

impl Interpolate for glm::IVec2 {
    fn interpolate(&self, next: &glm::IVec2, interpolation: f64) -> glm::IVec2 {
        let delta = *next - *self;
        *self + glm::to_ivec2(glm::to_dvec2(delta) * interpolation)
    }
}

impl Interpolate for Circle {
    fn interpolate(&self, next: &Circle, interpolation: f64) -> Circle {
        let delta_radius = next.radius - self.radius;
        let delta_center = next.center - self.center;
        Circle {
            radius: self.radius + delta_radius * interpolation,
            center: self.center + delta_center * interpolation,
        }
    }
}

#[derive(Clone)]
pub struct Wrapped<T: Clone> {
    pub actual: T,
    pub unwrapped: Option<T>,
    pub wrapping: glm::DVec2,
}

impl Wrapped<Circle> {
    pub fn displace(&self, displacement: glm::DVec2) -> Wrapped<Circle> {
        let mut center = self.actual.center + displacement;
        let mut unwrapped = None;
        match center {
            glm::DVec2 { x, y } if (x < 0.) | (y < 0.) | (x > self.wrapping.x) |
                                   (y > self.wrapping.y) => {
                unwrapped = Some(Circle {
                                     center: center,
                                     radius: self.actual.radius,
                                 });
                center = (center + self.wrapping) % self.wrapping;
            }
            _ => {}
        }
        Wrapped {
            actual: Circle {
                center: center,
                radius: self.actual.radius,
            },
            unwrapped: unwrapped,
            wrapping: self.wrapping,
        }
    }
}

impl<T: Clone + Interpolate> Interpolate for Wrapped<T> {
    fn interpolate(&self, next: &Wrapped<T>, interpolation: f64) -> Wrapped<T> {
        let interpolated = match next.unwrapped {
            Some(ref c) => self.actual.interpolate(c, interpolation),
            None => self.actual.interpolate(&next.actual, interpolation),
        };

        Wrapped {
            actual: interpolated,
            unwrapped: None,
            wrapping: self.wrapping,
        }
    }
}

pub struct State<T> {
    pub old: T,
    pub current: T,
}

impl<T> State<T> {
    pub fn update(&mut self, new: T) {
        self.old = mem::replace(&mut self.current, new);
    }
}

impl<T: Clone> State<T> {
    pub fn new(state: T) -> Self {
        State {
            old: state.clone(),
            current: state,
        }
    }
}

impl<T: Interpolate> State<T> {
    pub fn interpolated(&self, interpolation: f64) -> T {
        self.old.interpolate(&self.current, interpolation)
    }
}
