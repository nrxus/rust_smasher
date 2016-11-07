extern crate glm;

type Line = (glm::DVec2, glm::DVec2);

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

pub struct Rectangle {
    dims: glm::DVec2,
    center: glm::DVec2,
}

impl Rectangle {
    fn get_lines(&self) -> [Line; 4] {
        let tl = self.center + glm::dvec2(-self.dims.x / 2., -self.dims.y / 2.);
        let bl = self.center + glm::dvec2(-self.dims.x / 2., self.dims.y / 2.);
        let br = self.center + glm::dvec2(self.dims.x / 2., self.dims.y / 2.);
        let tr = self.center + glm::dvec2(self.dims.x / 2., -self.dims.y / 2.);

        [(tl, bl), (tl, tr), (bl, br), (br, tr)]
    }
}

impl Intersect<Rectangle> for Rectangle {
    fn intersects(&self, other: &Rectangle) -> bool {
        let half_x = self.dims.x / 2.;
        let half_y = self.dims.y / 2.;

        if self.center.x - half_x > other.center.x + other.dims.x / 2. {
            false
        } else if self.center.x + half_x < other.center.x - other.dims.x / 2. {
            false
        } else if self.center.y - half_y > other.center.y + other.dims.y / 2. {
            false
        } else if self.center.y + half_y < other.center.y - other.dims.y / 2. {
            false
        } else {
            true
        }
    }
}

impl Intersect<Circle> for Rectangle {
    fn intersects(&self, other: &Circle) -> bool {
        other.intersects(self)
    }
}

impl Intersect<Rectangle> for Circle {
    fn intersects(&self, other: &Rectangle) -> bool {
        if self.contains(&other.center) {
            true
        } else if other.contains(&self.center) {
            true
        } else {
            other.get_lines().iter().any(|l| self.intersects(l))
        }
    }
}

impl Intersect<Circle> for Circle {
    fn intersects(&self, other: &Circle) -> bool {
        let distance = glm::distance(self.center, other.center);
        distance < (self.radius + other.radius)
    }
}

impl Intersect<Line> for Circle {
    fn intersects(&self, other: &Line) -> bool {
        let length = other.1 - other.0;
        let dist_center = other.0 - self.center;
        let len_sq = glm::dot(length, length);
        let b = 2_f64 * glm::dot(dist_center, length);
        let c = glm::dot(dist_center, dist_center) - self.radius * self.radius;
        let mut discriminant = b * b - 4_f64 * len_sq * c;

        if discriminant < 0_f64 {
            return false;
        }

        discriminant = discriminant.sqrt();

        let t1 = (-b - discriminant) / (2_f64 * len_sq);
        let t2 = (-b + discriminant) / (2_f64 * len_sq);

        if t1 >= 0_f64 && t1 <= 1_f64 {
            true
        } else if t2 >= 0_f64 && t2 <= 1_f64 {
            true
        } else {
            false
        }
    }
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

        assert!(!circle_a.intersects(&circle_b));
        assert!(!circle_b.intersects(&circle_a));
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

        assert!(circle_a.intersects(&circle_b));
        assert!(circle_b.intersects(&circle_a));
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
        assert!(!rectangle_a.intersects(&rectangle_b));
        assert!(!rectangle_b.intersects(&rectangle_a));
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
        assert!(rectangle_a.intersects(&rectangle_b));
        assert!(rectangle_b.intersects(&rectangle_a));
    }

    #[test]
    fn rectangle_circle_no_intersect() {
        let rectangle = Rectangle {
            dims: glm::dvec2(5 as f64, 2 as f64),
            center: glm::dvec2(6 as f64, 0 as f64),
        };

        let circle = Circle {
            radius: 3 as f64,
            center: glm::dvec2(1 as f64, 3 as f64),
        };

        assert!(!rectangle.intersects(&circle));
        assert!(!circle.intersects(&rectangle));
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

        assert!(rectangle.intersects(&circle));
        assert!(circle.intersects(&rectangle));
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

        assert!(rectangle.intersects(&circle));
        assert!(circle.intersects(&rectangle));
    }

    #[test]
    fn rectangle_circle_intersect() {
        let rectangle = Rectangle {
            dims: glm::dvec2(2 as f64, 2 as f64),
            center: glm::dvec2(5 as f64, 3 as f64),
        };

        let circle = Circle {
            radius: 2 as f64,
            center: glm::dvec2(2 as f64, 3 as f64),
        };

        assert!(rectangle.intersects(&circle));
        assert!(circle.intersects(&rectangle));
    }
}
