use crate::bezier;
use super::{Road, Lane, CrossSection, LocationId};
use bezier::{Bezier, Point, Line};

const MAX_INTERSECT_DISTANCE: f32 = 100.0;
const FAR_POINT: Point = Point { x: 100000.0, y: 100000.0 };

#[derive(Clone)]
pub struct PathProperties {
    pub left_beziers: Vec<Bezier>,
    pub right_beziers: Vec<Bezier>,
    pub street_lights: Vec<Point>,
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

fn find_lane(road: &Road, lane: (LocationId, LocationId)) -> &Lane {
    let (from, to) = lane;
    let lane = road.lanes.iter().find(
        |&lane| lane.from == from && lane.to == to)
        .expect("road::math Lane doesn't exist");
    lane
}

fn find_cross_section(
    road: &Road, cross_section: (LocationId, LocationId, LocationId))
    -> &CrossSection
{
    let (from, across, to) = cross_section;
    road.cross_sections.iter().find(
        |&c| c.from == from && c.to == to && c.across == across)
        .expect("road::math CrossSection doesn't exist")
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
            for bezier in lane.left.iter() {
                let bezier = road.get_bezier(*bezier);
                left_beziers.push(bezier);
            }
            for bezier in lane.right.iter() {
                let bezier = road.get_bezier(*bezier);
                right_beziers.push(bezier);
            }
            let last_bezier = lane.right.last().unwrap();
            let last_bezier = road.get_bezier(*last_bezier);
            street_lights.push(last_bezier.c);
        }

        for cs in cs_it {
            let cs = find_cross_section(road, *cs);
            for bezier in cs.left.iter() {
                let bezier = road.get_bezier(*bezier);
                left_beziers.push(bezier);
            }
            for bezier in cs.right.iter() {
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
