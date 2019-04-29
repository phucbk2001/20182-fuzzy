use crate::config::Config;
use crate::road::{Road, Backbone};
use crate::car::{CarSystem, Car};

pub fn init(config: &Config) -> (Road, CarSystem) {
    let mut backbone = Backbone::new();

    let location_a = backbone.add_location("A", &config);
    let location_b = backbone.add_location("B", &config);
    let location_c = backbone.add_location("C", &config);
    let location_d = backbone.add_location("D", &config);

    let p1 = backbone.add_point((-20.0, -40.0), (0.0, 3.0));
    let p2 = backbone.add_point((-10.0, -10.0), (1.0, 2.0));
    let p3 = backbone.add_point((0.0, 0.0), (2.0, 1.0));
    let p4 = backbone.add_point((30.0, 13.0), (1.0, 0.0));
    let p5 = backbone.add_point((13.0, 30.0), (0.0, 1.0));
    let p6 = backbone.add_point((70.0, 13.0), (1.0, 0.0));
    let p7 = backbone.add_point((13.0, 30.0), (0.0, -1.0));
    let p8 = backbone.add_point((7.0, 60.0), (-0.5, 1.0));

    backbone.add_road(location_a, location_b, &[p1, p2, p3]);
    backbone.add_road(location_b, location_c, &[p4, p6]);
    backbone.add_road(location_b, location_d, &[p5, p8]);

    backbone.add_cross_section(
        location_a, location_b, location_c,
        &[p3, p4]);

    backbone.add_cross_section(
        location_a, location_b, location_d,
        &[p3, p5]);

    backbone.add_cross_section(
        location_d, location_b, location_c,
        &[p7, p4]);

    let mut road = Road::from(&backbone, &config);

    road.chosen_path = 
        vec![location_a, location_b, location_a];

    let mut car_system = CarSystem::new();
    let car = Car::from_path(&road, &[location_a, location_b, location_a]);
    car_system.add(car);

    let car = Car::from_path(&road, &[location_d, location_b, location_c]);
    car_system.add(car);

    let car = Car::from_path(&road, &[location_c, location_b, location_a]);
    car_system.add(car);

    (road, car_system)
}
