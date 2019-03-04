mod quad_eq;

mod config;
mod glhelper;
mod road_renderer;
mod camera;
mod road;
mod bezier;

fn main() {
    use glium;
    use glium::glutin;
    use glium::Surface;
    use road::{
        Bezier, Road, BezierId, 
        PointId, Lane, DirectedBezier,
        LocationId,
    };

    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let config = config::Config::new();
    let mut camera = camera::Camera::new(
        config.get_camera_size((100, 100))
    );
    let mut road_renderer = road_renderer::RoadRenderer::new(&display);

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

    road_renderer.update_from(&display, &road);

    let mut closed: bool = false;
    while !closed {
        let mut target = display.draw();
        target.clear_color(1.0, 1.0, 1.0, 1.0);

        let dims = target.get_dimensions();
        camera.set_camera_size(config.get_camera_size(dims));

        road_renderer.render(&mut target, camera.get_matrix());

        target.finish().unwrap();

        events_loop.poll_events(|e| {
            match e {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => closed = true,
                    _ => (),
                },
                _ => (),
            }
        });
    }
}
