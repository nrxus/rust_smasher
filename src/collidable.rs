use shape::Shape;
use shape::Intersect;

pub trait Collidable<S: Shape, I: Intersect<S>> {
    fn collides(&self, collision: &I) -> bool;
}
