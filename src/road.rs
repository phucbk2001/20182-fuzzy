use crate::bezier;

#[allow(dead_code)]
pub struct Location {
    pub name: String,
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub struct LocationId {
    pub id: usize,
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub struct BezierId {
    pub id: usize,
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub struct PointId {
    pub id: usize,
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub struct Bezier {
    pub point1: PointId,
    pub point2: PointId,
    pub middle: (f32, f32),
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub struct DirectedBezier {
    pub bezier_id: BezierId,
    pub is_forward: bool,
}

#[allow(dead_code)]
pub struct Lane {
    pub from: LocationId,
    pub to: LocationId,
    pub left: Vec<DirectedBezier>,
    pub right: Vec<DirectedBezier>,
}

#[allow(dead_code)]
pub struct CrossSection {
    pub from: LocationId,
    pub across: LocationId,
    pub to: LocationId,
    pub left: Vec<DirectedBezier>,
    pub right: Vec<DirectedBezier>,
}

#[allow(dead_code)]
pub struct Road {
    pub locations: Vec<Location>,
    pub points: Vec<(f32, f32)>,
    pub beziers: Vec<Bezier>,
    pub lanes: Vec<Lane>,
    pub cross_sections: Vec<CrossSection>,
}

impl Road {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            locations: vec![],
            points: vec![],
            beziers: vec![],
            lanes: vec![],
            cross_sections: vec![],
        }
    }

    #[allow(dead_code)]
    pub fn get_point(&self, point_id: PointId) -> bezier::Point {
        let (x, y) = self.points[point_id.id];
        bezier::Point { x: x, y: y }
    }

    #[allow(dead_code)]
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
}
