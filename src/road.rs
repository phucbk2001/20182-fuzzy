pub mod backbone;
pub mod math;
pub mod renderer;

use crate::bezier;
use crate::config::Config;

use bezier::Point;
use bezier::Line;

use std::time::{Instant};

pub use self::math::*;

#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct LocationId {
    pub id: usize,
}

impl PartialEq for LocationId {
    fn eq(&self, other: &LocationId) -> bool {
        self.id == other.id
    }
}

impl Eq for LocationId {}

#[derive(Copy, Clone)]
pub struct BezierId {
    id: usize,
}

#[derive(Copy, Clone)]
pub struct PointId {
    id: usize,
}

#[derive(Copy, Clone)]
pub struct LaneId {
    pub id: usize,
}

#[derive(Copy, Clone)]
pub struct CrossSectionId {
    id: usize,
}

impl PartialEq for LaneId {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Copy, Clone)]
pub enum StreetLightColor {
    Green,
    YellowToGreen,
    YellowToRed,
    RedToYellow,
}

#[derive(Copy, Clone)]
pub enum Color {
    Red,
    Green,
    Yellow,
}

#[derive(Clone)]
pub struct Location {
    pub name: String,
    pub incoming_lanes: Vec<LaneId>,
    pub street_light_index: usize,
    pub street_light_time: f32,
    pub street_light_color: StreetLightColor,
}

#[derive(Copy, Clone)]
pub struct Bezier {
    pub point1: PointId,
    pub point2: PointId,
    pub middle: Point,
}

#[derive(Copy, Clone)]
pub struct DirectedBezier {
    pub bezier: BezierId,
    pub is_forward: bool,
}

pub struct Lane {
    pub from: LocationId,
    pub to: LocationId,
    pub left: Vec<DirectedBezier>,
    pub right: Vec<DirectedBezier>,
}

pub struct CrossSection {
    pub from: LocationId,
    pub across: LocationId,
    pub to: LocationId,
    pub left: Vec<DirectedBezier>,
    pub right: Vec<DirectedBezier>,
}

pub struct Road {
    pub locations: Vec<Location>,
    pub points: Vec<Point>,
    pub beziers: Vec<Bezier>,
    pub lanes: Vec<Lane>,
    pub cross_sections: Vec<CrossSection>,

    pub chosen_path: Vec<LocationId>,
    prev_chosen_path: Vec<LocationId>,

    prev_instant: Instant,
}

// To construct the whole map

#[derive(Copy, Clone)]
pub struct PointBackbone {
    position: Point,
    direction: Point,
}

pub struct RoadBackbone {
    pub from: LocationId,
    pub to: LocationId,
    pub points: Vec<PointId>,
}

pub struct CrossSectionBackbone {
    pub from: LocationId,
    pub across: LocationId,
    pub to: LocationId,
    pub points: Vec<PointId>,
}

pub struct Backbone {
    pub locations: Vec<Location>,
    pub points: Vec<PointBackbone>,
    pub roads: Vec<RoadBackbone>,
    pub cross_sections: Vec<CrossSectionBackbone>,
}

fn random_green_time(config: &Config) -> f32 {
    config.min_green_duration +
        (config.max_green_duration - config.min_green_duration)
        * rand::random::<f32>()
}

fn update_lights(location: &mut Location, dt: f32, config: &Config) {
    use StreetLightColor::*;

    let time = location.street_light_time - dt;
    let time = if time < 0.0 { 0.0 } else { time };

    let len = location.incoming_lanes.len();
    let index = location.street_light_index;
    let next_index = |index| { (index + 1) % len };

    let (index, color, new_time) = match location.street_light_color {
        Green => {
            if time == 0.0 {
                let new_time = 2.0;
                (index, YellowToRed, new_time)
            }
            else {
                (index, Green, time)
            }
        },
        YellowToRed => {
            if time == 0.0 {
                let new_time = 2.0;
                (next_index(index), RedToYellow, new_time)
            }
            else {
                (index, YellowToRed, time)
            }
        },
        RedToYellow => {
            if time == 0.0 {
                let new_time = 2.0;
                (index, YellowToGreen, new_time)
            }
            else {
                (index, RedToYellow, time)
            }
        },
        YellowToGreen => {
            if time == 0.0 {
                let new_time = random_green_time(config);
                (index, Green, new_time)
            }
            else {
                (index, YellowToGreen, time)
            }
        },
    };

    location.street_light_index = index;
    location.street_light_time = new_time;
    location.street_light_color = color;
}

pub fn street_light_exists(road: &Road, lane: LaneId) -> bool {
    let lane = &road.lanes[lane.id];
    let location = &road.locations[lane.to.id];
    location.incoming_lanes.len() > 1
}

impl Road {
    fn get_point(&self, point_id: PointId) -> Point {
        self.points[point_id.id]
    }

    pub fn get_bezier(&self, b: DirectedBezier) 
        -> bezier::Bezier 
    {
        let Bezier { point1, point2, middle } = 
            self.beziers[b.bezier.id];

        if b.is_forward {
            bezier::Bezier {
                a: self.get_point(point1),
                b: middle,
                c: self.get_point(point2),
            }
        }
        else {
            bezier::Bezier {
                a: self.get_point(point2),
                b: middle,
                c: self.get_point(point1),
            }
        }
    }

    pub fn chosen_path_changed(&self) -> bool {
        self.prev_chosen_path != self.chosen_path
    }

    pub fn finish(&mut self) {
        self.prev_chosen_path = self.chosen_path.clone();
    }

    pub fn update_street_lights(&mut self, config: &Config) {
        let current = Instant::now();
        let delta = current.duration_since(self.prev_instant);
        let dt: f32 = delta.subsec_micros() as f32 / 1_000_000.0;
        self.prev_instant = current;

        for location in self.locations.iter_mut() {
            update_lights(location, dt, config);
        }
    }


    pub fn get_street_light_color(&self, lane: LaneId) -> Color {
        use StreetLightColor::*;

        let location = self.lanes[lane.id].to;
        let location = &self.locations[location.id];
        if location.incoming_lanes[location.street_light_index] == lane {
            match location.street_light_color {
                RedToYellow => Color::Red,
                YellowToRed => Color::Yellow,
                YellowToGreen => Color::Yellow,
                Green => Color::Green,
            }
        }
        else {
            Color::Red
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_rand_f32() {
        for _ in 0..10 {
            let random = rand::random::<f32>();
            assert!(random >= 0.0);
            assert!(random <= 1.0);
        }
    }
}
