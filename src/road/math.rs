use crate::bezier;
use super::{
    Road, CrossSectionId, LocationId, LaneId,
    street_light_exists,
};
use bezier::{Bezier, Point, Line, dot};

const MAX_INTERSECT_DISTANCE: f32 = 100.0;
const FAR_POINT_DISTANCE: f32 = 100000.0;
const MAX_STREET_LIGHT_ANGLE: f32 = 60.0;

#[derive(Clone)]
pub struct PathProperties {
    pub left_beziers: Vec<Bezier>,
    pub right_beziers: Vec<Bezier>,
    pub far_left_beziers: Vec<Bezier>,
    pub street_lights: Vec<(LaneId, Point)>,
    pub path: Vec<LocationId>,
}

fn path_to_lanes(path: &[LocationId]) 
    -> Vec<(LocationId, LocationId)> 
{
    let mut lanes = Vec::new();
    let mut it = path.iter();
    if let Some(prev_location) = it.next() {
        let mut prev_location = *prev_location;
        for location in it {
            lanes.push((prev_location, *location));
            prev_location = *location;
        }
    }
    lanes
}

fn path_to_cross_sections(path: &[LocationId])
    -> Vec<(LocationId, LocationId, LocationId)>
{
    let mut cross_sections = Vec::new();
    let mut it = path.iter();
    if let Some(prev_prev_location) = it.next() {
        if let Some(prev_location) = it.next() {
            let mut prev_prev_location = *prev_prev_location;
            let mut prev_location = *prev_location;
            for location in it {
                cross_sections.push((prev_prev_location, prev_location, *location));
                prev_prev_location = prev_location;
                prev_location = *location;
            }
        }
    }
    cross_sections
}

fn find_lane(road: &Road, lane: (LocationId, LocationId)) 
    -> LaneId
{
    let (from, to) = lane;
    let (id, _) = road.lanes.iter().enumerate().find(
        |&(_id, lane)| lane.from == from && lane.to == to)
        .expect("road::math Lane doesn't exist");
    LaneId { id }
}

fn find_cross_section(
    road: &Road, cross_section: (LocationId, LocationId, LocationId))
    -> CrossSectionId
{
    let (from, across, to) = cross_section;
    let (id, _) = road.cross_sections.iter().enumerate().find(
        |&(_id, c)| c.from == from && c.to == to && c.across == across)
        .expect("road::math CrossSection doesn't exist");
    CrossSectionId { id }
}

fn too_far(line: Line, bezier: &Bezier) -> bool {
    let p = line.position;
    (bezier.a - p).len() > MAX_INTERSECT_DISTANCE &&
        (bezier.c - p).len() > MAX_INTERSECT_DISTANCE
}

fn new_nearest(
    reference_point: Point, 
    nearest: Option<Point>, 
    candidate: Option<Point>) 
    -> Option<Point>
{
    if let Some(p) = nearest {
        if let Some(new_point) = candidate {
            let prev_len = (p - reference_point).len();
            let len = (new_point - reference_point).len();
            if prev_len > len {
                candidate
            }
            else {
                nearest
            }
        }
        else {
            nearest
        }
    }
    else {
        candidate
    }
}

fn intersect_line_beziers(
    line: Line, beziers: &Vec<Bezier>) 
    -> Option<Point>
{
    let not_too_far = |bezier: &&Bezier| { !too_far(line, *bezier) };
    let fold_fn =
        |nearest: Option<Point>, bezier: &Bezier| {
            let p = bezier::intersect_line_bezier(line, *bezier);
            new_nearest(line.position, nearest, p)
        };

    beziers.iter()
        .filter(not_too_far)
        .fold(None, fold_fn)
}


