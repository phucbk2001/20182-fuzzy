mod quad_eq;
mod bezier;
mod config;
mod glhelper;
mod road_renderer;
mod camera;
mod road;
mod context;

fn main() {
    use glium;
    use glium::glutin;
    use glium::Surface;

    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new();
    let glutin_context = glutin::ContextBuilder::new();
    let display = glium::Display::new(
        window, glutin_context, &events_loop).unwrap();
    
    let mut context = context::Context::new(&display);

    let mut closed: bool = false;
    while !closed {
        let mut target = display.draw();
        target.clear_color(1.0, 1.0, 1.0, 1.0);

        let dims = target.get_dimensions();
        context.camera.set_camera_size(
            context.config.get_camera_size(dims));

        context.road_renderer.render(
            &mut target, context.camera.get_matrix());

        target.finish().unwrap();

        events_loop.poll_events(|e| {
            match e {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => closed = true,
                    _ => context.handle_event(event),
                },
                _ => (),
            }
        });
    }
}
