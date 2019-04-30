use super::*;

const INTEGRAL_STEP: usize = 40;

fn integral(x1: f32, x2: f32, y1: f32, y2: f32) -> f32 {
    let mut result = 
        (y1 * (x2 * x2 - x1 * x1) - x1 * (x2 + x1) * (y2 - y1)) / 2.0;
    result += 
        (x2 * x2 + x2 * x1 + x1 * x1) * (y2 - y1) / 3.0;
    result
}

fn compute_input_membership(
    rules: &Vec<Rule>,
    input_sets: &Vec<InputSet>,
    rule: RuleId) 
    -> f32 
{
    let mut result = 1.0;
    for input_set in rules[rule.id].input_sets.iter() {
        result = f32::min(result, input_sets[input_set.id].membership);
    }
    result
}

fn collect_dirty_input_sets(
    rule_sets: &Vec<RuleSet>,
    rules: &Vec<Rule>,
    rule_set: RuleSetId) 
    -> Vec<InputSetId> 
{
    let mut dirty_input_sets: Vec<InputSetId> =
        rule_sets[rule_set.id].rules.iter()
            .flat_map(|rule| rules[rule.id].input_sets.iter())
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

fn output_fuzzy_function(
    output_sets: &Vec<OutputSet>,
    output: &Output, x: f32)
    -> f32
{
    let mut max = 0.0;
    for output_set in output.cached_output_sets.iter() {
        let output_set_result = f32::min(
            output_sets[output_set.id].input_membership,
            (output_sets[output_set.id].f)(x)
        );
        max = f32::max(max, output_set_result);
    }
    max
}

fn defuzzificate(
    output_sets: &Vec<OutputSet>,
    output: &Output) 
    -> f32 
{
    let min = output.min;
    let max = output.max;

    let mut x1: f32 = min;

    let mut nominator: f32 = 0.0;
    let mut denominator: f32 = 0.0;

    for i in 0..INTEGRAL_STEP {
        let x2 = (i + 1) as f32 * (max - min) / (INTEGRAL_STEP as f32) + min;

        let y1 = output_fuzzy_function(
            output_sets, output, x1);
        let y2 = output_fuzzy_function(
            output_sets, output, x2);

        nominator += integral(x1, x2, y1, y2);
        denominator += (x2 - x1) * (y1 + y2) / 2.0;

        x1 = x2;
    }

    nominator / denominator
}

impl Fuzzy {
    pub fn evaluate(&mut self, rule_set: RuleSetId) {
        let inputs = &self.inputs;
        let input_sets = &mut self.input_sets;
        let outputs = &mut self.outputs;
        let output_sets = &mut self.output_sets;
        let rules = &mut self.rules;
        let rule_sets = &self.rule_sets;

        let dirty_input_sets = 
            collect_dirty_input_sets(
                rule_sets, rules, rule_set);

        for input_set in dirty_input_sets.iter() {
            input_sets[input_set.id].membership = {
                let f = &input_sets[input_set.id].f;
                let input = input_sets[input_set.id].input;
                let value = inputs[input.id].value;
                f(value)
            };
        }

        let active_rules: Vec<RuleId> = rule_sets[rule_set.id].rules.clone();

        for output in outputs.iter_mut() {
            output.cached_output_sets.clear();
        }

        for rule in active_rules.iter() {
            let output_set = rules[rule.id].output_set;
            output_sets[output_set.id].input_membership = 0.0;
        }

        for rule in active_rules.iter() {
            let input_membership = 
                compute_input_membership(rules, input_sets, *rule);

            let output_set = rules[rule.id].output_set;

            output_sets[output_set.id].set_input_membership(input_membership);

            outputs[output_sets[output_set.id].output.id]
                .cached_output_sets.push(output_set);
        }

        for output in outputs.iter_mut() {
            use std::cmp::Ordering;

            let ordering = |a: &OutputSetId, b: &OutputSetId| {
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

            output.cached_output_sets.sort_by(ordering);
            output.cached_output_sets.dedup_by(|a, b| a.id == b.id);
        }

        let output_results: Vec<(OutputId, f32)> =
            outputs.iter().enumerate()
                .filter(|(_index, output)| !output.cached_output_sets.is_empty())
                .map(|(id, output)| (OutputId { id }, defuzzificate(output_sets, output)))
                .collect();

        for result in output_results.iter() {
            let (output, value) = *result;
            outputs[output.id].value = value;
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

        assert_relative_eq!(fuzzy.input_sets[is1.id].membership, 0.25);
        assert_relative_eq!(fuzzy.input_sets[is2.id].membership, 1.0);
        assert_relative_eq!(fuzzy.input_sets[is3.id].membership, 0.75);
        assert_relative_eq!(fuzzy.input_sets[is4.id].membership, 0.0);

        assert_relative_eq!(fuzzy.output_sets[os1.id].input_membership, 0.25);
        assert_relative_eq!(fuzzy.output_sets[os2.id].input_membership, 0.0);
    
        assert_relative_eq!(
            output_fuzzy_function(
                &fuzzy.output_sets, &fuzzy.outputs[o1.id], 0.25),
            0.25
        );
        assert_relative_eq!(
            output_fuzzy_function(
                &fuzzy.output_sets, &fuzzy.outputs[o1.id], 0.5),
            0.25
        );
        assert_relative_eq!(
            output_fuzzy_function(
                &fuzzy.output_sets, &fuzzy.outputs[o1.id], 0.125),
            0.125
        );
        
        assert_relative_eq!(fuzzy.outputs[o1.id].value, 0.5595238);
        assert_relative_eq!(fuzzy.outputs[o2.id].value, 0.0);
    }

    #[test]
    fn test_integral() {
        let s = (2.0 + 3.0) / 2.0;
        assert_relative_eq!(integral(0.0, 1.0, 2.0, 3.0) / s, 0.53333336);
    }
}