impl PathProperties {
    pub fn new(road: &Road, path: &[LocationId]) -> Self
    {
        let lanes = path_to_lanes(path);
        let cross_sections = path_to_cross_sections(path);

        let mut reverse_path = path.to_vec(); reverse_path.reverse();
        let left_lanes = path_to_lanes(&reverse_path);
        let left_cross_sections = path_to_cross_sections(&reverse_path);

        let mut left_beziers = Vec::new();
        let mut right_beziers = Vec::new();
        let mut far_left_beziers = Vec::new();

        let mut street_lights = Vec::new();

        for lane in lanes.iter() {
            let lane = find_lane(road, *lane);
            let lane_ref = &road.lanes[lane.id];

            for bezier in lane_ref.left.iter() {
                let bezier = road.get_bezier(*bezier);
                left_beziers.push(bezier);
            }
            for bezier in lane_ref.right.iter() {
                let bezier = road.get_bezier(*bezier);
                right_beziers.push(bezier);
            }

            if street_light_exists(road, lane) {
                let last_bezier = lane_ref.right.last().unwrap();
                let last_bezier = road.get_bezier(*last_bezier);
                street_lights.push((lane, last_bezier.c));
            }
        }

        for cs in cross_sections.iter() {
            let cs = find_cross_section(road, *cs);
            let cs_ref = &road.cross_sections[cs.id];

            for bezier in cs_ref.left.iter() {
                let bezier = road.get_bezier(*bezier);
                left_beziers.push(bezier);
            }
            for bezier in cs_ref.right.iter() {
                let bezier = road.get_bezier(*bezier);
                right_beziers.push(bezier);
            }
        }

        for lane in left_lanes.iter() {
            let lane = find_lane(road, *lane);
            let lane_ref = &road.lanes[lane.id];

            for bezier in lane_ref.right.iter() {
                let bezier = road.get_bezier(*bezier);
                far_left_beziers.push(bezier);
            }
        }

        for cs in left_cross_sections.iter() {
            let cs = find_cross_section(road, *cs);
            let cs_ref = &road.cross_sections[cs.id];

            for bezier in cs_ref.right.iter() {
                let bezier = road.get_bezier(*bezier);
                far_left_beziers.push(bezier);
            }
        }

        Self {
            left_beziers,
            right_beziers,
            far_left_beziers,
            street_lights,
            path: path.to_vec(),
        }
    }

    pub fn nearest_intersection(&self, line: Line)
        -> (Point, Point, Point)
    {
        let nearest_left = intersect_line_beziers(
            line, &self.left_beziers);

        let nearest_right = intersect_line_beziers(
            line, &self.right_beziers);

        let nearest_far_left = intersect_line_beziers(
            line, &self.far_left_beziers);

        let dir = line.direction;
        let pos = line.position;
        let nearest_left = nearest_left.unwrap_or(
            pos - FAR_POINT_DISTANCE * dir);
        let nearest_right = nearest_right.unwrap_or(
            pos + FAR_POINT_DISTANCE * dir);
        let nearest_far_left = nearest_far_left.unwrap_or(
            pos - FAR_POINT_DISTANCE * dir);

        (nearest_left, nearest_right, nearest_far_left)
    }
}

pub fn nearest_street_light(
    street_lights: &Vec<(LaneId, Point)>,
    pos: Point,
    dir: Point)
    -> Option<(LaneId, Point)>
{
    let dir = dir.normalize();

    let mut min_distance = 1000.0;
    let mut min_lane: Option<LaneId> = None;
    let mut min_point: Option<Point> = None;

    let min_cos = f32::cos(std::f32::consts::PI * MAX_STREET_LIGHT_ANGLE / 180.0);

    for light in street_lights.iter() {
        let (lane, p) = *light;
        let light_dir = (p - pos).normalize();
        let distance = (pos - p).len();

        if distance < min_distance && dot(dir, light_dir) >= min_cos {
            min_distance = distance;
            min_lane = Some(lane);
            min_point = Some(p);
        }
    }

    let min_lane = min_lane?;
    let min_point = min_point?;
    Some((min_lane, min_point))
}

impl Default for PathProperties {
    fn default() -> Self {
        Self {
            left_beziers: Vec::new(),
            right_beziers: Vec::new(),
            far_left_beziers: Vec::new(),
            street_lights: Vec::new(),
            path: Vec::new(),
        }
    }
}

fn add_straight_bezier(a: Point, c: Point, beziers: &mut Vec<Bezier>) {
    let middle = (a + c) * 0.5;
    let bezier = Bezier { a, b: middle, c };
    beziers.push(bezier);
}

