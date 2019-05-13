pub mod renderer;
pub mod fuzzy;

use crate::bezier;
use crate::ecs;
use crate::config::Config;
use fuzzy::CarFuzzy;

use bezier::{Point};

use crate::road;
use road::{Road, LocationId, LaneId};

use std::time::{Instant};

const DESTINATION_EFFECTIVE_RANGE: f32 = 1.2;
const NEAREST_CAR_POSITION_ANGLE: f32 = 60.0;
const NEAREST_CAR_DIRECTION_ANGLE: f32 = 90.0;
const FIND_RADIUS: f32 = 3.0;

#[derive(Copy, Clone)]
pub struct ForCar {}

#[derive(Copy, Clone)]
pub enum CarState {
    GoNormal,
    GoLeftLane,
    StayLeftLane,
    BackToRightLane,
}

#[derive(Copy, Clone)]
pub enum CarType {
    Slow, 
    Normal(CarState),
}

#[derive(Clone)]
pub struct Car {
    pub position: Point,
    pub direction: Point,
    pub velocity: f32,
    pub angle: f32,
    pub is_turning_left: bool,

    pub car_type: CarType,
    pub starting: Point,
    pub destination: Point,
    followed_car: Option<ecs::Entity<ForCar>>,
    is_turning_back: bool,

    pub path_properties: road::PathProperties,
}

#[derive(Copy, Clone)]
struct NearestCar {
    position: Point,
    velocity: f32,
    nearest_car: ecs::Entity<ForCar>,
}

#[derive(Copy, Clone)]
struct NearestOppositeCar {
    position: Point,
    velocity: f32,
}

#[derive(Copy, Clone)]
struct FollowedCar {
    position: Point,
}

fn default_velocity_for(car_type: CarType) -> f32 {
    use CarType::*;

    match car_type {
        Normal(_) => 10.0,
        Slow => 2.0,
    }
}

impl Default for Car {
    fn default() -> Car {
        let car_type = CarType::Normal(CarState::GoNormal);
        Car {
            position: Point { x: 0.0, y: 0.0 },
            direction: Point { x: 1.0, y: 0.0 },
            velocity: default_velocity_for(car_type),
            angle: std::f32::consts::PI / 36.0,
            is_turning_left: false,
            car_type,
            starting: Point { x: 0.0, y: 0.0 },
            destination: Point { x: 100.0, y: 100.0 },
            followed_car: None,
            is_turning_back: false,

            path_properties: road::PathProperties::default(),
        }
    }
}

fn calculate_start_and_destination(road: &Road, path: &[LocationId]) 
    -> (Point, Point, Point) 
{
    let a = path[0];
    let b = path[1];
    let start_lane = road.lanes.iter().find(
        |lane| lane.from == a && lane.to == b).unwrap();

    let len = path.len();
    let a = path[len - 2];
    let b = path[len - 1];
    let end_lane = road.lanes.iter().find(
        |lane| lane.from == a && lane.to == b).unwrap();

    let bezier1 = road.get_bezier(start_lane.left[0]);
    let bezier2 = road.get_bezier(start_lane.right[0]);
    let p1 = bezier1.pos(0.0);
    let p2 = bezier2.pos(0.0);
    let position = (p1 + p2) * 0.5;

    let b3 = *end_lane.left.iter().last().unwrap();
    let b4 = *end_lane.right.iter().last().unwrap();
    let p3 = road.get_bezier(b3).pos(1.0);
    let p4 = road.get_bezier(b4).pos(1.0);
    let destination = (p3 + p4) * 0.5;
    let direction = bezier1.direction(0.0);

    (position, destination, direction)
}

#[derive(Copy, Clone)]
struct MoveInput {
    front_wheel: f32,
    rear_wheel: f32,
    width: f32,
    position: Point,
    direction: Point,
    velocity: f32,
    angle: f32,
    dt: f32,
    is_turning_left: bool,
}

#[derive(Copy, Clone)]
struct MoveOutput {
    position: Point,
    direction: Point,
}

