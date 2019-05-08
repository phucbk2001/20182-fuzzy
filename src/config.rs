pub struct Config {
    pub lane_width: f32,
    pub car_width: f32,
    pub car_length: f32,
    pub front_wheel: f32,
    pub rear_wheel: f32,
    pub camera_width: f32,
    pub streetlight_size: f32,
    pub streetlight_distance: f32,
    pub min_green_duration: f32,
    pub max_green_duration: f32,
    pub location_mark_width: f32,
    pub location_mark_height: f32,
}

impl Config {
    pub fn new() -> Config {
        Config {
            lane_width: 3.7,
            car_width: 1.6,
            car_length: 3.5,
            front_wheel: 1.0,
            rear_wheel: 1.1,
            camera_width: 40.0,
            streetlight_size: 2.0,
            streetlight_distance: 2.0,
            min_green_duration: 6.0,
            max_green_duration: 10.0,
            location_mark_width: 2.5,
            location_mark_height: 4.0,
        }
    }
}
