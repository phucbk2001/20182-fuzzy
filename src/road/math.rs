use crate::bezier;
use super::{Road, CrossSectionId, LocationId, LaneId};
use bezier::{Bezier, Point, Line, dot};

const MAX_INTERSECT_DISTANCE: f32 = 100.0;
const FAR_POINT: Point = Point { x: 100000.0, y: 100000.0 };
const MAX_STREET_LIGHT_ANGLE: f32 = 60.0;

#[derive(Clone)]
pub struct PathProperties {
    pub left_beziers: Vec<Bezier>,
    pub right_beziers: Vec<Bezier>,
    pub street_lights: Vec<(LaneId, Point)>,
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
    candidate: Point) 
    -> Option<Point>
{
    if let Some(p) = nearest {
        if (p - reference_point).len() > (candidate - reference_point).len() {
            Some(candidate)
        }
        else {
            nearest
        }
    }
    else {
        Some(candidate)
    }
}

impl PathProperties {
    pub fn new(road: &Road, path: &[LocationId]) -> Self
    {
        let lanes = path_to_lanes(path);
        let cross_sections = path_to_cross_sections(path);

        let lane_it = lanes.iter();
        let cs_it = cross_sections.iter();

        let mut left_beziers = Vec::new();
        let mut right_beziers = Vec::new();

        let mut street_lights = Vec::new();

        for lane in lane_it {
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

            let last_bezier = lane_ref.right.last().unwrap();
            let last_bezier = road.get_bezier(*last_bezier);
            street_lights.push((lane, last_bezier.c));
        }

        for cs in cs_it {
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

        Self {
            left_beziers,
            right_beziers,
            street_lights,
        }
    }

    pub fn nearest_intersection(&self, line: Line) -> (Point, Point) {
        let candidate_left_it = self.left_beziers.iter()
            .filter(|&bezier| !too_far(line, bezier));
        let candidate_right_it = self.right_beziers.iter()
            .filter(|&bezier| !too_far(line, bezier));

        let mut nearest_left = None;
        for bezier in candidate_left_it {
            if let Some(p) = bezier::intersect_line_bezier(line, *bezier) {
                nearest_left = new_nearest(line.position, nearest_left, p);
            }
        }

        let mut nearest_right = None;
        for bezier in candidate_right_it {
            if let Some(p) = bezier::intersect_line_bezier(line, *bezier) {
                nearest_right = new_nearest(line.position, nearest_right, p);
            }
        }

        let nearest_left = nearest_left.unwrap_or(FAR_POINT);
        let nearest_right = nearest_right.unwrap_or(FAR_POINT);
        (nearest_left, nearest_right)
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
            street_lights: Vec::new(),
        }
    }
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
