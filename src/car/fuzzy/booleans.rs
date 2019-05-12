use super::*;

impl GoLeftLane {
    pub fn new(fuzzy: &mut Fuzzy) -> Self {
        let output = fuzzy.add_output(0.0, 1.0);

        let true_ = fuzzy.add_output_set(
            output, Box::new(true_fn));

        let false_ = fuzzy.add_output_set(
            output, Box::new(false_fn));

        Self {
            output,
            true_,
            false_,
        }
    }
}

impl StayLeftLane {
    pub fn new(fuzzy: &mut Fuzzy) -> Self {
        let output = fuzzy.add_output(0.0, 1.0);

        let true_ = fuzzy.add_output_set(
            output, Box::new(true_fn));

        let false_ = fuzzy.add_output_set(
            output, Box::new(false_fn));

        Self {
            output,
            true_,
            false_,
        }
    }
}

impl BackToRightLane {
    pub fn new(fuzzy: &mut Fuzzy) -> Self {
        let output = fuzzy.add_output(0.0, 1.0);

        let true_ = fuzzy.add_output_set(
            output, Box::new(true_fn));

        let false_ = fuzzy.add_output_set(
            output, Box::new(false_fn));

        Self {
            output,
            true_,
            false_,
        }
    }
}

impl GoNormal {
    pub fn new(fuzzy: &mut Fuzzy) -> Self {
        let output = fuzzy.add_output(0.0, 1.0);

        let true_ = fuzzy.add_output_set(
            output, Box::new(true_fn));

        let false_ = fuzzy.add_output_set(
            output, Box::new(false_fn));

        Self {
            output,
            true_,
            false_,
        }
    }
}
