mod deviation;
mod steering;
mod distance;
mod speed;
mod light_status;
mod car_distance;
mod car_velocity;
mod car_opposite_distance;
mod car_opposite_velocity;
mod road_deviation;
mod booleans;
mod left_deviation;
mod side_deviation;

use crate::fuzzy::*;

fn true_fn(x: f32) -> f32 {
    let x1 = 0.3;
    let x2 = 0.7;

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

fn false_fn(x: f32) -> f32 {
    let x1 = 0.3;
    let x2 = 0.7;
    if x < x1 {
        1.0
    }
    else if x < x2 {
        (x2 - x1) / (x2 - x1)
    }
    else {
        0.0
    }
}

pub struct Deviation {
    pub input: InputId,
    far_left: InputSetId,
    left: InputSetId,
    middle: InputSetId,
    right: InputSetId,
    far_right: InputSetId,
}

pub struct LeftDeviation {
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

pub struct CarDistance {
    pub input: InputId,
    near: InputSetId,
    near_medium: InputSetId,
    medium: InputSetId,
    far: InputSetId,
    medium_far: InputSetId,
}

pub struct CarVeclocity {
    pub input: InputId,
    slow: InputSetId,
    #[allow(dead_code)]
    medium: InputSetId,
}

pub struct CarOppositeDistance {
    pub input: InputId,
    near: InputSetId,
    far: InputSetId,
}

pub struct CarOppositeVelocity {
    pub input: InputId,
    #[allow(dead_code)]
    slow: InputSetId,
    #[allow(dead_code)]
    medium: InputSetId,
}

pub struct RoadDeviation {
    pub input: InputId,
    far_left: InputSetId,
    middle_left: InputSetId,
    left: InputSetId,
    middle: InputSetId,
    right: InputSetId,
    middle_right: InputSetId,
    far_right: InputSetId,
}

pub struct SideDeviation {
    pub input: InputId,
    back: InputSetId,
    not_back: InputSetId,
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

pub struct GoLeftLane {
    pub output: OutputId,
    true_: OutputSetId,
    false_: OutputSetId,
}

pub struct StayLeftLane {
    pub output: OutputId,
    true_: OutputSetId,
    false_: OutputSetId,
}

pub struct BackToRightLane {
    pub output: OutputId,
    true_: OutputSetId,
    false_: OutputSetId,
}

pub struct GoNormal {
    pub output: OutputId,
    true_: OutputSetId,
    false_: OutputSetId,
}

pub struct CarFuzzy {
    pub fuzzy: Fuzzy,

    pub deviation: Deviation,
    pub steering: Steering,
    pub distance: Distance,
    pub speed: Speed,
    pub light_status: LightStatus,
    pub car_distance: CarDistance,
    pub car_velocity: CarVeclocity,
    pub car_opposite_distance: CarOppositeDistance,
    pub car_opposite_velocity: CarOppositeVelocity,
    pub road_deviation: RoadDeviation,
    pub left_deviation: LeftDeviation,
    pub side_deviation: SideDeviation,

    pub go_left_lane: GoLeftLane,
    pub stay_left_lane: StayLeftLane,
    pub back_to_right_lane: BackToRightLane,
    pub go_normal: GoNormal,

    pub simple_rule_set: RuleSetId,
    pub normal_rule_set: RuleSetId,
    pub go_left_lane_rule_set: RuleSetId,
    pub stay_left_lane_rule_set: RuleSetId,
    pub back_to_right_lane_rule_set: RuleSetId,
}


impl CarFuzzy {
    pub fn new() -> Self {
        let mut fuzzy = Fuzzy::new();

        let deviation = Deviation::new(&mut fuzzy);
        let steering = Steering::new(&mut fuzzy);
        let distance = Distance::new(&mut fuzzy);
        let speed = Speed::new(&mut fuzzy);
        let light_status = LightStatus::new(&mut fuzzy);
        let car_distance = CarDistance::new(&mut fuzzy);
        let car_velocity = CarVeclocity::new(&mut fuzzy);
        let car_opposite_distance = CarOppositeDistance::new(&mut fuzzy);
        let car_opposite_velocity = CarOppositeVelocity::new(&mut fuzzy);
        let road_deviation = RoadDeviation::new(&mut fuzzy);
        let go_left_lane = GoLeftLane::new(&mut fuzzy);
        let stay_left_lane = StayLeftLane::new(&mut fuzzy);
        let back_to_right_lane = BackToRightLane::new(&mut fuzzy);
        let go_normal = GoNormal::new(&mut fuzzy);
        let left_deviation = LeftDeviation::new(&mut fuzzy);
        let side_deviation = SideDeviation::new(&mut fuzzy);

        let rule1 = fuzzy.add_rule(&[deviation.far_left], steering.hard_right);
        let rule2 = fuzzy.add_rule(&[deviation.left], steering.right);
        let rule3 = fuzzy.add_rule(&[deviation.middle], steering.straight);
        let rule4 = fuzzy.add_rule(&[deviation.right], steering.left);
        let rule5 = fuzzy.add_rule(&[deviation.far_right], steering.hard_left);

        let rule6 = fuzzy.add_rule(&[light_status.green, deviation.middle, car_distance.far], speed.medium);
        let rule7 = fuzzy.add_rule(&[light_status.green, deviation.left, car_distance.far], speed.slow);
        let rule7b = fuzzy.add_rule(&[light_status.green, deviation.left, car_distance.medium], speed.slow);
        let rule8 = fuzzy.add_rule(&[light_status.green, deviation.right, car_distance.far], speed.slow);
        let rule8b = fuzzy.add_rule(&[light_status.green, deviation.right, car_distance.medium], speed.slow);
        let rule9 = fuzzy.add_rule(&[light_status.green, deviation.far_left, car_distance.far], speed.slower);
        let rule9b = fuzzy.add_rule(&[light_status.green, deviation.far_left, car_distance.medium], speed.slower);
        let rule10 = fuzzy.add_rule(&[light_status.green, deviation.far_right, car_distance.far], speed.slower);
        let rule10b = fuzzy.add_rule(&[light_status.green, deviation.far_right, car_distance.medium], speed.slower);

        let rule11 = fuzzy.add_rule(&[distance.far, deviation.middle, car_distance.far], speed.medium);
        let rule12 = fuzzy.add_rule(&[distance.far, deviation.left, car_distance.far], speed.slow);
        let rule12b = fuzzy.add_rule(&[distance.far, deviation.left, car_distance.medium], speed.slower);
        let rule13 = fuzzy.add_rule(&[distance.far, deviation.right, car_distance.far], speed.slow);
        let rule13b = fuzzy.add_rule(&[distance.far, deviation.right, car_distance.medium], speed.slower);
        let rule14 = fuzzy.add_rule(&[distance.far, deviation.far_left, car_distance.medium_far], speed.slower);
        let rule15 = fuzzy.add_rule(&[distance.far, deviation.far_right, car_distance.medium_far], speed.slower);

        let rule16 = fuzzy.add_rule(&[light_status.yellow, distance.medium, deviation.middle, car_distance.far], speed.slow);
        let rule16b = fuzzy.add_rule(&[light_status.yellow, distance.medium, deviation.middle, car_distance.medium], speed.slower);
        let rule17 = fuzzy.add_rule(&[light_status.yellow, distance.medium, deviation.left, car_distance.medium_far], speed.slower);
        let rule18 = fuzzy.add_rule(&[light_status.yellow, distance.medium, deviation.right, car_distance.medium_far], speed.slower);
        let rule19 = fuzzy.add_rule(&[light_status.yellow, distance.medium, deviation.far_left, car_distance.medium_far], speed.slower);
        let rule20 = fuzzy.add_rule(&[light_status.yellow, distance.medium, deviation.far_right, car_distance.medium_far], speed.slower);

        let rule21 = fuzzy.add_rule(&[light_status.yellow, distance.near], speed.stop);

        let rule22 = fuzzy.add_rule(&[light_status.red, distance.medium, deviation.middle, car_distance.far], speed.slow);
        let rule22b = fuzzy.add_rule(&[light_status.red, distance.medium, deviation.middle, car_distance.medium], speed.slower);
        let rule23 = fuzzy.add_rule(&[light_status.red, distance.medium, deviation.left, car_distance.medium_far], speed.slower);
        let rule24 = fuzzy.add_rule(&[light_status.red, distance.medium, deviation.right, car_distance.medium_far], speed.slower);
        let rule25 = fuzzy.add_rule(&[light_status.red, distance.medium, deviation.far_left, car_distance.medium_far], speed.slower);
        let rule26 = fuzzy.add_rule(&[light_status.red, distance.medium, deviation.far_right, car_distance.medium_far], speed.slower);

        let rule27 = fuzzy.add_rule(&[light_status.red, distance.near], speed.stop);

        let rule28 = fuzzy.add_rule(&[light_status.less_green, distance.medium, deviation.middle, car_distance.far], speed.medium);
        let rule29 = fuzzy.add_rule(&[light_status.less_green, distance.medium, deviation.left, car_distance.far], speed.slow);
        let rule29b = fuzzy.add_rule(&[light_status.less_green, distance.medium, deviation.left, car_distance.medium], speed.slower);
        let rule30 = fuzzy.add_rule(&[light_status.less_green, distance.medium, deviation.right, car_distance.far], speed.slow);
        let rule30b = fuzzy.add_rule(&[light_status.less_green, distance.medium, deviation.right, car_distance.medium], speed.slower);
        let rule31 = fuzzy.add_rule(&[light_status.less_green, distance.medium, deviation.far_left, car_distance.medium_far], speed.slower);
        let rule32 = fuzzy.add_rule(&[light_status.less_green, distance.medium, deviation.far_right, car_distance.medium_far], speed.slower);

        let rule33 = fuzzy.add_rule(&[light_status.less_green, distance.near, deviation.middle, car_distance.medium_far], speed.slower);
        let rule34 = fuzzy.add_rule(&[light_status.less_green, distance.near, deviation.left, car_distance.medium_far], speed.slower);
        let rule35 = fuzzy.add_rule(&[light_status.less_green, distance.near, deviation.right, car_distance.medium_far], speed.slower);
        let rule36 = fuzzy.add_rule(&[light_status.less_green, distance.near, deviation.far_left], speed.stop);
        let rule37 = fuzzy.add_rule(&[light_status.less_green, distance.near, deviation.far_right], speed.stop);

        let rule38 = fuzzy.add_rule(&[light_status.less_red, distance.medium, deviation.middle, car_distance.far], speed.medium);
        let rule38b = fuzzy.add_rule(&[light_status.less_red, distance.medium, deviation.middle, car_distance.medium], speed.medium);
        let rule39 = fuzzy.add_rule(&[light_status.less_red, distance.medium, deviation.left, car_distance.medium_far], speed.slow);
        let rule40 = fuzzy.add_rule(&[light_status.less_red, distance.medium, deviation.right, car_distance.medium_far], speed.slow);
        let rule41 = fuzzy.add_rule(&[light_status.less_red, distance.medium, deviation.far_left, car_distance.medium_far], speed.slower);
        let rule42 = fuzzy.add_rule(&[light_status.less_red, distance.medium, deviation.far_right, car_distance.medium_far], speed.slower);
        let rule43 = fuzzy.add_rule(&[light_status.less_red, distance.near, deviation.far_right], speed.stop);

        let rule44 = fuzzy.add_rule(&[car_distance.near], speed.stop);

        let rule45 = fuzzy.add_rule(&[distance.far, car_distance.near_medium, car_opposite_distance.far, car_velocity.slow], go_left_lane.true_);
        // let rule45b = fuzzy.add_rule(&[distance.far, car_distance.medium, car_opposite_distance.far, car_velocity.slow], go_left_lane.true_);
        let rule46 = fuzzy.add_rule(&[car_opposite_distance.near], go_left_lane.false_);
        let rule47 = fuzzy.add_rule(&[car_distance.medium_far], go_left_lane.false_);
        let rule48 = fuzzy.add_rule(&[distance.medium], go_left_lane.false_);
        let rule49 = fuzzy.add_rule(&[distance.near], go_left_lane.false_);

        let rule50 = fuzzy.add_rule(&[road_deviation.far_left], steering.hard_right);
        let rule51 = fuzzy.add_rule(&[road_deviation.far_left], speed.slow);

        let rule52 = fuzzy.add_rule(&[road_deviation.middle_left], steering.straight);
        let rule53 = fuzzy.add_rule(&[road_deviation.middle_left], speed.medium);

        let rule54 = fuzzy.add_rule(&[road_deviation.middle], steering.straight);
        let rule55 = fuzzy.add_rule(&[road_deviation.middle], speed.medium);

        let rule56 = fuzzy.add_rule(&[road_deviation.right], steering.left);
        let rule57 = fuzzy.add_rule(&[road_deviation.right], speed.slow);

        let rule58 = fuzzy.add_rule(&[left_deviation.middle], stay_left_lane.true_);
        let rule59 = fuzzy.add_rule(&[left_deviation.left], stay_left_lane.true_);
        let rule60 = fuzzy.add_rule(&[left_deviation.far_left], stay_left_lane.false_);
        let rule61 = fuzzy.add_rule(&[left_deviation.right], stay_left_lane.false_);
        let rule62 = fuzzy.add_rule(&[left_deviation.right], stay_left_lane.false_);

        let rule63 = fuzzy.add_rule(&[left_deviation.far_left], steering.hard_right);
        let rule64 = fuzzy.add_rule(&[left_deviation.left], steering.right);
        let rule65 = fuzzy.add_rule(&[left_deviation.middle], steering.straight);
        let rule66 = fuzzy.add_rule(&[left_deviation.right], steering.left);
        let rule67 = fuzzy.add_rule(&[left_deviation.far_right], steering.hard_left);

        let rule68 = fuzzy.add_rule(&[left_deviation.middle], speed.medium);
        let rule69 = fuzzy.add_rule(&[left_deviation.left], speed.slow);
        let rule70 = fuzzy.add_rule(&[left_deviation.left], speed.slow);
        let rule71 = fuzzy.add_rule(&[left_deviation.right], speed.slow);
        let rule72 = fuzzy.add_rule(&[left_deviation.right], speed.slow);
        let rule73 = fuzzy.add_rule(&[left_deviation.far_left], speed.slower);
        let rule74 = fuzzy.add_rule(&[left_deviation.far_left], speed.slower);
        let rule75 = fuzzy.add_rule(&[left_deviation.far_right], speed.slower);
        let rule76 = fuzzy.add_rule(&[left_deviation.far_right], speed.slower);

        let rule77 = fuzzy.add_rule(&[side_deviation.back, car_distance.medium_far], back_to_right_lane.true_);
        let rule78 = fuzzy.add_rule(&[side_deviation.not_back], back_to_right_lane.false_);
        let rule79 = fuzzy.add_rule(&[car_distance.near], back_to_right_lane.false_);

        let rule80 = fuzzy.add_rule(&[road_deviation.far_right], steering.hard_left);
        let rule81 = fuzzy.add_rule(&[road_deviation.far_right], speed.slow);

        let rule82 = fuzzy.add_rule(&[road_deviation.middle_right], steering.straight);
        let rule83 = fuzzy.add_rule(&[road_deviation.middle_right], speed.medium);

        let rule84 = fuzzy.add_rule(&[road_deviation.middle], steering.straight);
        let rule85 = fuzzy.add_rule(&[road_deviation.middle], speed.medium);

        let rule86 = fuzzy.add_rule(&[road_deviation.left], steering.right);
        let rule87 = fuzzy.add_rule(&[road_deviation.left], speed.slow);

        let rule88 = fuzzy.add_rule(&[deviation.middle], go_normal.true_);
        let rule89 = fuzzy.add_rule(&[deviation.right], go_normal.true_);
        let rule90 = fuzzy.add_rule(&[deviation.far_right], go_normal.true_);
        let rule91 = fuzzy.add_rule(&[deviation.left], go_normal.false_);
        let rule92 = fuzzy.add_rule(&[deviation.far_left], go_normal.false_);

        let simple_rule_set = fuzzy.add_rule_set(
            &[
                rule1, rule2, rule3, rule4, rule5,
                rule6, rule7, rule7b, rule8, rule8b, rule9, rule9b, rule10, rule10b,
                rule11, rule12, rule12b, rule13, rule13b, rule14, rule15,
                rule16, rule16b, rule17, rule18, rule19, rule20,
                rule21, rule22, rule22b, rule23, rule24, rule25, rule26,
                rule27, rule28, rule29, rule29b, rule30, rule30b, rule31, rule32,
                rule33, rule34, rule35, rule36, rule37,
                rule38, rule38b, rule39, rule40, rule41, rule42, rule43,
                rule44,
            ]);

        let normal_rule_set = fuzzy.add_rule_set(
            &[
                rule1, rule2, rule3, rule4, rule5,
                rule6, rule7, rule7b, rule8, rule8b, rule9, rule9b, rule10, rule10b,
                rule11, rule12, rule12b, rule13, rule13b, rule14, rule15,
                rule16, rule16b, rule17, rule18, rule19, rule20,
                rule21, rule22, rule22b, rule23, rule24, rule25, rule26,
                rule27, rule28, rule29, rule29b, rule30, rule30b, rule31, rule32,
                rule33, rule34, rule35, rule36, rule37,
                rule38, rule38b, rule39, rule40, rule41, rule42, rule43,
                rule44,
                rule45, rule46, rule47, rule48, rule49,
            ]);

        let go_left_lane_rule_set = fuzzy.add_rule_set(
            &[
                rule50, rule51, rule52, rule53, rule54, rule55, rule56, rule57,
                rule58, rule59, rule60, rule61, rule62,
            ]);

        let stay_left_lane_rule_set = fuzzy.add_rule_set(
            &[
                rule63, rule64, rule65, rule66, rule67,
                rule68, rule69, rule70, rule71, rule72,
                rule73, rule74, rule75, rule76,
                rule77, rule78, rule79,
            ]);

        let back_to_right_lane_rule_set = fuzzy.add_rule_set(
            &[
                rule80, rule81, rule82, rule83, rule84, rule85, rule86, rule87,
                rule88, rule89, rule90, rule91, rule92,
            ]);

        Self {
            fuzzy,

            deviation,
            steering,
            distance,
            speed,
            light_status,
            car_distance,
            car_velocity,
            car_opposite_distance,
            car_opposite_velocity,
            road_deviation,
            left_deviation,
            side_deviation,

            go_left_lane,
            stay_left_lane,
            back_to_right_lane,
            go_normal,

            simple_rule_set,
            normal_rule_set,
            go_left_lane_rule_set,
            stay_left_lane_rule_set,
            back_to_right_lane_rule_set,
        }
    }
}
