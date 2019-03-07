use nalgebra as na;

const X2_ROOM_STEP: i32 = 8;
const MAX_ROOM_IN: i32 = 16;
const MAX_ROOM_OUT: i32 = 32;

pub struct Camera {
    position: [f32; 2],
    room_scale: i32,
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
    camera_size: (f32, f32), room_scale: i32) 
    -> na::Orthographic3<f32>
{
    let (width, height) = camera_size;
    let ratio = f32::exp2(room_scale as f32 / X2_ROOM_STEP as f32);
    let (width, height) = (width * ratio, height * ratio);

    na::Orthographic3::new(
        -width / 2.0, width / 2.0, 
        -height / 2.0, height / 2.0,
        0.0, 20.0
    )
}

fn projection_view_matrix(
    position: [f32; 2], 
    camera_size: (f32, f32),
    room_scale: i32,
    )
    -> na::Matrix4<f32>
{
    let proj = compute_projection(camera_size, room_scale);
    let view = compute_view(position);
    proj.as_matrix() * view.to_homogeneous()
}

impl Camera {
    pub fn new(default_camera_size: (f32, f32)) -> Self {
        let pos = [0.0, 0.0];

        let matrix = projection_view_matrix(
            pos, default_camera_size, 0);

        Self {
            position: pos, 
            room_scale: 0,
            camera_size: default_camera_size,
            matrix: matrix,
        }
    }

    fn update(&mut self) {
        self.matrix = projection_view_matrix(
            self.position, self.camera_size, self.room_scale);
    }

    pub fn set_camera_size(&mut self, camera_size: (f32, f32)) {
        self.camera_size = camera_size;
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
