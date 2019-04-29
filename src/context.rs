use crate::config::Config;
use crate::window::{WindowSystem, DragEvent};
use crate::camera::Camera;

use crate::road::Road;
use crate::road::renderer::RoadRenderer;

use crate::car::CarSystem;
use crate::car::renderer::CarRenderer;

use crate::action::{Action, CameraAction};

use crate::init;

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

        let (road, car_system) = init::init(&config);

        let road_renderer = RoadRenderer::from(
            &display, &road, &config);

        let car_renderer = CarRenderer::new(&display, &config);

        Self {
            display,
            window_system,
            config,
            camera,
            road,
            road_renderer,
            car_system,
            car_renderer,
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
