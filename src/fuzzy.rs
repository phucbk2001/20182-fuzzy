mod math;

type MembershipFunction = Box<dyn Fn(f32) -> f32>;

#[derive(Copy, Clone)]
pub struct InputId { id: usize }

#[derive(Copy, Clone)]
pub struct OutputId { id: usize }

#[derive(Copy, Clone)]
pub struct InputSetId { id: usize }

#[derive(Copy, Clone)]
pub struct OutputSetId { id: usize }

#[derive(Copy, Clone)]
pub struct RuleId { id: usize }

#[derive(Copy, Clone)]
pub struct RuleSetId { id: usize }

pub struct Input {
    #[allow(dead_code)]
    min: f32,
    #[allow(dead_code)]
    max: f32,
    value: f32,
}

pub struct Output {
    min: f32,
    max: f32,
    value: f32,
    cached_output_sets: Vec<OutputSetId>,
}

pub struct InputSet {
    input: InputId,
    f: MembershipFunction,
    membership: f32,
}

pub struct OutputSet {
    output: OutputId,
    f: MembershipFunction,
    input_membership: Option<f32>,
}

pub struct Rule {
    input_sets: Vec<InputSetId>,
    output_set: OutputSetId,
}

pub struct RuleSet {
    rules: Vec<RuleId>,
}

pub struct Fuzzy {
    inputs: Vec<Input>,
    outputs: Vec<Output>,
    input_sets: Vec<InputSet>,
    output_sets: Vec<OutputSet>,
    rules: Vec<Rule>,
    rule_sets: Vec<RuleSet>,
}

impl OutputSet {
    fn set_input_membership(&mut self, membership: f32) {
        self.input_membership = match self.input_membership {
            None => Some(membership),
            Some(value) => Some(f32::max(membership, value)),
        };
    }
}

impl Fuzzy {
    pub fn new() -> Self {
        Self {
            inputs: Vec::new(),
            outputs: Vec::new(),
            input_sets: Vec::new(),
            output_sets: Vec::new(),
            rules: Vec::new(),
            rule_sets: Vec::new(),
        }
    }
}

impl Fuzzy {
    pub fn add_input(&mut self, min: f32, max: f32) -> InputId {
        let input = Input {
            min,
            max,
            value: 0.0,
        };
        let id = self.inputs.len();
        self.inputs.push(input);
        InputId { id }
    }

    pub fn set_input(&mut self, id: InputId, value: f32) {
        debug_assert!(id.id < self.inputs.len());
        self.inputs[id.id].value = value;
    }
}

impl Fuzzy {
    pub fn add_output(&mut self, min: f32, max: f32) -> OutputId {
        let output = Output {
            min,
            max,
            value: 0.0,
            cached_output_sets: Vec::new(),
        };
        let id = self.outputs.len();
        self.outputs.push(output);
        OutputId { id }
    }

    pub fn get_output(&self, id: OutputId) -> f32 {
        debug_assert!(id.id < self.outputs.len());
        self.outputs[id.id].value
    }
}

impl Fuzzy {
    pub fn add_input_set(
        &mut self, input: InputId, f: MembershipFunction) 
        -> InputSetId 
    {
        let input_set = InputSet {
            input,
            f,
            membership: 0.0,
        };

        let id = self.input_sets.len();
        self.input_sets.push(input_set);
        InputSetId { id }
    }
}

impl Fuzzy {
    pub fn add_output_set(
        &mut self, output: OutputId, f: MembershipFunction) 
        -> OutputSetId 
    {
        let output_set = OutputSet {
            output,
            f, 
            input_membership: None,
        };
        let id = self.output_sets.len();
        self.output_sets.push(output_set);
        OutputSetId { id }
    }
}

impl Fuzzy {
    pub fn add_rule(
        &mut self, input_sets: &[InputSetId], 
        output_set: OutputSetId) 
        -> RuleId 
    {
        let rule = Rule {
            input_sets: input_sets.to_vec(),
            output_set,
        };
        let id = self.rules.len();
        self.rules.push(rule);
        RuleId { id }
    }
}

impl Fuzzy {
    pub fn add_rule_set(&mut self, rules: &[RuleId]) -> RuleSetId {
        let rule_set = RuleSet {
            rules: rules.to_vec(),
        };
        let id = self.rule_sets.len();
        self.rule_sets.push(rule_set);
        RuleSetId { id }
    }
}

#[cfg(test)]
mod tests {
}
