use super::*;

impl CarDistance {

    fn near_fn(x: f32) -> f32 {
        let x1 = 5.0;
        let x2 = 10.0;
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

    fn near_medium_fn(x: f32) -> f32 {
        let x1 = 10.0;
        let x2 = 20.0;
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

    fn medium_fn(x: f32) -> f32 {
        let x1 = 5.0;
        let x2 = 10.0;
        let x3 = 25.0;
        let x4 = 30.0;
        if x < x1 {
            0.0
        }
        else if x < x2 {
            (x - x1) / (x2 - x1)
        }
        else if x < x3 {
            1.0
        }
        else if x < x4 {
            (x4 - x) / (x4 - x3)
        }
        else {
            0.0
        }
    }

    fn far_fn(x: f32) -> f32 {
        let x1 = 20.0;
        let x2 = 25.0;
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

    fn medium_far_fn(x: f32) -> f32 {
        let x1 = 5.0;
        let x2 = 10.0;
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
        let input = fuzzy.add_input(0.0, 200.0);

        let near = fuzzy.add_input_set(
            input, Box::new(CarDistance::near_fn));

        let near_medium = fuzzy.add_input_set(
            input, Box::new(CarDistance::near_medium_fn));

        let medium = fuzzy.add_input_set(
            input, Box::new(CarDistance::medium_fn));

        let far = fuzzy.add_input_set(
            input, Box::new(CarDistance::far_fn));

        let medium_far = fuzzy.add_input_set(
            input, Box::new(CarDistance::medium_far_fn));

        Self {
            input,
            near,
            near_medium,
            medium,
            far,
            medium_far,
        }
    }
}
