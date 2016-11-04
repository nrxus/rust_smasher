extern crate glm;

pub trait Shape {
    fn get_center(&self) -> glm::DVec2;
    fn contains(&self, point: &glm::DVec2) -> bool;
    fn intersects_with_circle(&self, circle: &Circle) -> bool;
    fn intersects_with_rectangle(&self, rectangle: &Rectangle) -> bool;
    fn distance<S>(&self, other: &S) -> f64
        where S: Shape
    {
        glm::distance(self.get_center(), other.get_center())
    }
}

pub struct Rectangle {
    dims: glm::DVec2,
    center: glm::DVec2,
}

impl Shape for Rectangle {
    fn get_center(&self) -> glm::DVec2 {
        self.center
    }

    fn contains(&self, point: &glm::DVec2) -> bool {
        let half_x = self.dims.x / 2.;
        let half_y = self.dims.y / 2.;

        if self.center.x - half_x > point.x {
            false
        } else if self.center.x + half_x < point.x {
            false
        } else if self.center.y - half_y > point.y {
            false
        } else if self.center.y + half_y < point.y {
            false
        } else {
            true
        }
    }

    fn intersects_with_rectangle(&self, rectangle: &Rectangle) -> bool {
        let half_x = self.dims.x / 2.;
        let half_y = self.dims.y / 2.;

        if self.center.x - half_x > rectangle.center.x + rectangle.dims.x / 2. {
            false
        } else if self.center.x + half_x < rectangle.center.x - rectangle.dims.x / 2. {
            false
        } else if self.center.y - half_y > rectangle.center.y + rectangle.dims.y / 2. {
            false
        } else if self.center.y + half_y < rectangle.center.y - rectangle.dims.y / 2. {
            false
        } else {
            true
        }
    }

    fn intersects_with_circle(&self, circle: &Circle) -> bool {
        circle.intersects_with_rectangle(&self)
    }
}

pub struct Circle {
    radius: f64,
    center: glm::DVec2,
}

impl Shape for Circle {
    fn get_center(&self) -> glm::DVec2 {
        self.center
    }

    fn contains(&self, point: &glm::DVec2) -> bool {
        let distance = glm::distance(self.center, *point);
        distance < self.radius
    }

    fn intersects_with_rectangle(&self, rectangle: &Rectangle) -> bool {
        if self.contains(&rectangle.center) {
            true
        } else if rectangle.contains(&self.center) {
            true
        } else {
            false
        }
    }

    fn intersects_with_circle(&self, circle: &Circle) -> bool {
        let distance = glm::distance(self.center, circle.center);
        distance < (self.radius + circle.radius)
    }
}

#[cfg(test)]
mod test {
    extern crate glm;
    use super::*;

    #[test]
    fn circle_no_contains() {
        let circle = Circle {
            radius: 1 as f64,
            center: glm::dvec2(2 as f64, 0 as f64),
        };
        let point = glm::dvec2(4 as f64, 4 as f64);
        assert!(!circle.contains(&point));
    }

    #[test]
    fn circle_contains() {
        let circle = Circle {
            radius: 3 as f64,
            center: glm::dvec2(2 as f64, 3 as f64),
        };
        let point = glm::dvec2(4 as f64, 4 as f64);
        assert!(circle.contains(&point));
    }

    #[test]
    fn circle_circle_no_intersect() {
        let circle_a = Circle {
            radius: 3 as f64,
            center: glm::dvec2(2 as f64, 3 as f64),
        };

        let circle_b = Circle {
            radius: 1 as f64,
            center: glm::dvec2(4 as f64, 7 as f64),
        };

        assert!(!circle_a.intersects_with_circle(&circle_b));
        assert!(!circle_b.intersects_with_circle(&circle_a));
    }

    #[test]
    fn circle_circle_intersect() {
        let circle_a = Circle {
            radius: 3 as f64,
            center: glm::dvec2(2 as f64, 3 as f64),
        };

        let circle_b = Circle {
            radius: 1 as f64,
            center: glm::dvec2(1 as f64, 5 as f64),
        };

        assert!(circle_a.intersects_with_circle(&circle_b));
        assert!(circle_b.intersects_with_circle(&circle_a));
    }

    #[test]
    fn rectangle_no_contains() {
        let rectangle = Rectangle {
            dims: glm::dvec2(2 as f64, 3 as f64),
            center: glm::dvec2(3 as f64, 0 as f64),
        };
        let point = glm::dvec2(3.5 as f64, -1 as f64);
        assert!(rectangle.contains(&point));
    }

    #[test]
    fn rectangle_contains() {
        let rectangle = Rectangle {
            dims: glm::dvec2(2 as f64, 3 as f64),
            center: glm::dvec2(3 as f64, 0 as f64),
        };
        let point = glm::dvec2(3.5 as f64, 2 as f64);
        assert!(!rectangle.contains(&point));
    }

    #[test]
    fn rectangle_rectangle_no_intersect() {
        let rectangle_a = Rectangle {
            dims: glm::dvec2(2 as f64, 3 as f64),
            center: glm::dvec2(3 as f64, 0 as f64),
        };

        let rectangle_b = Rectangle {
            dims: glm::dvec2(5 as f64, 2 as f64),
            center: glm::dvec2(0 as f64, 8 as f64),
        };
        assert!(!rectangle_a.intersects_with_rectangle(&rectangle_b));
        assert!(!rectangle_b.intersects_with_rectangle(&rectangle_a));
    }

    #[test]
    fn rectangle_rectangle_intersect() {
        let rectangle_a = Rectangle {
            dims: glm::dvec2(2 as f64, 3 as f64),
            center: glm::dvec2(3 as f64, 0 as f64),
        };

        let rectangle_b = Rectangle {
            dims: glm::dvec2(1 as f64, 2 as f64),
            center: glm::dvec2(2.5 as f64, 1 as f64),
        };
        assert!(rectangle_a.intersects_with_rectangle(&rectangle_b));
        assert!(rectangle_b.intersects_with_rectangle(&rectangle_a));
    }

    #[test]
    fn rectangle_circle_no_intersect() {
        let rectangle = Rectangle {
            dims: glm::dvec2(5 as f64, 2 as f64),
            center: glm::dvec2(6 as f64, 0 as f64),
        };

        let circle = Circle {
            radius: 3 as f64,
            center: glm::dvec2(2 as f64, 3 as f64),
        };

        assert!(!rectangle.intersects_with_circle(&circle));
        assert!(!circle.intersects_with_rectangle(&rectangle));
    }

    #[test]
    fn rectangle_inside_circle() {
        let rectangle = Rectangle {
            dims: glm::dvec2(1 as f64, 2 as f64),
            center: glm::dvec2(3 as f64, 5 as f64),
        };

        let circle = Circle {
            radius: 5 as f64,
            center: glm::dvec2(2 as f64, 3 as f64),
        };

        assert!(rectangle.intersects_with_circle(&circle));
        assert!(circle.intersects_with_rectangle(&rectangle));
    }

    #[test]
    fn circle_inside_rectangle() {
        let rectangle = Rectangle {
            dims: glm::dvec2(5 as f64, 7 as f64),
            center: glm::dvec2(4 as f64, 2 as f64),
        };

        let circle = Circle {
            radius: 1 as f64,
            center: glm::dvec2(5 as f64, 3 as f64),
        };

        assert!(rectangle.intersects_with_circle(&circle));
        assert!(circle.intersects_with_rectangle(&rectangle));
    }
}
