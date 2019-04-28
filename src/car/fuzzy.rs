use crate::fuzzy::*;

pub struct Deviation {
    pub input: InputId,
    far_left: InputSetId,
    left: InputSetId,
    middle: InputSetId,
    right: InputSetId,
    far_right: InputSetId,
}

pub struct Steering {
    pub output: OutputId,
    hard_right: OutputSetId,
    right: OutputSetId,
    straight: OutputSetId,
    left: OutputSetId,
    hard_left: OutputSetId,
}

pub struct CarFuzzy {
    pub fuzzy: Fuzzy,
    pub deviation: Deviation,
    pub steering: Steering,
    pub simple_rule_set: RuleSetId,
}

impl Deviation {
    fn new(fuzzy: &mut Fuzzy) -> Self {
        let input = fuzzy.add_input(0.0, 1.0);

        let far_left_fn = 
            |x| {
                if x < 0.25 {
                    1.0
                }
                else if x <= 0.4 {
                    -6.6667 * x + 2.66667 
                }
                else {
                    0.0
                }
            };
        let far_left = fuzzy.add_input_set(input, Box::new(far_left_fn));

        let left_fn = 
            |x| {
                if x < 0.25 {
                    0.0
                }
                else if x <= 0.4 {
                    6.6667 * x - 1.6667
                }
                else if x <= 0.5 {
                    -10.0 * x + 5.0
                }
                else {
                    0.0
                }
            };
        let left = fuzzy.add_input_set(input, Box::new(left_fn));

        let middle_fn = 
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
        let middle = fuzzy.add_input_set(input, Box::new(middle_fn));

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
        let right = fuzzy.add_input_set(input, Box::new(right_fn));

        let far_right_fn = 
            |x| {
                if x < 0.6 {
                    0.0
                }
                else if x < 0.75 {
                    6.66667 * x - 4.0
                }
                else {
                    1.0
                }
            };
        let far_right = fuzzy.add_input_set(input, Box::new(far_right_fn));

        Deviation {
            input,
            far_left,
            left,
            middle,
            right,
            far_right,
        }
    }
}

impl Steering {
    fn new(fuzzy: &mut Fuzzy) -> Self {
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

impl CarFuzzy {
    pub fn new() -> Self {
        let mut fuzzy = Fuzzy::new();

        let deviation = Deviation::new(&mut fuzzy);
        let steering = Steering::new(&mut fuzzy);

        let rule1 = fuzzy.add_rule(&[deviation.far_left], steering.hard_right);
        let rule2 = fuzzy.add_rule(&[deviation.left], steering.right);
        let rule3 = fuzzy.add_rule(&[deviation.middle], steering.straight);
        let rule4 = fuzzy.add_rule(&[deviation.right], steering.left);
        let rule5 = fuzzy.add_rule(&[deviation.far_right], steering.hard_left);

        let simple_rule_set = fuzzy.add_rule_set(&[rule1, rule2, rule3, rule4, rule5]);

        Self {
            fuzzy,
            deviation,
            steering,
            simple_rule_set,
        }
    }
}
