mod deviation;
mod steering;
mod distance;
mod speed;
mod light_status;

use crate::fuzzy::*;

pub struct Deviation {
    pub input: InputId,
    far_left: InputSetId,
    left: InputSetId,
    middle: InputSetId,
    right: InputSetId,
    far_right: InputSetId,
}

pub struct Distance {
    pub input: InputId,
    near: InputSetId,
    medium: InputSetId,
    far: InputSetId,
}

pub struct LightStatus {
    pub input: InputId,
    green: InputSetId,
    less_green: InputSetId,
    yellow: InputSetId,
    less_red: InputSetId,
    red: InputSetId,
}

// Output Fuzzy Sets

pub struct Steering {
    pub output: OutputId,
    hard_right: OutputSetId,
    right: OutputSetId,
    straight: OutputSetId,
    left: OutputSetId,
    hard_left: OutputSetId,
}

pub struct Speed {
    pub output: OutputId,
    stop: OutputSetId,
    slower: OutputSetId,
    slow: OutputSetId,
    medium: OutputSetId,
}

pub struct CarFuzzy {
    pub fuzzy: Fuzzy,

    pub deviation: Deviation,
    pub steering: Steering,
    pub distance: Distance,
    pub speed: Speed,
    pub light_status: LightStatus,

    pub simple_rule_set: RuleSetId,
}


impl CarFuzzy {
    pub fn new() -> Self {
        let mut fuzzy = Fuzzy::new();

        let deviation = Deviation::new(&mut fuzzy);
        let steering = Steering::new(&mut fuzzy);
        let distance = Distance::new(&mut fuzzy);
        let speed = Speed::new(&mut fuzzy);
        let light_status = LightStatus::new(&mut fuzzy);

        let rule1 = fuzzy.add_rule(&[deviation.far_left], steering.hard_right);
        let rule2 = fuzzy.add_rule(&[deviation.left], steering.right);
        let rule3 = fuzzy.add_rule(&[deviation.middle], steering.straight);
        let rule4 = fuzzy.add_rule(&[deviation.right], steering.left);
        let rule5 = fuzzy.add_rule(&[deviation.far_right], steering.hard_left);

        let rule6 = fuzzy.add_rule(&[light_status.green, deviation.middle], speed.medium);
        let rule7 = fuzzy.add_rule(&[light_status.green, deviation.left], speed.slow);
        let rule8 = fuzzy.add_rule(&[light_status.green, deviation.right], speed.slow);
        let rule9 = fuzzy.add_rule(&[light_status.green, deviation.far_left], speed.slower);
        let rule10 = fuzzy.add_rule(&[light_status.green, deviation.far_right], speed.slower);

        let rule11 = fuzzy.add_rule(&[distance.far, deviation.middle], speed.medium);
        let rule12 = fuzzy.add_rule(&[distance.far, deviation.left], speed.slow);
        let rule13 = fuzzy.add_rule(&[distance.far, deviation.right], speed.slow);
        let rule14 = fuzzy.add_rule(&[distance.far, deviation.far_left], speed.slower);
        let rule15 = fuzzy.add_rule(&[distance.far, deviation.far_right], speed.slower);

        let rule16 = fuzzy.add_rule(&[light_status.yellow, distance.medium, deviation.middle], speed.slow);
        let rule17 = fuzzy.add_rule(&[light_status.yellow, distance.medium, deviation.left], speed.slower);
        let rule18 = fuzzy.add_rule(&[light_status.yellow, distance.medium, deviation.right], speed.slower);
        let rule19 = fuzzy.add_rule(&[light_status.yellow, distance.medium, deviation.far_left], speed.slower);
        let rule20 = fuzzy.add_rule(&[light_status.yellow, distance.medium, deviation.far_right], speed.slower);

        let rule21 = fuzzy.add_rule(&[light_status.yellow, distance.near], speed.stop);

        let rule22 = fuzzy.add_rule(&[light_status.red, distance.medium, deviation.middle], speed.slow);
        let rule23 = fuzzy.add_rule(&[light_status.red, distance.medium, deviation.left], speed.slower);
        let rule24 = fuzzy.add_rule(&[light_status.red, distance.medium, deviation.right], speed.slower);
        let rule25 = fuzzy.add_rule(&[light_status.red, distance.medium, deviation.far_left], speed.slower);
        let rule26 = fuzzy.add_rule(&[light_status.red, distance.medium, deviation.far_right], speed.slower);

        let rule27 = fuzzy.add_rule(&[light_status.red, distance.near], speed.stop);

        let rule28 = fuzzy.add_rule(&[light_status.less_green, distance.medium, deviation.middle], speed.medium);
        let rule29 = fuzzy.add_rule(&[light_status.less_green, distance.medium, deviation.left], speed.slow);
        let rule30 = fuzzy.add_rule(&[light_status.less_green, distance.medium, deviation.right], speed.slow);
        let rule31 = fuzzy.add_rule(&[light_status.less_green, distance.medium, deviation.far_left], speed.slower);
        let rule32 = fuzzy.add_rule(&[light_status.less_green, distance.medium, deviation.far_right], speed.slower);

        let rule33 = fuzzy.add_rule(&[light_status.less_green, distance.near, deviation.middle], speed.slower);
        let rule34 = fuzzy.add_rule(&[light_status.less_green, distance.near, deviation.left], speed.slower);
        let rule35 = fuzzy.add_rule(&[light_status.less_green, distance.near, deviation.right], speed.slower);
        let rule36 = fuzzy.add_rule(&[light_status.less_green, distance.near, deviation.far_left], speed.stop);
        let rule37 = fuzzy.add_rule(&[light_status.less_green, distance.near, deviation.far_right], speed.stop);

        let rule38 = fuzzy.add_rule(&[light_status.less_red, distance.medium, deviation.middle], speed.slow);
        let rule39 = fuzzy.add_rule(&[light_status.less_red, distance.medium, deviation.left], speed.slower);
        let rule40 = fuzzy.add_rule(&[light_status.less_red, distance.medium, deviation.right], speed.slower);
        let rule41 = fuzzy.add_rule(&[light_status.less_red, distance.medium, deviation.far_left], speed.slower);
        let rule42 = fuzzy.add_rule(&[light_status.less_red, distance.medium, deviation.far_right], speed.slower);
        let rule43 = fuzzy.add_rule(&[light_status.less_red, distance.near, deviation.far_right], speed.stop);

        let simple_rule_set = fuzzy.add_rule_set(
            &[
                rule1, rule2, rule3, rule4, rule5,
                rule6, rule7, rule8, rule9, rule10,
                rule11, rule12, rule13, rule14, rule15,
                rule16, rule17, rule18, rule19, rule20,
                rule21, rule22, rule23, rule24, rule25, rule26,
                rule27, rule28, rule29, rule30, rule31, rule32,
                rule33, rule34, rule35, rule36, rule37,
                rule38, rule39, rule40, rule41, rule42, rule43,
            ]);

        Self {
            fuzzy,

            deviation,
            steering,
            distance,
            speed,
            light_status,

            simple_rule_set,
        }
    }
}
