use std::ops::{Add, Sub, Mul};

#[derive(Copy, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x: x, y: y }
    }

    pub fn len(&self) -> f32 {
        let x = self.x;
        let y = self.y;
        f32::sqrt(x * x + y * y)
    }
}

impl From<(f32, f32)> for Point {
    fn from(v: (f32, f32)) -> Self {
        let (x, y) = v;
        Self { x: x, y: y }
    }
}

impl From<Point> for (f32, f32) {
    fn from(p: Point) -> Self {
        (p.x, p.y)
    }
}

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

#[derive(Copy, Clone)]
pub struct Bezier {
    pub a: Point,
    pub b: Point,
    pub c: Point,
}

impl Bezier {
    pub fn pos(self, s: f32) -> Point {
        let u = 1.0 - s;
        let v = s;
        self.a * u * u + self.b * 2.0 * u * v + self.c * v * v
    }
}

#[derive(Copy, Clone)]
pub struct Line {
    pub position: Point,
    pub direction: Point,
}

fn det(d: [[f32; 2]; 2]) -> f32 {
    d[0][0] * d[1][1] - d[1][0] * d[0][1]
}

pub fn intersect_lines(
    line1: Line, line2: Line) -> Point
{
    let v1 = line1.direction;
    let v2 = line2.direction;
    let (a1, a2) = (v1.x, v1.y);
    let (b1, b2) = (-v2.x, -v2.y);
    let p1 = line1.position;
    let p2 = line2.position;
    let c1 = p2.x - p1.x;
    let c2 = p2.y - p1.y;

    let d = det([
        [a1, b1],
        [a2, b2],
    ]);

    let du = det([
        [c1, b1],
        [c2, b2],
    ]);

    let u = du / d;

    p1 + u * v1
}

pub fn turn_left_90_degree(v: Point) -> Point {
    Point {
        x: -v.y,
        y: v.x,
    }
}

pub fn turn_right_90_degree(v: Point) -> Point {
    Point {
        x: v.y,
        y: -v.x,
    }
}

#[allow(dead_code)]
fn assert_point_eq(a: Point, b: Point) {
    use approx::assert_relative_eq;
    assert_relative_eq!(a.x, b.x);
    assert_relative_eq!(a.y, b.y);
}

#[test]
fn bezier_pos_middle() {
    let a = Point { x: 1.0, y: 2.0 };
    let b = Point { x: 2.0, y: 4.0 };
    let c = Point { x: 3.0, y: 6.0 };
    let curve = Bezier { a: a, b: b, c: c};
    let m = curve.pos(0.5);
    assert_point_eq(m, Point::new(2.0, 4.0));
}

#[test]
fn determinant() {
    use approx::assert_relative_eq;
    let r = det([
        [1.0, 2.0],
        [3.0, 4.0],
    ]);
    assert_relative_eq!(r, -2.0);
}

#[test]
fn two_lines_intersect() {
    let p1 = Point::new(0.0, 0.0);
    let p2 = Point::new(3.0, 0.0);
    let v1 = Point::new(1.0, 1.0);
    let v2 = Point::new(-1.0, 2.0);
    let p = intersect_lines(
        Line { position: p1, direction: v1 }, 
        Line { position: p2, direction: v2 });
    assert_point_eq(p, Point::new(2.0, 2.0));
}

#[test]
fn test_turn_90_degree() {
    let v = Point::new(2.0, 1.0);

    let r = turn_left_90_degree(v);
    assert_point_eq(r, Point::new(-1.0, 2.0));

    let r = turn_right_90_degree(v);
    assert_point_eq(r, Point::new(1.0, -2.0));
}
