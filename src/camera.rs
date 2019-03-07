use nalgebra as na;
use crate::bezier;
use crate::config;

const X2_ROOM_STEP: i32 = 8;
const MAX_ROOM_IN: i32 = 16;
const MAX_ROOM_OUT: i32 = 32;

pub struct Camera {
    position: [f32; 2],

    room_scale: i32,
    room_scale_value: f32,
    
    dimensions: (u32, u32),
    camera_size: (f32, f32),

    matrix: na::Matrix4<f32>
}

fn compute_view(position: [f32; 2]) 
    -> na::Isometry3<f32>
{
    let [x, y] = position;
    let eye = na::Point3::new(x, y, 10.0);
    let target = na::Point3::new(x, y, 0.0);
    let up = na::Vector3::new(0.0, 1.0, 0.0);

    na::Isometry3::look_at_rh(&eye, &target, &up)
}

fn compute_projection(
    camera_size: (f32, f32), scale: f32) 
    -> na::Orthographic3<f32>
{
    let (width, height) = camera_size;
    let (width, height) = (width * scale, height * scale);

    na::Orthographic3::new(
        -width / 2.0, width / 2.0, 
        -height / 2.0, height / 2.0,
        0.0, 20.0
    )
}

fn projection_view_matrix(
    position: [f32; 2], 
    camera_size: (f32, f32),
    scale: f32,
    )
    -> na::Matrix4<f32>
{
    let proj = compute_projection(camera_size, scale);
    let view = compute_view(position);
    proj.as_matrix() * view.to_homogeneous()
}

impl Camera {
    pub fn new(default_camera_size: (f32, f32)) -> Self {
        let pos = [0.0, 0.0];

        let matrix = projection_view_matrix(
            pos, default_camera_size, 1.0);

        Self {
            position: pos, 
            room_scale: 0,
            room_scale_value: 1.0,
            dimensions: (600, 600),
            camera_size: default_camera_size,
            matrix: matrix,
        }
    }

    fn update(&mut self) {
        self.room_scale_value = f32::exp2(
            self.room_scale as f32 / X2_ROOM_STEP as f32);

        self.matrix = projection_view_matrix(
            self.position, self.camera_size, 
            self.room_scale_value);
    }

    pub fn set_dimensions(
        &mut self, dims: (u32, u32), config: &config::Config) 
    {
        self.dimensions = dims;

        let (w, h) = dims;
        let ratio = w as f32 / h as f32;
        let height = config.camera_width / ratio;

        self.camera_size = (config.camera_width, height);
        self.update();
    }

    pub fn increase_room_scale(
        &mut self, room_scale_delta: i32) 
    {
        self.room_scale += room_scale_delta;

        if self.room_scale > MAX_ROOM_OUT {
            self.room_scale = MAX_ROOM_OUT;
        }
        else if self.room_scale < -MAX_ROOM_IN {
            self.room_scale = -MAX_ROOM_IN;
        }

        self.update();
    }

    pub fn get_matrix(&self) -> &na::Matrix4<f32> {
        &self.matrix
    }

    pub fn screen_coords_to_world(
        &self, x: f32, y: f32) -> bezier::Point
    {
        let (w, h) = self.dimensions;
        let w = w as f32;
        let h = h as f32;

        let x = 2.0 * x / w - 1.0;
        let y = 1.0 - 2.0 * y / h;
        let (camera_width, camera_height) = self.camera_size;
        let x = x * camera_width / 2.0 * self.room_scale_value;
        let y = y * camera_height / 2.0 * self.room_scale_value;
        bezier::Point{ x: x, y: y }
    }
}

#[test]
fn view_multiply_simple_point() {
    use approx::assert_relative_eq;

    let view = compute_view([0.0, 0.0]).to_homogeneous();
    let p = na::Point4::new(1.0, 2.0, 0.0, 1.0);
    let q = view * p;

    assert_relative_eq!(q.x, 1.0);
    assert_relative_eq!(q.y, 2.0);
    assert_relative_eq!(q.z, -10.0);
}

#[test]
fn power_of_two_simple() {
    use approx::assert_relative_eq;

    assert_relative_eq!(f32::exp2(3.0), 8.0);
    assert_relative_eq!(f32::exp2(-2.0), 1.0 / 4.0);
}
