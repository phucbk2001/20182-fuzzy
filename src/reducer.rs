use crate::context::Context;
use crate::action::{
    Action, 
    CameraAction,
};

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
    for action in actions {
        match *action {
            Action::Camera(action) => camera_reducer(context, action),
        };
    }
}
