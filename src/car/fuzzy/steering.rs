use super::*;

impl Steering {
    pub fn new(fuzzy: &mut Fuzzy) -> Self {
        let output = fuzzy.add_output(0.0, 1.0);

        let hard_right_fn = 
            |x| {
                if x > 0.75 {
                    1.0
                }
                else if x >= 0.6 {
                    6.66667 * x - 4.0
                }
                else {
                    0.0
                }
            };
        let hard_right = fuzzy.add_output_set(output, Box::new(hard_right_fn));

        let right_fn = 
            |x| {
                if x < 0.5 {
                    0.0
                }
                else if x <= 0.6 {
                    10.0 * x - 5.0
                }
                else if x <= 0.75 {
                    -6.66667 * x + 5.0
                }
                else {
                    0.0
                }
            };
        let right = fuzzy.add_output_set(output, Box::new(right_fn));

        let straight_fn = 
            |x| {
                if x < 0.4 {
                    0.0
                }
                else if x <= 0.5 {
                    10.0 * x - 4.0
                }
                else if x <= 0.6 {
                    -10.0 * x + 6.0
                }
                else {
                    0.0
                }
            };
        let straight = fuzzy.add_output_set(output, Box::new(straight_fn));

        let left_fn = 
            |x| {
                if x < 0.25 {
                    0.0
                }
                else if x <= 0.4 {
                    6.66667 * x - 1.66667
                }
                else if x <= 0.5 {
                    -10.0 * x + 5.0
                }
                else {
                    0.0
                }
            };
        let left = fuzzy.add_output_set(output, Box::new(left_fn));

        let hard_left_fn =
            |x| {
                if x < 0.25 {
                    1.0
                }
                else if x <= 0.4 {
                    -6.66667 * x + 2.66667
                }
                else {
                    0.0
                }
            };
        let hard_left = fuzzy.add_output_set(output, Box::new(hard_left_fn));

        Self {
            output,
            hard_right,
            right,
            straight,
            left,
            hard_left,
        }
    }
}
