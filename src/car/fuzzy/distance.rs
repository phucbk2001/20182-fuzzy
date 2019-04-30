use super::*;

impl Distance {
    fn near_fn(x: f32) -> f32 {
        let x1 = 1.5;
        let x2 = 5.0;
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
        let x1 = 4.0;
        let x2 = 10.0;
        let x3 = 20.0;
        let x4 = 25.0;

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
        let x2 = 35.0;

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
        let input = fuzzy.add_input(0.0, 50.0);

        let near = fuzzy.add_input_set(
            input, Box::new(Distance::near_fn));

        let medium = fuzzy.add_input_set(
            input, Box::new(Distance::medium_fn));

        let far = fuzzy.add_input_set(
            input, Box::new(Distance::far_fn));

        Self {
            input,
            near,
            medium,
            far,
        }
    }
}
