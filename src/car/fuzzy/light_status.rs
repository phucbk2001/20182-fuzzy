use super::*;

impl LightStatus {
    fn green_fn(x: f32) -> f32 {
        let x1 = 1.0;
        let x2 = 2.0;
        let x3 = 7.5;
        let x4 = 8.0;

        if x < x1 {
            1.0
        }
        else if x < x2 {
            (x2 - x) / (x2 - x1)
        }
        else if x < x3 {
            0.0
        }
        else if x < x4 {
            (x - x3) / (x4 - x3)
        }
        else {
            1.0
        }
    }

    fn less_green_fn(x: f32) -> f32 {
        let x1 = 1.0;
        let x2 = 2.0;
        let x3 = 3.0;

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

    fn yellow_fn(x: f32) -> f32 {
        let x1 = 2.0;
        let x2 = 3.0;
        let x3 = 4.0;

        let x4 = 6.0;
        let x5 = 7.0;
        let x6 = 8.0;

        if x < x1 {
            0.0
        }
        else if x < x2 {
            (x - x1) / (x2 - x1)
        }
        else if x < x3 {
            (x3 - x) / (x3 - x2)
        }
        else if x < x4 {
            0.0
        }
        else if x < x5 {
            (x - x4) / (x5 - x4)
        }
        else if x < x6 {
            (x6 - x) / (x6 - x5)
        }
        else {
            0.0
        }
    }

    fn red_fn(x: f32) -> f32 {
        let x1 = 3.5;
        let x2 = 4.0;
        let x3 = 5.0;
        let x4 = 6.0;

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

    fn less_red_fn(x: f32) -> f32 {
        let x1 = 5.0;
        let x2 = 6.0;
        let x3 = 7.0;

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

    pub fn new(fuzzy: &mut Fuzzy) -> Self {
        let input = fuzzy.add_input(0.0, 8.0);

        let green = fuzzy.add_input_set(
            input, Box::new(LightStatus::green_fn));

        let less_green = fuzzy.add_input_set(
            input, Box::new(LightStatus::less_green_fn));

        let yellow = fuzzy.add_input_set(
            input, Box::new(LightStatus::yellow_fn));
        
        let red = fuzzy.add_input_set(
            input, Box::new(LightStatus::red_fn));

        let less_red = fuzzy.add_input_set(
            input, Box::new(LightStatus::less_red_fn));

        Self {
            input,
            green,
            less_green,
            yellow,
            red,
            less_red,
        }
    }
}
