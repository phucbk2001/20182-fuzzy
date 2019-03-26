use crate::bezier;
use crate::config;

use bezier::Point;
use bezier::Line;

#[derive(Clone)]
pub struct Location {
    pub name: String,
}

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

#[derive(Copy, Clone)]
struct ThreePoints {
    left_pos: Point, 
    middle_pos: Point,
    right_pos: Point,
    left_line: Line, 
    middle_line: Line,
    right_line: Line,
}

#[derive(Copy, Clone)]
struct ThreePointIds {
    left_id: PointId,
    middle_id: PointId,
    right_id: PointId,
}

#[derive(Copy, Clone)]
struct ThreePointBezierIds {
    left_id: BezierId,
    middle_id: BezierId,
    right_id: BezierId,
}

fn point_id_to_three_points(
    backbone: &Backbone, config: &config::Config,
    point_id: &PointId) -> ThreePoints
{
    let p = backbone.get_point(*point_id);
    let middle_pos = Point::from(p.position);

    let direction = Point::from(p.direction);
    let left_dir = direction.turn_left_90_degree();
    let right_dir = direction.turn_right_90_degree();
    let ratio = config.lane_width / direction.len();

    let left_pos = middle_pos + ratio * left_dir;
    let right_pos = middle_pos + ratio * right_dir;

    let middle_line = Line {
        position: middle_pos,
        direction: direction,
    };

    let left_line = Line {
        position: left_pos,
        direction: direction,
    };

    let right_line = Line {
        position: right_pos,
        direction: direction,
    };

    ThreePoints {
        left_pos: left_pos,
        middle_pos: middle_pos,
        right_pos: right_pos,
        left_line: left_line,
        middle_line: middle_line,
        right_line: right_line,
    }
}

fn add_three_points(
    points: &mut Vec<(f32, f32)>, three_points: ThreePoints) 
    -> ThreePointIds
{
    let len = points.len();

    let Point { x, y } = three_points.left_pos;
    points.push((x, y));

    let Point { x, y } = three_points.middle_pos;
    points.push((x, y));

    let Point { x, y } = three_points.right_pos;
    points.push((x, y));

    ThreePointIds {
        left_id: PointId { id: len },
        middle_id: PointId { id: len + 1 },
        right_id: PointId { id: len + 2},
    }
}

