#[allow(dead_code)]
#[derive(Copy, Clone)]
pub struct QuadEq {
    a: f32,
    b: f32,
    c: f32,
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum Solution {
    Zero, 
    One(f32),
    Two(f32, f32),
}

#[allow(dead_code)]
pub fn solve(eq: QuadEq) -> Solution {
    use Solution::{Zero, One, Two};

    let QuadEq { a, b, c } = eq;
    let delta = b * b - 4.0 * a * c;

    if delta < 0.0 {
        Zero
    }
    else if delta == 0.0 {
        let x = -b / (2.0 * a);
        One(x)
    }
    else {
        let d = delta.sqrt();
        let x1 = (-b - d) / (2.0 * a);
        let x2 = (-b + d) / (2.0 * a);
        Two(x1, x2)
    }
}

#[test]
fn solve_simple() {
    let eq = QuadEq { a: 1.0, b: 2.0, c: 3.0 };
    let s = solve(eq);
    match s {
        Solution::Zero => (),
        _ => panic!("Wrong solution")
    }
}

#[test]
fn solve_one_solution() {
    use approx::assert_relative_eq;

    let eq = QuadEq { a: 1.0, b: 2.0, c: 1.0 };
    let s = solve(eq);

    match s {
        Solution::One(x) => assert_relative_eq!(x, -1.0),
        _ => panic!("Wrong solution"),
    }
}

#[test]
fn solve_two_solution() {
    use approx::assert_relative_eq;

    let eq = QuadEq { a: 1.0, b: 3.0, c: 2.0 };
    let s = solve(eq);

    match s {
        Solution::Two(x, y) => {
            assert_relative_eq!(x, -2.0);
            assert_relative_eq!(y, -1.0);
        },
        _ => panic!("Wrong solution"),
    }
}
