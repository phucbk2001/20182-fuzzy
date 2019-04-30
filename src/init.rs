use crate::config::Config;
use crate::road::{
    Road, Backbone,
    LocationId, PointId,
};
use crate::car::{CarSystem, Car};

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashMap;

#[derive(Copy, Clone)]
enum ReadingState {
    Location,
    Point,
    Road,
    CrossSection,
    ChosenPath,
    Car,
    Unrecognized,
}

fn is_special_line(line: &String) -> Option<ReadingState> {
    let mut word_it = line.split_whitespace();
    if let Some(word) = word_it.next() {
        if word == "=locations" {
            Some(ReadingState::Location)
        }
        else if word == "=points" {
            Some(ReadingState::Point)
        }
        else if word == "=roads" {
            Some(ReadingState::Road)
        }
        else if word == "=cross_sections" {
            Some(ReadingState::CrossSection)
        }
        else if word == "=chosen_path" {
            Some(ReadingState::ChosenPath)
        }
        else if word == "=cars" {
            Some(ReadingState::Car)
        }
        else if word.as_bytes()[0] == '=' as u8 {
            Some(ReadingState::Unrecognized)
        }
        else {
            None
        }
    }
    else {
        None
    }
}

fn read_locations(
    backbone: &mut Backbone,
    location_map: &mut HashMap<String, LocationId>,
    line: &String,
    config: &Config)
{
    let mut word_it = line.split_whitespace();
    if let Some(name) = word_it.next() {
        if location_map.contains_key(name) {
            println!("Warning: Location's name have already exist");
        }
        else {
            let location = backbone.add_location(&name, config);
            location_map.insert(name.to_string(), location);
        }
    }
}

fn read_points(
    backbone: &mut Backbone,
    point_map: &mut HashMap<String, PointId>,
    line: &String)
{
    let mut word_it = line.split_whitespace();
    if let Some(name) = word_it.next() {
        if point_map.contains_key(name) {
            println!("Warning: Point's name have already exist");
        }
        else {
            let x: f32 = word_it.next().unwrap().parse::<f32>().unwrap();
            let y: f32 = word_it.next().unwrap().parse::<f32>().unwrap();
            let vx: f32 = word_it.next().unwrap().parse::<f32>().unwrap();
            let vy: f32 = word_it.next().unwrap().parse::<f32>().unwrap();
            let point = backbone.add_point((x, y), (vx, vy));
            point_map.insert(name.to_string(), point);
        }
    }
}

fn read_roads(
    backbone: &mut Backbone,
    location_map: &HashMap<String, LocationId>,
    point_map: &HashMap<String, PointId>,
    line: &String)
{
    let mut word_it = line.split_whitespace();
    if let Some(from) = word_it.next() {
        let to = word_it.next().unwrap();

        let from: LocationId = *location_map.get(from).expect("Location doesn't exist");
        let to: LocationId = *location_map.get(to).expect("Location doesn't exist");

        let mut points = Vec::<PointId>::new();
        for point_name in word_it {
            let point = *point_map.get(point_name).expect("Point doesn't exist");
            points.push(point);
        }
        backbone.add_road(from, to, &points);
    }
}

fn read_cross_sections(
    backbone: &mut Backbone,
    location_map: &HashMap<String, LocationId>,
    point_map: &HashMap<String, PointId>,
    line: &String)
{
    let mut word_it = line.split_whitespace();
    if let Some(from) = word_it.next() {
        let across = word_it.next().unwrap();
        let to = word_it.next().unwrap();

        let from: LocationId = *location_map.get(from).expect("Location doesn't exist");
        let across: LocationId = *location_map.get(across).expect("Location doesn't exist");
        let to: LocationId = *location_map.get(to).expect("Location doesn't exist");

        let mut points = Vec::<PointId>::new();
        for point_name in word_it {
            let point = *point_map.get(point_name).expect("Point doesn't exist");
            points.push(point);
        }
        backbone.add_cross_section(from, across, to, &points);
    }
}

fn read_chosen_path(
    road: &mut Road,
    location_map: &HashMap<String, LocationId>,
    line: &String)
{
    let mut word_it = line.split_whitespace();
    let mut path = Vec::<LocationId>::new();
    if let Some(a) = word_it.next() {
        let location = *location_map.get(a).unwrap();
        path.push(location);

        for name in word_it {
            let location = *location_map.get(name).unwrap();
            path.push(location);
        }
        road.chosen_path = path;
    }
}

fn read_cars(
    car_system: &mut CarSystem,
    road: &Road,
    location_map: &HashMap<String, LocationId>,
    line: &String)
{
    let mut word_it = line.split_whitespace();
    let mut path = Vec::<LocationId>::new();
    if let Some(a) = word_it.next() {
        let location = *location_map.get(a).unwrap();
        path.push(location);

        for name in word_it {
            let location = *location_map.get(name).unwrap();
            path.push(location);
        }
    }
    let car = Car::from_path(road, &path);
    car_system.add(car);
}

fn read_file(
    backbone: &mut Backbone, 
    car_system: &mut CarSystem,
    config: &Config)
    -> Road
{
    let mut road = Road::new();

    let f = File::open("assets/map")
        .expect("File \"assets/map\" doesn't exist");

    let f = BufReader::new(f);

    let mut state: ReadingState = ReadingState::Location;

    let mut location_map = HashMap::<String, LocationId>::new();
    let mut point_map = HashMap::<String, PointId>::new();

    for line in f.lines() {
        let line = line.expect("Error while reading assets/map file");
        if let Some(s) = is_special_line(&line) {
            state = s;
            match state {
                ReadingState::ChosenPath => {
                    road = Road::from(backbone, config);
                },
                ReadingState::Unrecognized => 
                    println!("Warning: Unrecognized section's name"),
                _ => (),
            }
        }
        else {
            match state {
                ReadingState::Location =>
                    read_locations(backbone, &mut location_map, &line, config),

                ReadingState::Point =>
                    read_points(backbone, &mut point_map, &line),

                ReadingState::Road =>
                    read_roads(backbone, &location_map, &point_map, &line),
                
                ReadingState::CrossSection =>
                    read_cross_sections(backbone, &location_map, &point_map, &line),

                ReadingState::ChosenPath =>
                    read_chosen_path(&mut road, &location_map, &line),

                ReadingState::Car => 
                    read_cars(car_system, &road, &location_map, &line),

                ReadingState::Unrecognized => (),
            }
        }
    }

    road
}

pub fn init(config: &Config) -> (Road, CarSystem) {
    let mut backbone = Backbone::new();
    let mut car_system = CarSystem::new();

    let road = read_file(&mut backbone, &mut car_system, config);

    (road, car_system)
}
