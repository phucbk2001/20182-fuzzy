#[allow(dead_code)]
pub struct Config {
    pub lane_width: f32,
    pub car_width: f32,
    pub car_length: f32,
    camera_width: f32,
}

#[allow(dead_code)]
impl Config {
    pub fn new() -> Config {
        Config {
            lane_width: 3.7,
            car_width: 1.7,
            car_length: 4.115,
            camera_width: 40.0,
        }
    }

    pub fn get_camera_size(&self, dims: (u32, u32)) -> (f32, f32) {
        let (w, h) = dims;
        let ratio = w as f32 / h as f32;
        let height = self.camera_width / ratio;
        (self.camera_width, height)
    }
}