fn move_car(input: MoveInput) -> MoveOutput {
    let wb = input.front_wheel + input.rear_wheel;
    let radius = wb / f32::tan(input.angle) + input.width / 2.0;
    let vdt = input.velocity * input.dt;

    let ey = input.direction;
    let ex = ey.turn_right_90_degree();

    let mut dx: f32;
    let dy: f32;
    let mut vx: f32;
    let vy: f32;

    if radius < 100.0 {
        let phi = vdt / radius;
        dx = radius * (1.0 - f32::cos(phi));
        dy = radius * f32::sin(phi);
        vx = f32::sin(phi);
        vy = f32::cos(phi);
    }
    else {
        dx = (vdt * vdt) / (2.0 * radius);
        dy = vdt;
        vx = vdt / radius;
        vy = 1.0 - (vdt * vdt) / (2.0 * radius * radius);
    }

    if input.is_turning_left {
        dx = -dx;
        vx = -vx;
    }

    let middle_rear_wheel = input.position - ey * input.rear_wheel;

    let middle_rear_wheel = middle_rear_wheel + dx * ex + dy * ey;
    let direction = (vx * ex + vy * ey).normalize();
    let position = middle_rear_wheel + ey * input.rear_wheel;

    MoveOutput { position, direction }
}

fn get_lane_light_status(road: &Road, lane: LaneId) -> f32 {
    use road::StreetLightColor::*;

    let location = road.lanes[lane.id].to;
    let location = &road.locations[location.id];

    let t = location.street_light_time;
    if lane == location.incoming_lanes[location.street_light_index] {
        match location.street_light_color {
            RedToYellow => {
                6.0 - t / 2.0
            },
            YellowToRed => {
                4.0 - t
            },
            YellowToGreen => {
                8.0 - t
            },
            Green => {
                if t > 2.0 {
                    1.0
                }
                else {
                    2.0 - t / 2.0
                }
            }
        }
    }
    else {
        5.0
    }
}

impl Car {
    pub fn from_path(road: &Road, path: &[LocationId]) -> Self {
        let (pos, dest, dir) = calculate_start_and_destination(road, path);
        let path_properties = road::PathProperties::new(road, path);

        let car_type = CarType::Normal(CarState::GoNormal);
        Self {
            position: pos,
            direction: dir,
            velocity: default_velocity_for(car_type),
            angle: std::f32::consts::PI / 36.0,
            is_turning_left: false,
            car_type,
            starting: pos,
            destination: dest,
            followed_car: None,
            is_turning_back: false,

            path_properties,
        }
    }

    pub fn from_positions(road: &Road, a: Point, b: Point, car_type: CarType) 
        -> Option<Self>
    {
        let lane1_id = road::math::find_lane_contains(road, a)?;
        let lane2_id = road::math::find_lane_contains(road, b)?;

        let lane1 = &road.lanes[lane1_id.id];
        let lane2 = &road.lanes[lane2_id.id];

        let mut path = vec![lane1.from];
        if lane1_id.id != lane2_id.id {
            path.append(&mut road.shortest_path(lane1.to, lane2.from));
        }
        path.push(lane2.to);
        let path_names: Vec<String> =
            path.iter()
            .map(|location| { road.locations[location.id].name.clone() })
            .collect();

        println!("Path: {:?}", path_names);

        let path_properties = road::math::PathProperties::new(road, &path);

        Some(Self {
            position: a,
            direction: road::math::direction_in_lane_of(road, lane1_id, a),
            velocity: default_velocity_for(car_type),
            angle: std::f32::consts::PI / 36.0,
            is_turning_left: false,
            car_type: car_type,
            starting: a,
            destination: b,
            followed_car: None,
            is_turning_back: false,

            path_properties,
        })
    }

