pub mod animation;
pub mod animation_data;

pub use self::animation_data::AnimationData;
pub use self::animation::Animation;

use master_smasher::shape::Circle;

use moho::errors as moho_errors;

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
    fn try<F>(self, action: F) -> moho_errors::Result<()>
        where F: FnMut(&Self::Item) -> moho_errors::Result<()>;
}

impl<'a, T> TryIterator for Iter<'a, T> {
    type Item = T;
    fn try<F>(self, action: F) -> moho_errors::Result<()>
        where F: FnMut(&T) -> moho_errors::Result<()>
    {
        self.map(action)
            .take_while(Result::is_ok)
            .last()
            .unwrap_or(Ok(()))
    }
}
