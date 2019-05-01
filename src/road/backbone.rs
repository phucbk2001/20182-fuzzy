use super::*;
use crate::bezier::Point;

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
    backbone: &Backbone, config: &Config,
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
    points: &mut Vec<Point>, three_points: ThreePoints) 
    -> ThreePointIds
{
    let len = points.len();

    let point = three_points.left_pos;
    points.push(point);

    let point = three_points.middle_pos;
    points.push(point);

    let point = three_points.right_pos;
    points.push(point);

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

fn add_point(points: &mut Vec<Point>, point: Point) -> PointId {
    let id = points.len();
    points.push(point);
    PointId { id }
}

fn add_bezier(beziers: &mut Vec<Bezier>, bezier: Bezier) -> BezierId {
    let id = beziers.len();
    beziers.push(bezier);
    BezierId { id }
}


fn turn_back_map_fn(
    input: (Point, Point, LocationId, LocationId), 
    config: &Config,
    points: &mut Vec<Point>,
    beziers: &mut Vec<Bezier>)
    -> CrossSection
{
    let (dir, pos, from, to) = input;

    let phi = std::f32::consts::PI / 4.0;
    let phi2 = phi / 2.0;

    let ey = dir;
    let ex = dir.turn_right_90_degree();
    
    let mut point_it = (0..5).into_iter()
        .map(|i| {
            let angle = (i as f32) * phi;
            let x = config.lane_width * f32::cos(angle);
            let y = config.lane_width * f32::sin(angle);
            pos + x * ex + y * ey
        });

    let middle_point_it = (0..5).into_iter()
        .map(|i| {
            let angle = (i as f32) * phi + phi2;
            let radius = config.lane_width / f32::cos(phi2);
            let x = radius * f32::cos(angle);
            let y = radius * f32::sin(angle);
            pos + x * ex + y * ey
        });

    let prev_point: Point = point_it.next().unwrap();
    let mut prev_point_id = add_point(points, prev_point);

    let center_point_id = add_point(points, pos);

    let center_bezier = add_bezier(beziers,
        Bezier {
            point1: center_point_id,
            middle: pos,
            point2: center_point_id,
        }
    );

    let directed_center_bezier = 
        DirectedBezier { 
            is_forward: true,
            bezier: center_bezier,
        };

    let mut left = Vec::<DirectedBezier>::new();
    let mut right = Vec::<DirectedBezier>::new();

    for (point, middle) in point_it.zip(middle_point_it) {
        let point_id = add_point(points, point);
        let bezier = add_bezier(
            beziers,
            Bezier {
                point1: prev_point_id,
                middle,
                point2: point_id,
            });

        let directed_bezier = 
            DirectedBezier {
                is_forward: true,
                bezier,
            };

        left.push(directed_center_bezier);
        right.push(directed_bezier);

        prev_point_id = point_id;
    }

    CrossSection {
        from,
        across: to,
        to: from,
        left,
        right,
    }
}

fn get_last_left_point_of(
    lanes: &Vec<Lane>,
    beziers: &Vec<Bezier>,
    points: &Vec<Point>,
    lane: LaneId)
    -> Point
{
    let last_bezier = *lanes[lane.id].left.last().unwrap();
    let bezier = get_bezier_from_beziers(
        beziers, points, last_bezier);
    bezier.c
}

impl Road {
    pub fn from(backbone: &Backbone, config: &Config) -> Self {
        let mut locations = backbone.locations.clone();
        let mut points: Vec<Point> = vec![];
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
                    bezier: bezier_ids.left_id,
                    is_forward: false,
                });
                left_lane.left.push(DirectedBezier {
                    bezier: bezier_ids.middle_id,
                    is_forward: false,
                });

                right_lane.left.push(DirectedBezier {
                    bezier: bezier_ids.middle_id, 
                    is_forward: true,
                });
                right_lane.right.push(DirectedBezier {
                    bezier: bezier_ids.right_id,
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
                    bezier: bezier_ids.left_id,
                    is_forward: false,
                });
                left_section.left.push(DirectedBezier {
                    bezier: bezier_ids.middle_id,
                    is_forward: false,
                });

                right_section.left.push(DirectedBezier {
                    bezier: bezier_ids.middle_id, 
                    is_forward: true,
                });
                right_section.right.push(DirectedBezier {
                    bezier: bezier_ids.right_id,
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


        let last_cross_section_it = backbone.roads.iter()
            .map(|r| {
                let last = r.points.iter().last()
                    .expect("backbone last: road.points can't be empty");

                let last_dir = backbone.points[last.id].direction.normalize();
                let last_pos = backbone.points[last.id].position;

                (last_dir, last_pos, r.from, r.to)
            })
            .map(|input| {
                turn_back_map_fn(input, config, &mut points, &mut beziers)
            });

        cross_sections.extend(last_cross_section_it);

        let first_cross_section_it = backbone.roads.iter()
            .map(|r| {
                let first = r.points.iter().next()
                    .expect("backbone first: road.points can't be empty");

                let first_dir = backbone.points[first.id].direction.normalize();
                let first_pos = backbone.points[first.id].position;

                (-1.0 * first_dir, first_pos, r.to, r.from)
            })
            .map(|input| {
                turn_back_map_fn(input, config, &mut points, &mut beziers)
            });

        cross_sections.extend(first_cross_section_it);

        for (id, lane) in lanes.iter().enumerate() {
            let id = LaneId { id };
            let location = lane.to;
            locations[location.id].incoming_lanes.push(id);
        }

        for location in locations.iter_mut() {
            location.position =
                location.incoming_lanes.iter()
                .fold(Point { x: 0.0, y: 0.0 }, |point, lane| {
                    point + get_last_left_point_of(
                        &lanes, &beziers, &points, *lane)
                });
            location.position = location.position *
                (1.0 / location.incoming_lanes.len() as f32);
        }

        for location in locations.iter_mut() {
            location.adjacents = location.incoming_lanes.iter()
                .map(|&lane| {
                    lanes[lane.id].from
                })
                .collect();
        }

        Self {
            locations,
            points,
            beziers,
            lanes,
            cross_sections,

            chosen_path: vec![],
            prev_chosen_path: vec![],

            prev_instant: std::time::Instant::now(),
        }
    }

    pub fn new() -> Self {
        Self {
            locations: Vec::new(),
            points: Vec::new(),
            beziers: Vec::new(),
            lanes: Vec::new(),
            cross_sections: Vec::new(),

            chosen_path: Vec::new(),
            prev_chosen_path: Vec::new(),

            prev_instant: std::time::Instant::now(),
        }
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
        &mut self, name: &str, config: &Config) -> LocationId 
    {
        let len = self.locations.len();
        self.locations.push(Location {
            name: String::from(name),
            incoming_lanes: Vec::new(),
            street_light_index: 0,
            street_light_color: StreetLightColor::Green,
            street_light_time: random_green_time(config),
            position: Point { x: 0.0, y: 0.0 },
            adjacents: Vec::new(),
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
            position: Point::from(position),
            direction: Point::from(direction),
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
