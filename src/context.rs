use crate::config::Config;
use crate::camera::Camera;
use crate::road_renderer::RoadRenderer;

use crate::road::{Road, Backbone};

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

        let mut backbone = Backbone::new();

        let location_a = backbone.add_location("A");
        let location_b = backbone.add_location("B");

        let p1 = backbone.add_point((40.0, -20.0), (-3.0, 0.0));
        let p2 = backbone.add_point((10.0, -10.0), (-2.0, 1.0));
        let p3 = backbone.add_point((-10.0, 10.0), (-1.0, 2.0));
        let p4 = backbone.add_point((-13.0, 30.0), (0.0, 5.0));

        backbone.add_road(location_a, location_b, &[p1, p2, p3, p4]);

        let road = Road::from(&backbone, &config);

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

    fn handle_cursor_moved(&mut self, _x: f64, _y: f64) {
        // println!("CursorMoved: {} {}", x, y);
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
