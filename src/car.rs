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

#[allow(dead_code)]
const DESTINATION_EFFECTIVE_RANGE: f32 = 0.8;
const MAX_VELOCITY: f32 = 10.0;

#[derive(Copy, Clone)]
pub struct ForCar {}

#[derive(Copy, Clone)]
pub enum CarType {
    #[allow(dead_code)]
    Slow, 
    Fast,
}

#[derive(Clone)]
pub struct Car {
    pub position: Point,
    pub direction: Point,
    pub velocity: f32,
    pub angle: f32,
    pub is_turning_left: bool,

    pub car_type: CarType,
    pub destination: Point,

    pub path_properties: road::PathProperties,
}

impl Default for Car {
    fn default() -> Car {
        Car {
            position: Point { x: 0.0, y: 0.0 },
            direction: Point { x: 1.0, y: 0.0 },
            velocity: MAX_VELOCITY,
            angle: std::f32::consts::PI / 36.0,
            is_turning_left: false,
            car_type: CarType::Fast,
            destination: Point { x: 100.0, y: 100.0 },

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
        vx = radius * f32::sin(phi);
        vy = radius * f32::cos(phi);
    }
    else {
        dx = (vdt * vdt) / (2.0 * radius);
        dy = vdt;
        vx = vdt;
        vy = radius - (vdt * vdt) / (2.0 * radius);
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

        Self {
            position: pos,
            direction: dir,
            velocity: MAX_VELOCITY,
            angle: std::f32::consts::PI / 36.0,
            is_turning_left: false,
            car_type: CarType::Fast,
            destination: dest,

            path_properties,
        }
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

    fn fuzzy_set_deviation(&self, fuzzy: &mut CarFuzzy, config: &Config) {
        let pos = self.position + self.direction * (config.car_width / 2.0);
        let line = bezier::Line { 
            position: pos,
            direction: self.direction.turn_right_90_degree(),
        };
        let (left, right) = self.path_properties
            .nearest_intersection(line);
        let dx = (left - pos).len() - config.car_width / 2.0;
        let dy = (right - pos).len() - config.car_width / 2.0;

        fuzzy.fuzzy.set_input(fuzzy.deviation.input, dx / (dx + dy));
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
        let output = if f32::is_finite(output) { output } else { 0.0 };
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

        let v = MAX_VELOCITY * output;
        self.velocity = if v < 0.2 { 0.0 } else { v };
    }

    fn do_fuzzy(&mut self, fuzzy: &mut CarFuzzy, road: &Road, config: &Config) {
        self.fuzzy_set_deviation(fuzzy, config);
        self.fuzzy_set_light_status_distance(fuzzy, road);

        fuzzy.fuzzy.evaluate(fuzzy.simple_rule_set);

        self.fuzzy_output_set_steering(fuzzy);
        self.fuzzy_output_set_speed(fuzzy);
    }
}

pub struct CarSystem {
    prev_instant: Instant,
    pub em: ecs::EntityManager<ForCar>,
    pub cars: ecs::Components<Car, ForCar>,
    fuzzy: CarFuzzy,
}

impl CarSystem {
    pub fn new() -> Self {
        Self {
            prev_instant: Instant::now(),
            em: ecs::EntityManager::new(),
            cars: ecs::Components::new(),
            fuzzy: CarFuzzy::new(),
        }
    }

    pub fn add(&mut self, car: Car) {
        self.cars.add(&mut self.em, car);
    }

    pub fn update(&mut self, road: &Road, config: &Config) {
        let current = Instant::now();
        let delta = current.duration_since(self.prev_instant);
        let dt: f32 = delta.subsec_micros() as f32 / 1_000_000.0;
        self.prev_instant = current;

        for (e, car) in self.cars.iter_mut() {
            if self.em.is_alive(*e) { 
                car.do_move(dt, config);

                car.do_fuzzy(&mut self.fuzzy, road, config);

                if (car.destination - car.position).len() < DESTINATION_EFFECTIVE_RANGE {
                    self.em.deallocate(*e);
                }
            }
        }
    }
}