    fn do_move(&mut self, dt: f32, config: &Config) {
        let input = MoveInput {
            front_wheel: config.front_wheel,
            rear_wheel: config.rear_wheel,
            width: config.car_width,
            position: self.position,
            direction: self.direction,
            velocity: self.velocity,
            angle: self.angle,
            dt,
            is_turning_left: self.is_turning_left,
        };

        let output = move_car(input);

        self.position = output.position;
        self.direction = output.direction;
    }

    fn fuzzy_set_deviation(&mut self, fuzzy: &mut CarFuzzy, config: &Config) {
        let pos = self.position + self.direction * (config.car_width / 2.0);
        let line = bezier::Line { 
            position: pos,
            direction: self.direction.turn_right_90_degree(),
        };
        let (left, right, far_left) = self.path_properties
            .nearest_intersection(line);

        self.is_turning_back = approx_eq(right, far_left);

        let dir = line.direction;
        let dx = -bezier::dot(left - pos, dir) - config.car_width / 2.0;
        let dy = bezier::dot(right - pos, dir) - config.car_width / 2.0;
        let far_dx = -bezier::dot(far_left - pos, dir) - config.car_width / 2.0;
        let left_dy = bezier::dot(left - pos, dir) - config.car_width / 2.0;

        let dx = if dx < 0.0 { 0.0 } else { dx };
        let dy = if dy < 0.0 { 0.0 } else { dy };
        let far_dx = if far_dx < 0.0 { 0.0 } else { far_dx };
        let left_dy = if left_dy < 0.0 { 0.0 } else { left_dy };

        fuzzy.fuzzy.set_input(fuzzy.deviation.input, dx / (dx + dy));
        fuzzy.fuzzy.set_input(fuzzy.road_deviation.input, far_dx / (far_dx + dy));
        fuzzy.fuzzy.set_input(fuzzy.left_deviation.input, far_dx / (far_dx + left_dy));
    }

    fn fuzzy_set_light_status_distance(&self, fuzzy: &mut CarFuzzy, road: &Road) {
        if let Some((lane, light_pos)) = road::math::nearest_street_light(
            &self.path_properties.street_lights, self.position, self.direction)
        {
            fuzzy.fuzzy.set_input(fuzzy.distance.input, (light_pos - self.position).len());
            fuzzy.fuzzy.set_input(fuzzy.light_status.input, get_lane_light_status(road, lane));
        }
        else {
            fuzzy.fuzzy.set_input(fuzzy.distance.input, 1000.0);
            fuzzy.fuzzy.set_input(fuzzy.light_status.input, 1.0);
        }
    }

    fn fuzzy_set_nearest_car(
        &self, fuzzy: &mut CarFuzzy,
        nearest_car: Option<NearestCar>)
    {
        if let Some(nearest_car) = nearest_car {
            let distance = (nearest_car.position - self.position).len();
            fuzzy.fuzzy.set_input(fuzzy.car_distance.input, distance);
            fuzzy.fuzzy.set_input(fuzzy.car_velocity.input, nearest_car.velocity);
        }
        else {
            fuzzy.fuzzy.set_input(fuzzy.car_distance.input, 200.0);
            fuzzy.fuzzy.set_input(fuzzy.car_velocity.input, 30.0);
        }
    }

    fn fuzzy_set_nearest_opposite_car(
        &self, fuzzy: &mut CarFuzzy,
        nearest_car: Option<NearestOppositeCar>)
    {
        if let Some(nearest_car) = nearest_car {
            let distance = (nearest_car.position - self.position).len();
            fuzzy.fuzzy.set_input(fuzzy.car_opposite_distance.input, distance);
            fuzzy.fuzzy.set_input(fuzzy.car_opposite_velocity.input, nearest_car.velocity);
        }
        else {
            fuzzy.fuzzy.set_input(fuzzy.car_opposite_distance.input, 200.0);
            fuzzy.fuzzy.set_input(fuzzy.car_opposite_velocity.input, 30.0);
        }
    }

