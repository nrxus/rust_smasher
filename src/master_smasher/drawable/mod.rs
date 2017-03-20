pub mod animation;
pub mod animation_data;

pub use self::animation_data::AnimationData;
pub use self::animation::Animation;

use master_smasher::shape::Circle;

use std::slice::Iter;

use glm;

pub trait Rectifiable {
    fn rectify(&self) -> glm::IVec4;
}

impl Rectifiable for Circle {
    fn rectify(&self) -> glm::IVec4 {
        glm::to_ivec4(glm::dvec4(self.center.x - self.radius,
                                 self.center.y - self.radius,
                                 self.radius * 2.,
                                 self.radius * 2.))
    }
}

pub trait TryIterator {
    type Item;
    fn try<F, E>(self, action: F) -> Result<(), E> where F: FnMut(&Self::Item) -> Result<(), E>;
}

impl<'a, T> TryIterator for Iter<'a, T> {
    type Item = T;
    fn try<F, E>(self, action: F) -> Result<(), E>
        where F: FnMut(&T) -> Result<(), E>
    {
        self.map(action)
            .take_while(Result::is_ok)
            .last()
            .unwrap_or(Ok(()))
    }
}
