pub mod renderer;
pub mod fuzzy;

use crate::bezier;
use crate::ecs;
use crate::config::Config;
use fuzzy::CarFuzzy;

use bezier::{Point};

use crate::road;
use road::{Road, LocationId};

use std::time::{Instant};

#[allow(dead_code)]
const DESTINATION_EFFECTIVE_RANGE: f32 = 0.8;

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
            velocity: 2.0,
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

impl Car {
    pub fn from_path(road: &Road, path: &[LocationId]) -> Self {
        let (pos, dest, dir) = calculate_start_and_destination(road, path);

        Self {
            position: pos,
            direction: dir,
            velocity: 5.0,
            angle: std::f32::consts::PI / 36.0,
            is_turning_left: false,
            car_type: CarType::Fast,
            destination: dest,

            path_properties: road::PathProperties::new(road, path),
        }
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

    pub fn update(&mut self, config: &Config) {
        let current = Instant::now();
        let delta = current.duration_since(self.prev_instant);
        let dt: f32 = delta.subsec_micros() as f32 / 1_000_000.0;
        self.prev_instant = current;

        for (e, car) in self.cars.iter_mut() {
            if self.em.is_alive(*e) { 
                let input = MoveInput {
                    front_wheel: config.front_wheel,
                    rear_wheel: config.rear_wheel,
                    width: config.car_width,
                    position: car.position,
                    direction: car.direction,
                    velocity: car.velocity,
                    angle: car.angle,
                    dt,
                    is_turning_left: car.is_turning_left,
                };

                let output = move_car(input);
                car.position = output.position;
                car.direction = output.direction;

                let pos = car.position + car.direction * (config.car_width / 2.0);
                let line = bezier::Line { 
                    position: pos,
                    direction: car.direction.turn_right_90_degree(),
                };
                let (left, right) = car.path_properties
                    .nearest_intersection(line);
                let dx = (left - pos).len() - config.car_width / 2.0;
                let dy = (right - pos).len() - config.car_width / 2.0;

                self.fuzzy.fuzzy.set_input(self.fuzzy.deviation.input, dx / (dx + dy));
                self.fuzzy.fuzzy.evaluate(self.fuzzy.simple_rule_set);
                let output = self.fuzzy.fuzzy.get_output(self.fuzzy.steering.output);
                let output = (output - 0.5) / 0.5;

                let angle = f32::abs(output) * std::f32::consts::PI / 6.0;
                let is_turning_left = 
                    if output < 0.0 {
                        true
                    }
                    else {
                        false
                    };
                car.angle = angle;
                car.is_turning_left = is_turning_left;

                if (car.destination - car.position).len() < DESTINATION_EFFECTIVE_RANGE {
                    self.em.deallocate(*e);
                }
            }
        }
    }
}
