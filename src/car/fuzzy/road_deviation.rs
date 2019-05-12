use super::*;

impl RoadDeviation {
    fn far_left_fn(x: f32) -> f32 {
        let x1 = 0.1;
        let x2 = 0.25;
        if x < x1 {
            0.0
        }
        else if x < x2 {
            (x2 - x) / (x2 - x1)
        }
        else {
            0.0
        }
    }

    fn middle_left_fn(x: f32) -> f32 {
        let x1 = 0.1;
        let x2 = 0.25;
        let x3 = 0.4;

        if x < x1 {
            0.0
        }
        else if x < x2  {
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
        let x1 = 0.35;
        let x2 = 0.5;
        let x3 = 0.65;

        if x < x1 {
            0.0
        }
        else if x < x2  {
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
        let x2 = 0.65;
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
            input, Box::new(RoadDeviation::far_left_fn));

        let middle_left = fuzzy.add_input_set(
            input, Box::new(RoadDeviation::middle_left_fn));

        let middle = fuzzy.add_input_set(
            input, Box::new(RoadDeviation::middle_fn));

        let right = fuzzy.add_input_set(
            input, Box::new(RoadDeviation::right_fn));

        Self {
            input,
            far_left,
            middle_left,
            middle,
            right,
        }
    }
}
