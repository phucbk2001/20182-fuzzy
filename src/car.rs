use crate::bezier;
use bezier::{Point};

use crate::road;
use road::{Road, LocationId};

use std::time::{Instant};

const DESTINATION_EFFECTIVE_RANGE: f32 = 3.0;

#[derive(Copy, Clone)]
pub enum CarType {
    Slow, 
    Fast,
}

#[derive(Copy, Clone)]
pub struct Car {
    pub position: Point,
    pub direction: Point,
    pub velocity: f32,
    pub car_type: CarType,
    pub destination: Point,
}

impl Car {
    pub fn from_path(road: &Road, path: &[LocationId]) -> Self {
        let start = path[0];
        let end = *path.iter().last().unwrap();
        let start_lane = road.lanes.iter().find(
            |lane| lane.from == start).unwrap();
        let end_lane = road.lanes.iter().find(
            |lane| lane.to == end).unwrap();

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

        Self {
            position: position,
            direction: direction,
            velocity: 10.0,
            car_type: CarType::Fast,
            destination: destination,
        }
    }
}

pub struct CarSystem {
    prev_instant: Instant,
    cars: Vec<Car>,
}

impl CarSystem {
    pub fn new() -> Self {
        Self {
            prev_instant: Instant::now(),
            cars: vec![],
        }
    }

    pub fn add(&mut self, car: Car) {
        self.cars.push(car);
    }

    pub fn update(&mut self) {
        let current = Instant::now();
        let delta = current.duration_since(self.prev_instant);
        let d: f32 = delta.subsec_micros() as f32 / 1_000_000.0;

        for car in self.cars.iter_mut() {
            let pos = car.position;
            let v = car.direction * car.velocity;
            car.position = pos + v * d;

            println!("Car: {} {}", car.position.x, car.position.y);
        }

        self.prev_instant = current;
    }
}
