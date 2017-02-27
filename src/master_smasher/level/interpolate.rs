use glm;

pub trait Interpolate {
    fn interpolate(&self, next: Self, interpolation: f64) -> Self;
}

impl Interpolate for glm::IVec2 {
    fn interpolate(&self, next: glm::IVec2, interpolation: f64) -> glm::IVec2 {
        let delta = next - *self;
        *self + glm::to_ivec2(glm::to_dvec2(delta) * interpolation)
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
