use super::*;

impl CarDistance {

    fn near_fn(x: f32) -> f32 {
        let x1 = 10.0;
        let x2 = 40.0;
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
        let x1 = 10.0;
        let x2 = 20.0;
        let x3 = 50.0;
        let x4 = 70.0;
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
        let x1 = 40.0;
        let x2 = 70.0;
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

        let medium = fuzzy.add_input_set(
            input, Box::new(CarDistance::medium_fn));

        let far = fuzzy.add_input_set(
            input, Box::new(CarDistance::far_fn));

        Self {
            input,
            near,
            medium,
            far,
        }
    }
}
