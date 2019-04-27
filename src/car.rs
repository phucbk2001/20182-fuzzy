pub mod renderer;

use crate::bezier;
use crate::ecs;
use bezier::{Point};

use crate::road;
use road::{Road, LocationId};

use std::time::{Instant};

const DESTINATION_EFFECTIVE_RANGE: f32 = 3.0;

#[derive(Copy, Clone)]
pub struct ForCar {}

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

impl Default for Car {
    fn default() -> Car {
        Car {
            position: Point { x: 0.0, y: 0.0 },
            direction: Point { x: 1.0, y: 0.0 },
            velocity: 5.0,
            car_type: CarType::Fast,
            destination: Point { x: 100.0, y: 100.0 },
        }
    }
}

impl Car {
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

    pub fn from_path(road: &Road, path: &[LocationId]) -> Self {
        let (pos, dest, dir) = Car::calculate_start_and_destination(road, path);

        Self {
            position: pos,
            direction: dir,
            velocity: 10.0,
            car_type: CarType::Fast,
            destination: dest,
        }
    }
}

pub struct CarSystem {
    prev_instant: Instant,
    pub em: ecs::EntityManager<ForCar>,
    pub cars: ecs::Components<Car, ForCar>,
}

impl CarSystem {
    pub fn new() -> Self {
        Self {
            prev_instant: Instant::now(),
            em: ecs::EntityManager::new(),
            cars: ecs::Components::new(),
        }
    }

    pub fn add(&mut self, car: Car) {
        self.cars.add(&mut self.em, car);
    }

    pub fn update(&mut self) {
        let current = Instant::now();
        let delta = current.duration_since(self.prev_instant);
        let d: f32 = delta.subsec_micros() as f32 / 1_000_000.0;

        for (e, car) in self.cars.iter_mut() {
            if self.em.is_alive(*e) { 
                let pos = car.position;
                let v = car.direction * car.velocity;
                car.position = pos + v * d;
            }
        }

        self.prev_instant = current;
    }
}
