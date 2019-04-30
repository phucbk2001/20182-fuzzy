use nalgebra as na;
use crate::bezier::Point;
use crate::config;

const X2_ROOM_STEP: i32 = 8;
const MAX_ROOM_IN: i32 = 16;
const MAX_ROOM_OUT: i32 = 32;

pub struct Camera {
    old_position: Point,
    position: Point,

    room_scale: i32,
    room_scale_value: f32,
    
    logical_window_size: (f32, f32),
    camera_size: (f32, f32),

    matrix: na::Matrix4<f32>
}

fn compute_view(position: Point) 
    -> na::Isometry3<f32>
{
    let Point { x, y } = position;
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
    position: Point,
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
        let pos = Point { x: 0.0, y: 0.0 };

        let matrix = projection_view_matrix(
            pos, default_camera_size, 1.0);

        Self {
            old_position: pos,
            position: pos, 
            room_scale: 0,
            room_scale_value: 1.0,
            logical_window_size: (600.0, 600.0),
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

    pub fn set_logical_window_size(
        &mut self, width: f32, height: f32, config: &config::Config) 
    {
        self.logical_window_size = (width, height);

        let (w, h) = (width, height);
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
        &self, x: f32, y: f32) -> Point
    {
        let (w, h) = self.logical_window_size;
        let x = (2.0 * x - w) / w;
        let y = (h - 2.0 * y) / h;
        let (camera_width, camera_height) = self.camera_size;
        let x = x * camera_width / 2.0 * self.room_scale_value;
        let y = y * camera_height / 2.0 * self.room_scale_value;
        Point { x, y }
    }

    pub fn screen_coords_to_real_position(
        &self, x: f32, y: f32) -> Point 
    {
        self.screen_coords_to_world(x, y) + self.position
    }

    pub fn get_old_position(&self) -> Point {
        self.old_position
    }

    pub fn set_temp_position(&mut self, p: Point) {
        self.position = p;
        self.update();
    }

    pub fn set_position(&mut self, p: Point) {
        self.position = p;
        self.old_position = self.position;
        self.update();
    }
}

#[test]
fn view_multiply_simple_point() {
    use approx::assert_relative_eq;

    let pos = Point { x: 0.0, y: 0.0 };
    let view = compute_view(pos).to_homogeneous();
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
