extern crate glm;

pub type Line = (glm::DVec2, glm::DVec2);

pub trait Intersect<S> {
    fn intersects(&self, other: &S) -> bool;
}

pub trait Shape {
    fn get_center(&self) -> glm::DVec2;
    fn contains(&self, point: &glm::DVec2) -> bool;
    fn distance<S>(&self, other: &S) -> f64
        where S: Shape
    {
        glm::distance(self.get_center(), other.get_center())
    }
}
