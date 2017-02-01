use master_smasher::shape::{Shape, Intersect};

pub trait Collidable<S: Shape, I: Intersect<S>> {
    fn collides(&self, collision: &I) -> bool;
}
