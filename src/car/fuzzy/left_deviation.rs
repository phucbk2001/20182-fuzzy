use super::*;

impl LeftDeviation {
    fn far_left_fn(x: f32) -> f32 {
        let x1 = 0.25;
        let x2 = 0.4;

        if x < x1 {
            1.0
        }
        else if x < x2 {
            (x2 - x) / (x2 - x1)
        }
        else {
            0.0
        }
    }

    fn left_fn(x: f32) -> f32 {
        let x1 = 0.25;
        let x2 = 0.4;
        let x3 = 0.5;

        if x < x1 {
            0.0
        }
        else if x < x2 {
            (x - x1) / (x2 - x1)
        }
        else if x < x3 {
            (x3 - x) / (x3 - x2)
        }
        else {
            0.0
        }
    }

    fn middle_fn(x: f32) -> f32 {
        let x1 = 0.4;
        let x2 = 0.5;
        let x3 = 0.6;

        if x < x1 {
            0.0
        }
        else if x < x2 {
            (x - x1) / (x2 - x1)
        }
        else if x < x3 {
            (x3 - x) / (x3 - x2)
        }
        else {
            0.0
        }
    }

    fn right_fn(x: f32) -> f32 {
        let x1 = 0.5;
        let x2 = 0.6;
        let x3 = 0.75;

        if x < x1 {
            0.0
        }
        else if x < x2 {
            (x - x1) / (x2 - x1)
        }
        else if x < x3 {
            (x3 - x) / (x3 - x2)
        }
        else {
            0.0
        }
    }

    fn far_right_fn(x: f32) -> f32 {
        let x1 = 0.6;
        let x2 = 0.75;

        if x < x1 {
            0.0
        }
        else if x < x2 {
            (x - x1) / (x2 - x1)
        }
        else {
            1.0
        }
    }

    pub fn new(fuzzy: &mut Fuzzy) -> Self {
        let input = fuzzy.add_input(0.0, 1.0);

        let far_left = fuzzy.add_input_set(
            input, Box::new(LeftDeviation::far_left_fn));

        let left = fuzzy.add_input_set(
            input, Box::new(LeftDeviation::left_fn));

        let middle = fuzzy.add_input_set(
            input, Box::new(LeftDeviation::middle_fn));

        let right = fuzzy.add_input_set(
            input, Box::new(LeftDeviation::right_fn));

        let far_right = fuzzy.add_input_set(
            input, Box::new(LeftDeviation::far_right_fn));

        LeftDeviation {
            input,
            far_left,
            left,
            middle,
            right,
            far_right,
        }
    }
}
