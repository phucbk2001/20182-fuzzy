use super::*;

impl CarVeclocity {
    fn slow_fn(x: f32) -> f32 {
        let x1 = 2.0;
        let x2 = 3.0;
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
        let x1 = 2.0;
        let x2 = 3.0;
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
        let input = fuzzy.add_input(0.0, 30.0);

        let slow = fuzzy.add_input_set(
            input, Box::new(CarVeclocity::slow_fn));

        let medium = fuzzy.add_input_set(
            input, Box::new(CarVeclocity::medium_fn));

        Self {
            input,
            slow,
            medium,
        }
    }
}
