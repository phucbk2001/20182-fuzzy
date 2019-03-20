pub struct Config {
    pub lane_width: f32,
    pub car_width: f32,
    pub car_length: f32,
    pub camera_width: f32,
}

impl Config {
    pub fn new() -> Config {
        Config {
            lane_width: 3.7,
            car_width: 1.9,
            car_length: 4.115,
            camera_width: 40.0,
        }
    }
}
