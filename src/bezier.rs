use std::ops::{Add, Sub, Mul};
use nalgebra as na;

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

    pub fn normalize(self) -> Self {
        self * (1.0 / self.len())
    }

    pub fn turn_left_90_degree(self) -> Point {
        Point {
            x: -self.y,
            y: self.x,
        }
    }

    pub fn turn_right_90_degree(self) -> Point {
        Point {
            x: self.y,
            y: -self.x,
        }
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

impl Add<Point> for Point {
    type Output = Self;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub<Point> for Point {
    type Output = Self;

    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
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

impl Mul<Point> for f32 {
    type Output = Point;

    fn mul(self, p: Point) -> Point {
        Point {
            x: self * p.x,
            y: self * p.y,
        }
    }
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

    pub fn direction(self, s: f32) -> Point {
        (self.a * 2.0 * s - self.a * 2.0 + 
            self.b * 2.0 - self.b * 4.0 * s +
            self.c * 2.0 * s).normalize()
    }
}

#[derive(Copy, Clone)]
pub struct Line {
    pub position: Point,
    pub direction: Point,
}

#[derive(Copy, Clone)]
pub struct Matrix {
    pub a: f32,
    pub b: f32, 
    pub c: f32, 
    pub d: f32,
}

impl Matrix {
    pub fn det(self) -> f32 {
        let Matrix { a, b, c, d } = self;
        a * d - b * c
    }

    pub fn inv(self) -> Self {
        let det = self.det();
        let Matrix { a, b, c, d } = self;
        Matrix {
            a: d / det,
            b: -b / det,
            c: -c / det,
            d: a / det,
        }
    }

    pub fn rotation_from(u: Point) -> Matrix {
        let u = u.normalize();
        let v = u.turn_left_90_degree();

        Matrix {
            a: u.x, 
            c: u.y,
            b: v.x, 
            d: v.y,
        }
    }

    pub fn transpose(self) -> Matrix {
        let Matrix { a, b, c, d } = self;
        Matrix {
            a: a, b: c, 
            c: b, d: d,
        }
    }

    pub fn to_na_matrix(self) -> na::Matrix4<f32> {
        let Matrix { a, b, c, d } = self;
        na::Matrix4::<f32>::new(
              a,   b, 0.0, 0.0,
              c,   d, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        )
    }
}

impl Mul<Point> for Matrix {
    type Output = Point;

    fn mul(self, p: Point) -> Point {
        let Matrix { a, b, c, d } = self;
        Point {
            x: a * p.x + b * p.y,
            y: c * p.x + d * p.y,
        }
    }
}

impl Mul<Matrix> for Matrix {
    type Output = Matrix;

    fn mul(self, m: Matrix) -> Matrix {
        let Matrix { a, b, c, d } = self;
        let (a1, b1, c1, d1) = (a, b, c, d);

        let Matrix { a, b, c, d } = m;
        let (a2, b2, c2, d2) = (a, b, c, d);

        Matrix {
            a: a1 * a2 + b1 * c2,
            b: a1 * b2 + b1 * d2,
            c: c1 * a2 + d1 * c2, 
            d: c1 * b2 + d1 * d2,
        }
    }
}

impl Mul<f32> for Matrix {
    type Output = Matrix;

    fn mul(self, k: f32) -> Matrix {
        let Matrix { a, b, c, d } = self;
        Matrix {
            a: a * k, b: b * k,
            c: c * k, d: d * k,
        }
    }
}

impl Mul<Matrix> for f32 {
    type Output = Matrix;

    fn mul(self, m: Matrix) -> Matrix {
        let Matrix { a, b, c, d } = m;
        let k = self;
        Matrix {
            a: a * k, b: b * k,
            c: c * k, d: d * k,
        }
    }
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

    let m = Matrix {
        a: a1, b: b1,
        c: a2, d: b2,
    };
    let d = m.det();

    if f32::abs(d) <= 0.0001 {
        (p1 + p2) * 0.5
    }
    else {
        let m1 = Matrix {
            a: c1, b: b1,
            c: c2, d: b2,
        };
        let du = m1.det();
        let u = du / d;
        p1 + u * v1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn add_points() {
        let a = Point { x: 10.0, y: 4.0 };
        let b = Point { x: 5.0, y: 3.0 };
        let c = Point { x: 15.0, y: 7.0 };
        let s = a + b;
        assert_relative_eq!(s.x, c.x);
        assert_relative_eq!(s.y, c.y);
    }

    #[test]
    fn sub_points() {
        let a = Point { x: 10.0, y: 4.0 };
        let b = Point { x: 4.0, y: 3.0 };
        let c = Point { x: 6.0, y: 1.0 };
        let s = a - b;
        assert_relative_eq!(s.x, c.x);
        assert_relative_eq!(s.y, c.y);
    }

    #[test]
    fn mul_point_right() {
        let a = Point { x: 10.0, y: 4.0 };
        let b = Point { x: 20.0, y: 8.0 };
        let s = a * 2.0;
        assert_relative_eq!(s.x, b.x);
        assert_relative_eq!(s.y, b.y);
    }

    #[test]
    fn mul_point_left() {
        let a = Point { x: 10.0, y: 4.0 };
        let b = Point { x: 20.0, y: 8.0 };
        let s = 2.0 * a;
        assert_relative_eq!(s.x, b.x);
        assert_relative_eq!(s.y, b.y);
    }


    #[allow(dead_code)]
    fn assert_point_eq(a: Point, b: Point) {
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
        let r = Matrix {
            a: 1.0, b: 2.0,
            c: 3.0, d: 4.0,
        }.det();
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

        let r = v.turn_left_90_degree();
        assert_point_eq(r, Point::new(-1.0, 2.0));

        let r = v.turn_right_90_degree();
        assert_point_eq(r, Point::new(1.0, -2.0));
    }

    #[test]
    fn test_mul_matrix_to_point() {
        let m = Matrix {
            a: 1.0, b: 2.0,
            c: 3.0, d: 4.0,
        };

        let p = Point { x: 1.0, y: 2.0 };
        let p1 = m * p;
        assert_relative_eq!(p1.x, 5.0);
        assert_relative_eq!(p1.y, 11.0);
    }

    fn mat_is_id(m: Matrix) {
        let Matrix { a, b, c, d } = m;
        assert_relative_eq!(a, 1.0);
        assert_relative_eq!(b, 0.0);
        assert_relative_eq!(c, 0.0);
        assert_relative_eq!(d, 1.0);
    }

    #[test]
    fn test_matrix_mul_matrix() {
        let m1 = Matrix {
            a: 1.0, b: 2.0,
            c: 2.0, d: 3.0,
        };

        let m2 = Matrix {
            a: 3.0, b: 1.0,
            c: 2.0, d: 4.0,
        };

        let m = m1 * m2;

        let Matrix { a, b, c, d } = m;

        assert_relative_eq!(a, 7.0);
        assert_relative_eq!(b, 9.0);
        assert_relative_eq!(c, 12.0);
        assert_relative_eq!(d, 14.0);
    }

    #[test]
    fn test_matrix_inv() {
        let m1 = Matrix {
            a: 1.0, b: 2.0,
            c: 2.0, d: 3.0,
        };

        let m = m1.inv();
        mat_is_id(m1 * m);
        mat_is_id(m * m1);
    }

    #[test]
    fn test_matrix_from_direction() {
        let m = Matrix::rotation_from(
            Point { x: 2.0, y: 2.0 });

        let m1 = m.transpose();

        mat_is_id(m1 * m);

        let k = 1.0 / f32::sqrt(2.0);
        let Matrix { a, b, c, d } = m;

        assert_relative_eq!(a, k);
        assert_relative_eq!(b, -k);
        assert_relative_eq!(c, k);
        assert_relative_eq!(d, k);

        let p = m * Point { x: 3.0, y: 3.0 };
        assert_relative_eq!(p.x, 0.0);
        assert_relative_eq!(p.y, 3.0 * f32::sqrt(2.0));
    }
}
