use crate::config::Config;
use crate::camera::Camera;
use crate::road_renderer::RoadRenderer;

use crate::road::{
    Bezier, Road, BezierId, 
    PointId, Lane, DirectedBezier,
    LocationId,
};

use glium::Display;
use glium::glutin;

#[allow(dead_code)]
pub struct Context<'a> {
    pub display: &'a Display,
    pub config: Config,
    pub camera: Camera,
    pub road: Road,
    pub road_renderer: RoadRenderer,
}

impl<'a> Context<'a> {
    pub fn new(display: &'a Display) -> Self {
        let config = Config::new();
        let camera = Camera::new(
            config.get_camera_size((100, 100))
        );
        let mut road_renderer = RoadRenderer::new(&display);

        let mut road = Road::new();
        road.points = vec![(0.0, -10.0), (10.0, 0.0), (0.0, 10.0), (-10.0, 0.0)];

        let p1 = PointId { id: 0 };
        let p2 = PointId { id: 1 };
        let p3 = PointId { id: 2 };
        let p4 = PointId { id: 3 };

        let b1 = Bezier { point1: p1, point2: p2, middle: (3.0, -3.0) };
        let b2 = Bezier { point1: p4, point2: p3, middle: (-8.0, 8.0) };
        road.beziers = vec![b1, b2];

        let bezier_id1 = BezierId{ id: 0 };
        let bezier_id2 = BezierId{ id: 1 };

        let db1 = DirectedBezier { bezier_id: bezier_id2, is_forward: true };
        let db2 = DirectedBezier { bezier_id: bezier_id1, is_forward: true };

        let location = LocationId { id: 0 };
        road.lanes = vec![Lane { 
            from: location, 
            to: location,
            left: vec![db1],
            right: vec![db2],
        }];

        road_renderer.update_from(display, &road);

        Self {
            display: display,
            config: config,
            camera: camera,
            road: road,
            road_renderer: road_renderer,
        }
    }

    fn handle_mouse_wheel(&mut self, event: glutin::MouseScrollDelta) {
        use glutin::MouseScrollDelta::LineDelta;
        match event {
            LineDelta(_h, v) => {
                self.camera.increase_room_scale(-v as i32);
            },
            _ => (), 
        }
    }

    fn handle_cursor_moved(&mut self, x: f64, y: f64) {
        println!("CursorMoved: {} {}", x, y);
    }

    #[allow(dead_code)]
    pub fn handle_event(&mut self, event: glutin::WindowEvent) {
        use glutin::WindowEvent::{
            MouseWheel,
            CursorMoved,
        };
        match event {
            MouseWheel { delta, .. } => 
                self.handle_mouse_wheel(delta),
            CursorMoved { position, .. } => 
                self.handle_cursor_moved(position.x, position.y),
            _ => (),
        }
    }
}
