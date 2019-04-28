pub struct Config {
    pub lane_width: f32,
    pub car_width: f32,
    pub car_length: f32,
    pub front_wheel: f32,
    pub rear_wheel: f32,
    pub camera_width: f32,
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
        }
    }
}
