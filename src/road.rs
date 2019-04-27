pub mod backbone;
pub mod math;
pub mod renderer;

use crate::bezier;
use crate::config;

use bezier::Point;
use bezier::Line;

pub use self::math::*;

#[derive(Clone)]
pub struct Location {
    pub name: String,
}

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

#[derive(Copy, Clone)]
pub struct BezierId {
    pub id: usize,
}

#[derive(Copy, Clone)]
pub struct PointId {
    pub id: usize,
}

#[derive(Copy, Clone)]
pub struct Bezier {
    pub point1: PointId,
    pub point2: PointId,
    pub middle: (f32, f32),
}

#[derive(Copy, Clone)]
pub struct DirectedBezier {
    pub bezier_id: BezierId,
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
    pub points: Vec<(f32, f32)>,
    pub beziers: Vec<Bezier>,
    pub lanes: Vec<Lane>,
    pub cross_sections: Vec<CrossSection>,

    pub chosen_path: Vec<LocationId>,
    prev_chosen_path: Vec<LocationId>,
}

// To construct the whole map

#[derive(Copy, Clone)]
pub struct PointBackbone {
    position: (f32, f32),
    direction: (f32, f32),
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

impl Road {
    fn get_point(&self, point_id: PointId) -> bezier::Point {
        let (x, y) = self.points[point_id.id];
        bezier::Point { x: x, y: y }
    }

    pub fn get_bezier(&self, b: DirectedBezier) 
        -> bezier::Bezier 
    {
        let Bezier { point1, point2, middle } = 
            self.beziers[b.bezier_id.id];

        let is_forward = b.is_forward;

        let (xb, yb) = middle;

        if is_forward {
            bezier::Bezier {
                a: self.get_point(point1),
                b: bezier::Point { x: xb, y: yb },
                c: self.get_point(point2),
            }
        }
        else {
            bezier::Bezier {
                a: self.get_point(point2),
                b: bezier::Point { x: xb, y: yb },
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
}
