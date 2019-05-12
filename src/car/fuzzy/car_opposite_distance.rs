use super::*;

impl CarOppositeDistance {
    fn near_fn(x: f32) -> f32 {
        let x1 = 30.0;
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

    fn far_fn(x: f32) -> f32 {
        let x1 = 30.0;
        let x2 = 50.0;
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
        let input = fuzzy.add_input(0.0, 400.0);

        let near = fuzzy.add_input_set(
            input, Box::new(CarOppositeDistance::near_fn));

        let far = fuzzy.add_input_set(
            input, Box::new(CarOppositeDistance::far_fn));

        Self {
            input,
            near,
            far,
        }
    }
}
