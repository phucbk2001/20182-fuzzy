use std::ops::{Add, Sub, Mul};

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[allow(dead_code)]
impl Add for Point {
    type Output = Self;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[test]
fn add_points() {
    let a = Point { x: 10.0, y: 4.0 };
    let b = Point { x: 5.0, y: 3.0 };
    let c = Point { x: 15.0, y: 7.0 };
    let s = a + b;
    assert_eq!(s.x, c.x);
    assert_eq!(s.y, c.y);
}

#[allow(dead_code)]
impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[test]
fn sub_points() {
    let a = Point { x: 10.0, y: 4.0 };
    let b = Point { x: 5.0, y: 3.0 };
    let c = Point { x: 5.0, y: 1.0 };
    let s = a - b;
    assert_eq!(s.x, c.x);
    assert_eq!(s.y, c.y);
}

#[allow(dead_code)]
impl Mul<f32> for Point {
    type Output = Self;

    fn mul(self, k: f32) -> Point {
        Point {
            x: self.x * k,
            y: self.y * k,
        }
    }
}

#[test]
fn mul_point_right() {
    let a = Point { x: 10.0, y: 4.0 };
    let b = Point { x: 20.0, y: 8.0 };
    let s = a * 2.0;
    assert_eq!(s.x, b.x);
    assert_eq!(s.y, b.y);
}

#[allow(dead_code)]
impl Mul<Point> for f32 {
    type Output = Point;

    fn mul(self, p: Point) -> Point {
        Point {
            x: self * p.x,
            y: self * p.y,
        }
    }
}

#[test]
fn mul_point_left() {
    let a = Point { x: 10.0, y: 4.0 };
    let b = Point { x: 20.0, y: 8.0 };
    let s = 2.0 * a;
    assert_eq!(s.x, b.x);
    assert_eq!(s.y, b.y);
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub struct Bezier {
    pub a: Point,
    pub b: Point,
    pub c: Point,
}

impl Bezier {
    #[allow(dead_code)]
    pub fn pos(self, s: f32) -> Point {
        let u = 1.0 - s;
        let v = s;
        self.a * u * u + self.b * 2.0 * u * v + self.c * v * v
    }
}

#[test]
fn bezier_pos_middle() {
    use approx::assert_relative_eq;

    let a = Point { x: 1.0, y: 2.0 };
    let b = Point { x: 2.0, y: 4.0 };
    let c = Point { x: 3.0, y: 6.0 };
    let curve = Bezier { a: a, b: b, c: c};
    let m = curve.pos(0.5);

    assert_relative_eq!(m.x, 2.0);
    assert_relative_eq!(m.y, 4.0);
}
