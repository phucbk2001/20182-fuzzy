mod quad_eq;
mod bezier;
mod config;
mod glhelper;
mod road_renderer;
mod camera;
mod road;
mod context;
mod car;
mod car_renderer;
mod window;
mod action;
mod reducer;

use std::time::Instant;

#[allow(dead_code)]
fn print_fps(prev: &mut Instant) {
    let current = Instant::now();
    let d = current.duration_since(*prev);
    println!("FPS: {}", 1_000_000_000 / d.subsec_nanos() as u64);
    *prev = current;
}

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

    #[allow(unused_variables, unused_mut)]
    let mut prev_instant = Instant::now();
    let mut closed: bool = false;
    while !closed {
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

        context.update(&display);

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        context.render(&mut target);

        target.finish().unwrap();

        context.finish();

        // print_fps(&mut prev_instant);
    }
}
