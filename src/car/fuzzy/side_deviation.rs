use super::*;

impl SideDeviation {
    fn back_fn(x: f32) -> f32 {
        let x1 = -2.5;
        let x2 = -1.5;

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

    fn not_back_fn(x: f32) -> f32 {
        let x1 = -2.5;
        let x2 = -1.5;
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
        let input = fuzzy.add_input(-100.0, 100.0);

        let back = fuzzy.add_input_set(
            input, Box::new(SideDeviation::back_fn));

        let not_back = fuzzy.add_input_set(
            input, Box::new(SideDeviation::not_back_fn));

        Self {
            input,
            back,
            not_back,
        }
    }
}
