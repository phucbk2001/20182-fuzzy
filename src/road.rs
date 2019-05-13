pub mod backbone;
pub mod math;
pub mod renderer;

use crate::bezier;
use crate::config::Config;

use bezier::Point;
use bezier::Line;

use std::time::{Instant};

use std::collections::BinaryHeap;
use std::cmp::Ordering;

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
    pub position: Point,
    pub adjacents: Vec<LocationId>,
}

#[derive(Copy, Clone)]
struct TmpLocation {
    location: LocationId,
    distance: f32,
}

impl PartialEq for TmpLocation {
    fn eq(&self, other: &TmpLocation) -> bool {
        other.distance == self.distance
    }
}

impl Eq for TmpLocation {}

impl Ord for TmpLocation {
    fn cmp(&self, other: &TmpLocation) -> Ordering {
        if other.distance < self.distance {
            Ordering::Less
        }
        else if other.distance > self.distance {
            Ordering::Greater
        }
        else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for TmpLocation {
    fn partial_cmp(&self, other: &TmpLocation) 
        -> Option<Ordering> 
    {
        Some(self.cmp(other))
    }
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
                let new_time = 1.0;
                (index, YellowToRed, new_time)
            }
            else {
                (index, Green, time)
            }
        },
        YellowToRed => {
            if time == 0.0 {
                let new_time = 1.0;
                (next_index(index), RedToYellow, new_time)
            }
            else {
                (index, YellowToRed, time)
            }
        },
        RedToYellow => {
            if time == 0.0 {
                let new_time = 1.0;
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

fn get_bezier_from_beziers(
    beziers: &Vec<Bezier>,
    points: &Vec<Point>,
    b: DirectedBezier)
    -> bezier::Bezier 
{
    let Bezier { point1, point2, middle } = beziers[b.bezier.id];

    if b.is_forward {
        bezier::Bezier {
            a: points[point1.id],
            b: middle,
            c: points[point2.id],
        }
    }
    else {
        bezier::Bezier {
            a: points[point2.id],
            b: middle,
            c: points[point1.id],
        }
    }
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
            if location.incoming_lanes.len() > 1 {
                update_lights(location, dt, config);
            }
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

    pub fn shortest_path(&self, a: LocationId, b: LocationId)
        -> Vec<LocationId> 
    {
        let len = self.locations.len();
        let mut queue = BinaryHeap::<TmpLocation>::new();
        let mut prevs: Vec<Option<LocationId>> =
            (0..len).into_iter().map(|_| None).collect();

        let mut visited: Vec<bool> =
            (0..len).into_iter().map(|_| false).collect();

        let mut distances: Vec<f32> =
            (0..len).into_iter().map(|_| std::f32::INFINITY).collect();

        distances[a.id] = 0.0;
        let start = TmpLocation {
            location: LocationId { id: a.id },
            distance: 0.0,
        };
        queue.push(start);

        while let Some(current) = queue.pop() {
            if current.location == b {
                break;
            }

            let current_index = current.location.id;

            if !visited[current_index] {
                visited[current_index] = true;

                for n in self.locations[current_index].adjacents.iter() {
                    if !visited[n.id] {
                        let dcurrent = distances[current_index];
                        let dn = distances[n.id];
                        let alt = dcurrent + (
                            self.locations[n.id].position -
                            self.locations[current_index].position).len();

                        if alt < dn {
                            distances[n.id] = alt;
                            prevs[n.id] = Some(current.location);
                            let loc = TmpLocation {
                                location: *n,
                                distance: alt,
                            };
                            queue.push(loc);
                        }
                    }
                }
            }
        }

        let mut result = Vec::<LocationId>::new();
        let mut current = b;
        result.push(current);
        while let Some(node) = prevs[current.id] {
            current = node;
            result.push(current);
        }
        result.reverse();
        result
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
