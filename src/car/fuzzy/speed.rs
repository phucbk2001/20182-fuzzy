use super::*;

impl Speed {
    fn stop_fn(x: f32) -> f32 {
        let x1 = 0.0;
        let x2 = 0.05;

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

    fn slower_fn(x: f32) -> f32 {
        let x1 = 0.025;
        let x2 = 0.25;
        let x3 = 0.5;

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

    fn slow_fn(x: f32) -> f32 {
        let x1 = 0.3;
        let x2 = 0.6;
        let x3 = 0.8;

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

    fn medium_fn(x: f32) -> f32 {
        let x1 = 0.7;
        let x2 = 0.9;

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
        let output = fuzzy.add_output(0.0, 1.0);

        let stop = fuzzy.add_output_set(
            output, Box::new(Speed::stop_fn));

        let slower = fuzzy.add_output_set(
            output, Box::new(Speed::slower_fn));

        let slow = fuzzy.add_output_set(
            output, Box::new(Speed::slow_fn));

        let medium = fuzzy.add_output_set(
            output, Box::new(Speed::medium_fn));

        Self {
            output,
            stop,
            slower,
            slow,
            medium,
        }
    }
}
