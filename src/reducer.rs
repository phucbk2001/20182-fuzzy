use crate::context::Context;
use crate::action::{
    Action, 
    CameraAction,
};
use crate::car::Car;

fn camera_reducer(
    context: &mut Context, 
    action: CameraAction) 
{
    use CameraAction::{
        Zoom, 
        Drag,
    };
    let camera = &mut context.camera;

    match action {
        Zoom(v) => camera.increase_room_scale(v),
        Drag { from, to, completed } => {
            let (x0, y0) = from;
            let (x1, y1) = to;
            let p0 = camera.screen_coords_to_world(x0 as f32, y0 as f32);
            let p1 = camera.screen_coords_to_world(x1 as f32, y1 as f32);
            let v = p0 - p1;
            let pos = camera.get_old_position();

            if completed {
                camera.set_position(pos + v);
            }
            else {
                camera.set_temp_position(pos + v);
            }
        },
    }
}

pub fn reduce(
    context: &mut Context, 
    actions: &Vec<Action>)
{
    use crate::car::AddCar::*;

    for action in actions {
        match *action {
            Action::Camera(action) => camera_reducer(context, action),
            Action::Click(x, y) => {
                let p = context.camera.screen_coords_to_real_position(x as f32, y as f32);
                context.car_system.add_car =
                    match context.car_system.add_car {
                        Nope => Nope,
                        Adding => AddedPoint(p),
                        AddedPoint(prev_pos) => {
                            if let Some(car) = Car::from_positions(
                                &context.road, prev_pos, p)
                            {
                                context.car_system.add(car);
                            }
                            else {
                                println!("Error while chosing points to add a car");
                            }
                            Nope
                        },
                    };
            },
            Action::Esc => {
                context.car_system.add_car = Nope;
            },
            Action::AddCar => {
                context.car_system.add_car = Adding;
            }
        };
    }
}
