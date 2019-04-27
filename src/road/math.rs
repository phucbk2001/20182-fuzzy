use crate::bezier;
use crate::road;

use road::{Road, LocationId};
use bezier::{Point, Line};

pub struct PathProperties {
    pub left_beziers: Vec<bezier::Bezier>,
    pub right_beziers: Vec<bezier::Bezier>,
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

pub fn compute_path_properties(road: &Road, path: &[LocationId]) 
{
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
