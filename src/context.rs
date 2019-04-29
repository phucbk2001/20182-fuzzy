use crate::config::Config;
use crate::window::{WindowSystem, DragEvent};
use crate::camera::Camera;

use crate::road::{Road, Backbone};
use crate::road::renderer::RoadRenderer;

use crate::car::{CarSystem, Car};
use crate::car::renderer::CarRenderer;

use crate::action::{Action, CameraAction};

use glium::Display;
use glium::Surface;

#[allow(dead_code)]
pub struct Context<'a> {
    pub display: &'a Display,
    pub config: Config,
    pub window_system: WindowSystem,
    pub camera: Camera,
    pub road: Road,
    pub road_renderer: RoadRenderer,
    pub car_system: CarSystem,
    pub car_renderer: CarRenderer,
}

fn on_scroll(v: f32, actions: &mut Vec<Action>) {
    actions.push(Action::Camera(CameraAction::Zoom(-v as i32)));
}

fn camera_on_drag(event: DragEvent, actions: &mut Vec<Action>) {
    let action = CameraAction::Drag {
        from: event.from,
        to: event.to,
        completed: event.completed,
    };
    actions.push(Action::Camera(action));
}

impl<'a> Context<'a> {
    pub fn new(display: &'a Display) -> Self {
        let config = Config::new();
        let mut window_system = WindowSystem::new();
        let camera = Camera::new(
            (config.camera_width, config.camera_width)
        );
        window_system.set_on_scroll(Box::new(on_scroll));
        let window = window_system.root_window;
        window_system.set_on_drag(window, Box::new(camera_on_drag));

        let mut backbone = Backbone::new();

        let location_a = backbone.add_location("A", &config);
        let location_b = backbone.add_location("B", &config);
        let location_c = backbone.add_location("C", &config);
        let location_d = backbone.add_location("D", &config);

        let p1 = backbone.add_point((-20.0, -40.0), (0.0, 3.0));
        let p2 = backbone.add_point((-10.0, -10.0), (1.0, 2.0));
        let p3 = backbone.add_point((0.0, 0.0), (2.0, 1.0));
        let p4 = backbone.add_point((30.0, 13.0), (1.0, 0.0));
        let p5 = backbone.add_point((13.0, 30.0), (0.0, 1.0));
        let p6 = backbone.add_point((70.0, 13.0), (1.0, 0.0));
        let p7 = backbone.add_point((13.0, 30.0), (0.0, -1.0));
        let p8 = backbone.add_point((7.0, 60.0), (-0.5, 1.0));

        backbone.add_road(location_a, location_b, &[p1, p2, p3]);
        backbone.add_road(location_b, location_c, &[p4, p6]);
        backbone.add_road(location_b, location_d, &[p5, p8]);

        backbone.add_cross_section(
            location_a, location_b, location_c,
            &[p3, p4]);

        backbone.add_cross_section(
            location_a, location_b, location_d,
            &[p3, p5]);

        backbone.add_cross_section(
            location_d, location_b, location_c,
            &[p7, p4]);

        let mut road = Road::from(&backbone, &config);

        let road_renderer = RoadRenderer::from(
            &display, &road, &config);

        road.chosen_path = 
            vec![location_a, location_b, location_c];

        let mut car_system = CarSystem::new();
        let car = Car::from_path(&road, &[location_a, location_b, location_c]);

        car_system.add(car);

        let car_renderer = CarRenderer::new(&display, &config);

        Self {
            display: display,
            window_system: window_system,
            config: config,
            camera: camera,
            road: road,
            road_renderer: road_renderer,
            car_system: car_system,
            car_renderer: car_renderer,
        }
    }

    pub fn update(&mut self, display: &Display) {
        self.road.update_street_lights(&self.config);
        self.car_system.update(&self.config);
        self.road_renderer.update(display, &self.road);
    }

    pub fn finish(&mut self) {
        self.road.finish();
    }

    pub fn render<T>(&mut self, target: &mut T) 
        where T: Surface
    {
        let dims = target.get_dimensions();
        self.camera.set_dimensions(dims, &self.config);

        self.road_renderer.render(
            target, &self.road, self.camera.get_matrix());

        self.car_renderer.render(
            target, &self.car_system, self.camera.get_matrix());
    }
}
