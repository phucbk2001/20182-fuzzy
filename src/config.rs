#[allow(dead_code)]
pub struct Config {
    lane_width: f32,
    car_width: f32,
    car_length: f32,
    camera_width: f32,
    window_ratio: f32,
}

#[allow(dead_code)]
impl Config {
    pub fn new() -> Config {
        Config {
            lane_width: 3.7,
            car_width: 1.7,
            car_length: 4.115,
            camera_width: 40.0,
            window_ratio: 1.0,
        }
    }

    pub fn camera_size(&self) -> (f32, f32) {
        let height = self.camera_width / self.window_ratio;
        (self.camera_width, height)
    }
}