    fn fuzzy_set_side_car(
        &self, fuzzy: &mut CarFuzzy,
        followed_car: Option<FollowedCar>) 
    {
        if let Some(followed_car) = followed_car {
            let deviation = bezier::dot(followed_car.position - self.position, self.direction);
            fuzzy.fuzzy.set_input(fuzzy.side_deviation.input, deviation);
        }
        else {
            fuzzy.fuzzy.set_input(fuzzy.side_deviation.input, 100.0);
        }
    }

    fn fuzzy_output_set_steering(&mut self, fuzzy: &CarFuzzy) {
        let output = fuzzy.fuzzy.get_output(fuzzy.steering.output);
        let output = (output - 0.5) / 0.5;

        let angle = f32::abs(output) * std::f32::consts::PI / 2.0;
        let is_turning_left = 
            if output < 0.0 {
                true
            }
            else {
                false
            };
        self.angle = angle;
        self.is_turning_left = is_turning_left;
    }

    fn fuzzy_output_set_speed(&mut self, fuzzy: &CarFuzzy) {
        let output = fuzzy.fuzzy.get_output(fuzzy.speed.output);
        let output = if f32::is_normal(output) { output } else { 0.0 };
        let output = 
            if output < 0.0 {
                0.0 
            }
            else if output <= 1.0 {
                output
            }
            else {
                1.0
            };

        let v = default_velocity_for(self.car_type) * output;
        self.velocity = if v < 0.2 { 0.0 } else { v };
    }

    fn do_fuzzy(
        &mut self, fuzzy: &mut CarFuzzy,
        road: &Road, config: &Config,
        nearest_car: Option<NearestCar>,
        nearest_opposite_car: Option<NearestOppositeCar>,
        followed_car: Option<FollowedCar>)
    {
        use CarType::*;
        use CarState::*;

        self.fuzzy_set_deviation(fuzzy, config);
        self.fuzzy_set_light_status_distance(fuzzy, road);
        self.fuzzy_set_nearest_car(fuzzy, nearest_car);

        self.car_type = match self.car_type {
            Slow => {
                fuzzy.fuzzy.evaluate(fuzzy.simple_rule_set);
                Slow
            },
            Normal(state) => {
                match state {
                    GoNormal => {
                        self.fuzzy_set_nearest_opposite_car(fuzzy, nearest_opposite_car);

                        fuzzy.fuzzy.evaluate(fuzzy.normal_rule_set);

                        let output = fuzzy.fuzzy.get_output(fuzzy.go_left_lane.output);
                        if output > 0.5 && !self.is_turning_back {
                            self.followed_car =
                                if let Some(nearest_car) = nearest_car {
                                    Some(nearest_car.nearest_car)
                                }
                                else {
                                    println!("Warning: FollowedCar");
                                    None
                                };

                            Normal(GoLeftLane)
                        }
                        else {
                            Normal(state)
                        }
                    },
                    GoLeftLane => {
                        fuzzy.fuzzy.evaluate(fuzzy.go_left_lane_rule_set);
                        let output = fuzzy.fuzzy.get_output(fuzzy.stay_left_lane.output);
                        if output > 0.5 {
                            Normal(StayLeftLane)
                        }
                        else {
                            Normal(state)
                        }
                    },
                    StayLeftLane => {
                        self.fuzzy_set_side_car(fuzzy, followed_car);
                        fuzzy.fuzzy.evaluate(fuzzy.stay_left_lane_rule_set);
                        let output = fuzzy.fuzzy.get_output(fuzzy.back_to_right_lane.output);
                        if output > 0.5 {
                            Normal(BackToRightLane)
                        }
                        else {
                            Normal(state)
                        }
                    },
                    BackToRightLane => {
                        fuzzy.fuzzy.evaluate(fuzzy.back_to_right_lane_rule_set);
                        let output = fuzzy.fuzzy.get_output(fuzzy.go_normal.output);
                        if output > 0.5 {
                            Normal(GoNormal)
                        }
                        else {
                            Normal(state)
                        }
                    },
                }
            },
        };

        self.fuzzy_output_set_steering(fuzzy);
        self.fuzzy_output_set_speed(fuzzy);
    }
}

