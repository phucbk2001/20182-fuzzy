mod quad_eq;
mod bezier;
mod config;
mod glhelper;
mod camera;
mod road;
mod context;
mod car;
mod window;
mod action;
mod reducer;
mod ecs;

use std::time::Instant;

#[allow(dead_code)]
fn print_fps(prev: Instant, current: Instant) {
    let d = current.duration_since(prev);
    println!("FPS: {}", 1_000_000_000 / d.subsec_nanos() as u64);
}

fn main() {
    use glium;
    use glium::glutin;
    use glium::Surface;

    let mut events_loop = glutin::EventsLoop::new();
    let window = 
        glutin::WindowBuilder::new()
        .with_title("Fuzzy System")
        // .with_decorations(false)
        .with_fullscreen(None);
    let glutin_context = 
        glutin::ContextBuilder::new()
        .with_gl_debug_flag(false)
        .with_vsync(true);

    let display = glium::Display::new(
        window, glutin_context, &events_loop).unwrap();

    let mut context = context::Context::new(&display);

    #[allow(unused_variables, unused_mut)]
    let mut prev_instant = Instant::now();

    let mut closed: bool = false;
    while !closed {
        context.update(&display);
        context.finish();

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        context.render(&mut target);
        target.finish().unwrap();

        let mut actions: Vec<action::Action> = Vec::new();
        events_loop.poll_events(|e| {
            actions.clear();
            match e {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => closed = true,
                    _ => window::handle_event(
                        &mut context, event, &mut actions),
                },
                _ => (),
            }
            reducer::reduce(&mut context, &actions);
        });
    }
}