fn add_bezier_from_two_three_points(
    beziers: &mut Vec<Bezier>,
    t1: ThreePoints, t1_point_ids: ThreePointIds,
    t2: ThreePoints, t2_point_ids: ThreePointIds)
    -> ThreePointBezierIds 
{
    let len = beziers.len();

    let left_bezier_middle = 
        bezier::intersect_lines(t1.left_line, t2.left_line);

    let middle_bezier_middle =
        bezier::intersect_lines(t1.middle_line, t2.middle_line);

    let right_bezier_middle = 
        bezier::intersect_lines(t1.right_line, t2.right_line);

    beziers.push(Bezier {
        point1: t1_point_ids.left_id,
        point2: t2_point_ids.left_id,
        middle: From::from(left_bezier_middle),
    });

    beziers.push(Bezier {
        point1: t1_point_ids.middle_id,
        point2: t2_point_ids.middle_id,
        middle: From::from(middle_bezier_middle),
    });

    beziers.push(Bezier {
        point1: t1_point_ids.right_id,
        point2: t2_point_ids.right_id,
        middle: From::from(right_bezier_middle),
    });

    ThreePointBezierIds {
        left_id: BezierId { id: len },
        middle_id: BezierId { id: len + 1 },
        right_id: BezierId { id: len + 2 },
    }
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

    pub fn from(backbone: &Backbone, config: &config::Config) -> Self {
        let locations = backbone.locations.clone();
        let mut points: Vec<(f32, f32)> = vec![];
        let mut beziers: Vec<Bezier> = vec![];
        let mut lanes: Vec<Lane> = vec![];
        let mut cross_sections: Vec<CrossSection> = vec![];

        let map_func = |point_id: &PointId| {
            point_id_to_three_points(backbone, config, point_id)
        };

        for r in &backbone.roads {
            let mut left_lane = Lane {
                from: r.to,
                to: r.from,
                left: vec![],
                right: vec![],
            };
            let mut right_lane = Lane {
                from: r.from, 
                to: r.to,
                left: vec![],
                right: vec![],
            };

            let mut point_it = r.points.iter().map(map_func);
            let mut prev_three_points: ThreePoints = point_it.next().unwrap();
            let mut prev_three_point_ids = 
                add_three_points(&mut points, prev_three_points);

            for three_points in point_it {
                let three_point_ids = add_three_points(&mut points, three_points);

                let bezier_ids = 
                    add_bezier_from_two_three_points(
                        &mut beziers,
                        prev_three_points, prev_three_point_ids,
                        three_points, three_point_ids
                    );

                left_lane.right.push(DirectedBezier {
                    bezier_id: bezier_ids.left_id,
                    is_forward: false,
                });
                left_lane.left.push(DirectedBezier {
                    bezier_id: bezier_ids.middle_id,
                    is_forward: false,
                });

                right_lane.left.push(DirectedBezier {
                    bezier_id: bezier_ids.middle_id, 
                    is_forward: true,
                });
                right_lane.right.push(DirectedBezier {
                    bezier_id: bezier_ids.right_id,
                    is_forward: true,
                });

                prev_three_points = three_points;
                prev_three_point_ids = three_point_ids;
            }

            left_lane.left.reverse();
            left_lane.right.reverse();

            lanes.push(left_lane);
            lanes.push(right_lane);
        }

        for s in &backbone.cross_sections {
            let mut left_section = CrossSection {
                from: s.to,
                across: s.across,
                to: s.from,
                left: vec![],
                right: vec![],
            };

            let mut right_section = CrossSection {
                from: s.from,
                across: s.across,
                to: s.to,
                left: vec![],
                right: vec![],
            };

            let mut point_it = s.points.iter().map(map_func);
            let mut prev_three_points: ThreePoints = point_it.next().unwrap();
            let mut prev_three_point_ids = 
                add_three_points(&mut points, prev_three_points);

            for three_points in point_it {
                let three_point_ids = add_three_points(&mut points, three_points);

                let bezier_ids = 
                    add_bezier_from_two_three_points(
                        &mut beziers,
                        prev_three_points, prev_three_point_ids,
                        three_points, three_point_ids
                    );

                left_section.right.push(DirectedBezier {
                    bezier_id: bezier_ids.left_id,
                    is_forward: false,
                });
                left_section.left.push(DirectedBezier {
                    bezier_id: bezier_ids.middle_id,
                    is_forward: false,
                });

                right_section.left.push(DirectedBezier {
                    bezier_id: bezier_ids.middle_id, 
                    is_forward: true,
                });
                right_section.right.push(DirectedBezier {
                    bezier_id: bezier_ids.right_id,
                    is_forward: true,
                });

                prev_three_points = three_points;
                prev_three_point_ids = three_point_ids;
            }

            left_section.left.reverse();
            left_section.right.reverse();

            cross_sections.push(left_section);
            cross_sections.push(right_section);
        }

        Self {
            locations: locations,
            points: points,
            beziers: beziers,
            lanes: lanes,
            cross_sections: cross_sections,

            chosen_path: vec![],
            prev_chosen_path: vec![],
        }
    }

    pub fn chosen_path_changed(&self) -> bool {
        self.prev_chosen_path != self.chosen_path
    }

    pub fn finish(&mut self) {
        self.prev_chosen_path = self.chosen_path.clone();
    }
}

impl Backbone {
    pub fn new() -> Self {
        Self {
            locations: vec![],
            points: vec![],
            roads: vec![],
            cross_sections: vec![],
        }
    }

    pub fn get_point(&self, id: PointId) -> PointBackbone {
        self.points[id.id]
    }

    pub fn add_location(
        &mut self, name: &str) -> LocationId 
    {
        let len = self.locations.len();
        self.locations.push(Location {
            name: String::from(name),
        });
        LocationId { id: len }
    }

    pub fn add_point(
        &mut self, 
        position: (f32, f32),
        direction: (f32, f32)) -> PointId 
    {
        let len = self.points.len();
        self.points.push(PointBackbone {
            position: position,
            direction: direction,
        });
        PointId { id: len }
    }

    pub fn add_road(
        &mut self, 
        from: LocationId, 
        to: LocationId,
        points: &[PointId])
    {
        self.roads.push(RoadBackbone {
            from: from,
            to: to, 
            points: points.to_vec(),   
        });
    }

    pub fn add_cross_section( 
        &mut self, 
        from: LocationId, 
        across: LocationId,
        to: LocationId, 
        points: &[PointId])
    {
        self.cross_sections.push(CrossSectionBackbone {
            from: from,
            across: across,
            to: to,
            points: points.to_vec(),
        });
    }
}