pub enum AddCar {
    Nope,
    Adding,
    AddedPoint(Point),
}

pub struct CarSystem {
    prev_instant: Instant,
    pub em: ecs::EntityManager<ForCar>,
    pub cars: ecs::Components<Car, ForCar>,
    nearest_cars: ecs::Components<Option<NearestCar>, ForCar>,
    nearest_opposite_cars:
        ecs::Components<Option<NearestOppositeCar>, ForCar>,
    followed_cars:
        ecs::Components<Option<FollowedCar>, ForCar>,

    fuzzy: CarFuzzy,
    pub add_car: AddCar,
    pub add_car_type: CarType,
    pub chosen_car: Option<ecs::Entity<ForCar>>,
    old_chosen_car: Option<ecs::Entity<ForCar>>,
}

fn approx_eq(a: bezier::Point, b: bezier::Point) -> bool {
    if f32::abs(a.x - b.x) > 0.0001 {
        false
    }
    else if f32::abs(a.y - b.y) > 0.0001 {
        false
    }
    else {
        true
    }
}

fn find_nearest_car(
    em: &ecs::EntityManager<ForCar>,
    cars: &ecs::Components<Car, ForCar>,
    pos: Point, dir: Point)
    -> Option<ecs::Entity<ForCar>>
{
    let dir = dir.normalize();
    let mut result = None;
    for (e, car) in cars.iter() {
        if em.is_alive(*e) {
            if approx_eq(pos, car.position) {
                continue;
            }

            let phi_pos = NEAREST_CAR_POSITION_ANGLE * std::f32::consts::PI / 180.0;
            let phi_dir = NEAREST_CAR_DIRECTION_ANGLE * std::f32::consts::PI / 180.0;

            let cos_pos = bezier::dot((car.position - pos).normalize(), dir);
            let cos_dir = bezier::dot(car.direction.normalize(), dir);

            if cos_pos >= f32::cos(phi_pos) && cos_dir >= f32::cos(phi_dir) {
                result =
                    if let Some(old_entity) = result {
                        let new_pos = car.position;
                        let old_pos = cars.get(old_entity).position;
                        if (new_pos - pos).len() < (old_pos - pos).len() {
                            Some(*e)
                        }
                        else {
                            result
                        }
                    }
                    else {
                        Some(*e)
                    }
            }
        }
    }
    result
}

fn find_nearest_opposite_car(
    em: &ecs::EntityManager<ForCar>,
    cars: &ecs::Components<Car, ForCar>,
    pos: Point, dir: Point)
    -> Option<ecs::Entity<ForCar>>
{
    let dir = dir.normalize();
    let mut result = None;
    for (e, car) in cars.iter() {
        if em.is_alive(*e) {
            if approx_eq(pos, car.position) {
                continue;
            }

            let phi_pos = NEAREST_CAR_POSITION_ANGLE * std::f32::consts::PI / 180.0;
            let phi_dir = NEAREST_CAR_DIRECTION_ANGLE * std::f32::consts::PI / 180.0;

            let cos_pos = bezier::dot((car.position - pos).normalize(), dir);
            let cos_dir = bezier::dot(-1.0 * car.direction.normalize(), dir);

            if cos_pos >= f32::cos(phi_pos) && cos_dir >= f32::cos(phi_dir) {
                result =
                    if let Some(old_entity) = result {
                        let new_pos = car.position;
                        let old_pos = cars.get(old_entity).position;
                        if (new_pos - pos).len() < (old_pos - pos).len() {
                            Some(*e)
                        }
                        else {
                            result
                        }
                    }
                    else {
                        Some(*e)
                    }
            }
        }
    }
    result
}