fn enclosed_left_right_beziers(road: &Road, lane: LaneId)
    -> (Vec<Bezier>, Vec<Bezier>)
{
    let mut left_beziers: Vec<Bezier> =
        road.lanes[lane.id].left.iter()
        .map(|&bezier| {
            road.get_bezier(bezier)
        })
        .collect();

    let mut right_beziers: Vec<Bezier> =
        road.lanes[lane.id].right.iter()
        .map(|&bezier| {
            road.get_bezier(bezier)
        })
        .collect();

    let first_left = *left_beziers.iter().next().unwrap();
    let first_right = *right_beziers.iter().next().unwrap();
    let last_left = *left_beziers.last().unwrap();
    let last_right = *right_beziers.last().unwrap();

    add_straight_bezier(
        first_left.a,
        first_right.a,
        &mut right_beziers);

    add_straight_bezier(
        last_left.c,
        last_right.c,
        &mut left_beziers);

    (left_beziers, right_beziers)
}

fn is_middle_of(
    reference: Point,
    left: Option<Point>,
    right: Option<Point>)
    -> bool 
{
    if let Some(p1) = left {
        if let Some(p2) = right {
            let v1 = p1 - reference;
            let v2 = p2 - reference;
            bezier::dot(v1, v2) < 0.0
        }
        else {
            false
        }
    }
    else {
        false
    }
}

fn is_inside_lane(road: &Road, pos: Point, lane: LaneId) -> bool {
    let (left_beziers, right_beziers) =
        enclosed_left_right_beziers(road, lane);
    let line1 = Line {
        position: pos,
        direction: Point { x: 1.0, y: 0.0 },
    };
    let line2 = Line {
        position: pos,
        direction: Point { x: 0.0, y: 1.0 },
    };

    let left1 = intersect_line_beziers(line1, &left_beziers);
    let right1 = intersect_line_beziers(line1, &right_beziers);
    let left2 = intersect_line_beziers(line2, &left_beziers);
    let right2 = intersect_line_beziers(line2, &right_beziers);

    is_middle_of(pos, left1, right1) || is_middle_of(pos, left2, right2)
}

pub fn find_lane_contains(road: &Road, p: Point)
    -> Option<LaneId>
{
    (0..(road.lanes.len())).into_iter()
        .map(|id| { LaneId { id } })
        .filter(|&lane| is_inside_lane(road, p, lane))
        .next()
}

pub fn direction_in_lane_of(road: &Road, lane: LaneId, p: Point) 
    -> Point
{
    let first_left: Point = road.get_bezier(road.lanes[lane.id].left[0]).a;
    let first_right: Point = road.get_bezier(road.lanes[lane.id].right[0]).a;

    let last_left: Point = road.get_bezier(
        *road.lanes[lane.id].left.last().unwrap()).c;
    let last_right: Point = road.get_bezier(
        *road.lanes[lane.id].right.last().unwrap()).c;

    let dfirst = ((first_left + first_right) * 0.5 - p).len();
    let dlast = ((last_left + last_right) * 0.5 - p).len();

    let dir = (first_left - first_right) * dlast + (last_left - last_right) * dfirst;
    dir.normalize().turn_right_90_degree()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_to_lanes() {
        let a = LocationId { id: 0 };
        let b = LocationId { id: 1 };
        let c = LocationId { id: 2 };
        let d = LocationId { id: 3 };

        let path = vec![a, b, c, d];
        let lanes = path_to_lanes(&path);
        assert_eq!(lanes.len(), 3);
        assert_eq!(lanes[0], (a, b));
        assert_eq!(lanes[1], (b, c));
        assert_eq!(lanes[2], (c, d));
    }

    #[test]
    fn test_path_to_cross_sections() {
        let a = LocationId { id: 0 };
        let b = LocationId { id: 1 };
        let c = LocationId { id: 2 };
        let d = LocationId { id: 3 };
        let e = LocationId { id: 4 };

        let path = vec![a, b, c, d, e];
        let cross_sections = path_to_cross_sections(&path);
        assert_eq!(cross_sections.len(), 3);
        assert_eq!(cross_sections[0], (a, b, c));
        assert_eq!(cross_sections[1], (b, c, d));
        assert_eq!(cross_sections[2], (c, d, e));
    }
}
