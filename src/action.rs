#[derive(Copy, Clone)]
pub enum Action {
    Camera(CameraAction),
    Click(f64, f64),
}

#[derive(Copy, Clone)]
pub enum CameraAction {
    Zoom(i32),
    Drag {
        from: (f64, f64),
        to: (f64, f64),
        completed: bool,
    },
}