impl CarSystem {
    pub fn new() -> Self {
        let car_type = CarType::Normal(CarState::GoNormal);
        Self {
            prev_instant: Instant::now(),
            em: ecs::EntityManager::new(),
            cars: ecs::Components::new(),
            nearest_cars: ecs::Components::new(),
            nearest_opposite_cars: ecs::Components::new(),
            followed_cars: ecs::Components::new(),
            fuzzy: CarFuzzy::new(),
            add_car: AddCar::Nope,
            add_car_type: car_type,
            chosen_car: None,
            old_chosen_car: None,
        }
    }

    pub fn add(&mut self, car: Car) {
        let e = self.em.allocate();
        self.chosen_car = Some(e);
        self.cars.set(e, car);
        self.nearest_cars.set(e, None);
        self.nearest_opposite_cars.set(e, None);
        self.followed_cars.set(e, None);
    }

    pub fn update(&mut self, road: &Road, config: &Config) {
        let current = Instant::now();
        let delta = current.duration_since(self.prev_instant);
        let dt: f32 = delta.subsec_micros() as f32 / 1_000_000.0;
        self.prev_instant = current;

        for (e, nearest_car) in self.nearest_cars.iter_mut() {
            if self.em.is_alive(*e) {
                let pos = self.cars.get(*e).position;
                let dir = self.cars.get(*e).direction;
                let maybe_found_entity = find_nearest_car(&self.em, &self.cars, pos, dir);

                if let Some(found_entity) = maybe_found_entity {
                    let position = self.cars.get(found_entity).position;
                    let velocity = self.cars.get(found_entity).velocity;

                    *nearest_car = Some(NearestCar {
                        position,
                        velocity,
                        nearest_car: found_entity,
                    });
                }
                else {
                    *nearest_car = None;
                }
            }
        }

        for (e, nearest_car) in self.nearest_opposite_cars.iter_mut(){
            if self.em.is_alive(*e) {
                let pos = self.cars.get(*e).position;
                let dir = self.cars.get(*e).direction;
                let maybe_found_entity = find_nearest_opposite_car(&self.em, &self.cars, pos, dir);

                if let Some(found_entity) = maybe_found_entity {
                    let position = self.cars.get(found_entity).position;
                    let velocity = self.cars.get(found_entity).velocity;

                    *nearest_car = Some(NearestOppositeCar {
                        position,
                        velocity,
                    });
                }
                else {
                    *nearest_car = None;
                }
            }
        }

        for (e, followed_car) in self.followed_cars.iter_mut() {
            if self.em.is_alive(*e) {
                let car = self.cars.get(*e);
                *followed_car =
                    if let Some(followed_car) = car.followed_car {
                        let followed_car = self.cars.get(followed_car);
                        Some(FollowedCar {
                            position: followed_car.position,
                        })
                    }
                    else {
                        None
                    }
            }
        }

        for (e, car) in self.cars.iter_mut() {
            if self.em.is_alive(*e) { 
                car.do_move(dt, config);
                let nearest_car = *self.nearest_cars.get(*e);
                let nearest_opposite_car = *self.nearest_opposite_cars.get(*e);
                let followed_car = *self.followed_cars.get(*e);
                car.do_fuzzy(&mut self.fuzzy, 
                             road, config, nearest_car, 
                             nearest_opposite_car, followed_car
                             );

                if (car.destination - car.position).len() < DESTINATION_EFFECTIVE_RANGE {
                    self.em.deallocate(*e);
                }
            }
        }
    }

    pub fn chosen_car_changed(&self) -> bool {
        match self.old_chosen_car {
            None => {
                match self.chosen_car {
                    None => false,
                    Some(_) => true,
                }
            },
            Some(e1) => {
                match self.chosen_car {
                    None => true,
                    Some(e2) => e1 != e2,
                }
            },
        }
    }

    pub fn finish(&mut self) {
        self.old_chosen_car = self.chosen_car;
    }

    pub fn find_car_near(&self, p: Point) -> Option<ecs::Entity<ForCar>> {
        for (e, car) in self.cars.iter() {
            if self.em.is_alive(*e) {
                if (car.position - p).len() < FIND_RADIUS {
                    return Some(*e);
                }
            }
        }
        None
    }
}
