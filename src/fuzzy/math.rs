use super::*;

const INTEGRAL_STEP: usize = 20;

fn integral(x1: f32, x2: f32, y1: f32, y2: f32) -> f32 {
    let mut result = 
        (y1 * (x2 * x2 - x1 * x1) - x1 * (x2 + x1) * (y2 - y1)) / 2.0;
    result += 
        (x2 * x2 + x2 * x1 + x1 * x1) * (y2 - y1) / 3.0;
    result
}

impl Fuzzy {
    fn compute_input_membership(&self, rule: RuleId) -> f32 {
        let mut result = 1.0;
        for input_set in self.get_rule(rule).input_sets.iter() {
            result = f32::min(result, self.get_input_set(*input_set).membership);
        }
        result
    }

    fn collect_dirty_input_sets(&self, rule_set: RuleSetId) 
        -> Vec<InputSetId> 
    {
        let mut dirty_input_sets: Vec<InputSetId> =
            self.get_rule_set(rule_set).rules.iter()
                .flat_map(|rule| self.get_rule(*rule).input_sets.iter())
                .map(|item| *item)
                .collect();

        use std::cmp::Ordering;

        let ordering = |a: &InputSetId, b: &InputSetId| {
            if a.id < b.id {
                Ordering::Less
            }
            else if a.id > b.id {
                Ordering::Greater
            }
            else {
                Ordering::Equal
            }
        };

        dirty_input_sets.sort_by(ordering);
        dirty_input_sets.dedup_by(|a, b| a.id == b.id);

        dirty_input_sets
    }

    fn output_fuzzy_function(&self, output: &Output, x: f32) -> f32 {
        let mut max = 0.0;
        for output_set in output.cached_output_sets.iter() {
            let output_set_result = f32::min(
                self.get_output_set(*output_set).input_membership,
                (self.get_output_set(*output_set).f)(x)
            );
            max = f32::max(max, output_set_result);
        }
        max
    }

    fn defuzzificate(&self, output: &Output) -> f32 {
        let min = output.min;
        let max = output.max;

        let mut x1: f32 = min;
        let mut x2: f32;

        let mut nominator: f32 = 0.0;
        let mut denominator: f32 = 0.0;

        for i in 0..INTEGRAL_STEP {
            x2 = (i + 1) as f32 * (max - min) / (INTEGRAL_STEP as f32) + min;
            let y1 = self.output_fuzzy_function(output, x1);
            let y2 = self.output_fuzzy_function(output, x2);

            nominator += integral(x1, x2, y1, y2);
            denominator += (x2 - x1) * (y1 + y2) / 2.0;

            x1 = x2;
        }

        nominator / denominator
    }

    pub fn evaluate(&mut self, rule_set: RuleSetId) {
        let dirty_input_sets = self.collect_dirty_input_sets(rule_set);
        for input_set in dirty_input_sets.iter() {
            self.get_input_set_mut(*input_set).membership = {
                let f = &self.get_input_set(*input_set).f;
                let input = self.get_input_set(*input_set).input;
                let value = self.get_input(input);
                f(value)
            };
        }

        let rules: Vec<RuleId> = self.get_rule_set(rule_set).rules.clone();

        for output in self.outputs.iter_mut() {
            output.cached_output_sets.clear();
        }

        for rule in rules.iter() {
            let input_membership = self.compute_input_membership(*rule);
            let output_set = self.get_rule(*rule).output_set;
            self.get_output_set_mut(output_set)
                .input_membership = input_membership;
            self.get_output_mut(self.get_output_set(output_set).output)
                .cached_output_sets.push(output_set);
        }

        let output_results: Vec<(OutputId, f32)> =
            self.outputs.iter().enumerate()
                .filter(|(_index, output)| !output.cached_output_sets.is_empty())
                .map(|(id, output)| (OutputId { id }, self.defuzzificate(output)))
                .collect();

        for result in output_results.iter() {
            let (output, value) = *result;
            self.get_output_mut(output).value = value;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use approx::assert_relative_eq;

    #[test]
    fn test_input_memberships() {
        let mut fuzzy = Fuzzy::new();
        let i1 = fuzzy.add_input(0.0, 4.0);
        let i2 = fuzzy.add_input(0.0, 2.0);

        let o1 = fuzzy.add_output(0.0, 1.0);
        let o2 = fuzzy.add_output(0.0, 2.0);

        let is1 = fuzzy.add_input_set(
            i1, Box::new(|x| x / 4.0));
        let is2 = fuzzy.add_input_set(
            i2, Box::new(|x| x / 2.0));
        let is3 = fuzzy.add_input_set(
            i1, Box::new(|x| (4.0 - x) / 4.0));
        let is4 = fuzzy.add_input_set(
            i2, Box::new(|x| (2.0 - x) / 2.0));

        let os1 = fuzzy.add_output_set(
            o1, Box::new(|x| x));
        let os2 = fuzzy.add_output_set(
            o1, Box::new(|x| 1.0 - x));

        let r1 = fuzzy.add_rule(&[is1, is2], os1);
        let r2 = fuzzy.add_rule(&[is3, is4], os2);
        let rs1 = fuzzy.add_rule_set(&[r1, r2]);

        fuzzy.set_input(i1, 1.0);
        fuzzy.set_input(i2, 2.0);

        fuzzy.evaluate(rs1);

        assert_relative_eq!(fuzzy.get_input_set(is1).membership, 0.25);
        assert_relative_eq!(fuzzy.get_input_set(is2).membership, 1.0);
        assert_relative_eq!(fuzzy.get_input_set(is3).membership, 0.75);
        assert_relative_eq!(fuzzy.get_input_set(is4).membership, 0.0);

        assert_relative_eq!(fuzzy.get_output_set(os1).input_membership, 0.25);
        assert_relative_eq!(fuzzy.get_output_set(os2).input_membership, 0.0);
    
        assert_relative_eq!(
            fuzzy.output_fuzzy_function(&fuzzy.outputs[o1.id], 0.25),
            0.25
        );
        assert_relative_eq!(
            fuzzy.output_fuzzy_function(&fuzzy.outputs[o1.id], 0.5),
            0.25
        );
        assert_relative_eq!(
            fuzzy.output_fuzzy_function(&fuzzy.outputs[o1.id], 0.125),
            0.125
        );
        
        assert_relative_eq!(fuzzy.get_output(o1), 0.5595238);
        assert_relative_eq!(fuzzy.get_output(o2), 0.0);
    }

    #[test]
    fn test_integral() {
        let s = (2.0 + 3.0) / 2.0;
        assert_relative_eq!(integral(0.0, 1.0, 2.0, 3.0) / s, 0.53333336);
    }
}
