use glium::glutin;
use crate::context::Context;
use crate::action::Action;
use crate::ecs;

#[allow(dead_code)]
const MAX_WINDOW_COUNT: usize = 128;

const CLICK_RANGE: f64 = 10.0;

#[derive(Copy, Clone)]
pub struct ForWindow {}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct Window {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
}

impl Default for Window {
    fn default() -> Window {
        Window {
            x: 0.0, 
            y: 0.0, 
            w: 0.0, 
            h: 0.0, 
        }
    }
}

type WindowId = ecs::Entity<ForWindow>;

#[derive(Copy, Clone)]
pub enum MouseButton {
    Left, 
    Right, 
    Middle,
}

#[derive(Copy, Clone)]
pub struct ClickEvent {
    pub window: WindowId,
    pub button: MouseButton,
    pub position: (f64, f64),
}

#[derive(Copy, Clone)]
pub struct DragEvent {
    pub window: WindowId,
    pub from: (f64, f64),
    pub to: (f64, f64), 
    pub completed: bool,
}

type OnClick = Box<Fn(ClickEvent, &mut Vec<Action>)>;

type OnDrag = Box<Fn(DragEvent, &mut Vec<Action>)>;

type OnScroll = Box<Fn(f32, &mut Vec<Action>)>;

#[derive(Copy, Clone)]
enum LeftState {
    Nothing,
    Pressing((f64, f64)),
    Dragging((f64, f64)),
}

pub struct MouseManager {
    on_scroll: OnScroll,

    left_state: LeftState,
    right_state: Option<(f64, f64)>,
    middle_state: Option<(f64, f64)>,

    pos: (f64, f64),
}

pub struct WindowSystem {
    em: ecs::EntityManager<ForWindow>,
    #[allow(dead_code)]
    windows: ecs::Components<Window, ForWindow>,
    pub root_window: WindowId,

    mouse: MouseManager,
    on_clicks: ecs::Components<Option<OnClick>, ForWindow>,
    on_drags: ecs::Components<Option<OnDrag>, ForWindow>,
}

fn default_on_scroll(_: f32, _: &mut Vec<Action>) {}

impl WindowSystem {
    pub fn new() -> Self {
        let mouse = MouseManager::new();
        let em = ecs::EntityManager::<ForWindow>::new();

        let mut this = Self {
            em: em,
            root_window: WindowId::new(0, 0),
            windows: ecs::Components::<Window, ForWindow>::new(),
            mouse: mouse,
            on_clicks: ecs::Components::<Option<OnClick>, ForWindow>::new(),
            on_drags: ecs::Components::<Option<OnDrag>, ForWindow>::new(),
        };
        this.root_window = this.new_window();
        this
    }

    pub fn new_window(&mut self) -> WindowId {
        let window = self.em.allocate();
        self.on_clicks.set(window, None);
        self.on_drags.set(window, None);
        window
    }

    pub fn set_on_scroll(&mut self, on_scroll: OnScroll) {
        self.mouse.on_scroll = on_scroll;
    }

    pub fn set_on_drag(&mut self, window: WindowId, on_drag: OnDrag) {
        self.on_drags.set(window, Some(on_drag));
    }

    fn get_drag_callback(&self, window: WindowId) -> Option<&OnDrag> {
        if self.em.is_alive(window) {
            if let Some(func) = self.on_drags.get(window) {
                Some(&func)
            }
            else {
                None
            }
        }
        else {
            None
        }
    }

    fn get_click_callback(&self, window: WindowId) -> Option<&OnClick> {
        if self.em.is_alive(window) {
            if let Some(func) = self.on_clicks.get(window) {
                Some(&func)
            }
            else {
                None
            }
        }
        else {
            None
        }
    }
}

fn near(old_pos: (f64, f64), new_pos: (f64, f64)) -> bool {
    let (x0, y0) = old_pos;
    let (x1, y1) = new_pos;
    let dx = f64::abs(x1 - x0);
    let dy = f64::abs(y1 - y0);
    dx <= CLICK_RANGE && dy <= CLICK_RANGE
}

impl MouseManager {
    pub fn new() -> Self {
        Self {
            on_scroll: Box::new(default_on_scroll),
            left_state: LeftState::Nothing,
            right_state: None,
            middle_state: None,
            pos: (0.0, 0.0),
        }
    }
}

impl WindowSystem {
    fn handle_mouse_wheel(
        &mut self,
        event: glutin::MouseScrollDelta,
        actions: &mut Vec<Action>) 
    {
        use glutin::MouseScrollDelta::LineDelta;
        let on_scroll = &self.mouse.on_scroll;
        match event {
            LineDelta(_h, v) => {
                on_scroll(v, actions);
            },
            _ => (), 
        }
    }

    fn handle_cursor_moved(
        &mut self,
        x: f64, y: f64,
        actions: &mut Vec<Action>) 
    {
        self.mouse.pos = (x, y);
        use LeftState::{
            Nothing,
            Pressing,
            Dragging,
        };

        let new_pos = self.mouse.pos;
        let window = self.root_window;

        let mut call_drag = |old_pos| {
            if let Some(callback) = self.get_drag_callback(window) {
                let event = DragEvent {
                    window: window,
                    from: old_pos,
                    to: new_pos,
                    completed: false,
                };
                callback(event, actions);
            }
        };

        self.mouse.left_state = match self.mouse.left_state {
            Nothing => Nothing,
            Pressing(old_pos) => {
                if near(old_pos, new_pos) {
                    call_drag(old_pos);
                    Pressing(old_pos)
                }
                else {
                    Dragging(old_pos)
                }
            },
            Dragging(old_pos) => {
                call_drag(old_pos);
                Dragging(old_pos)
            },
        };

        self.mouse.right_state = match self.mouse.right_state {
            None => None,
            Some(old_pos) => {
                if near(old_pos, new_pos) {
                    Some(old_pos)
                }
                else {
                    None
                }
            }
        };

        self.mouse.middle_state = match self.mouse.middle_state {
            None => None,
            Some(old_pos) => {
                if near(old_pos, new_pos) {
                    Some(old_pos)
                }
                else {
                    None
                }
            }
        };
    }

    fn click(
        &self, window: WindowId, 
        button: MouseButton,
        actions: &mut Vec<Action>) 
    {
        if let Some(callback) = self.get_click_callback(window) {
            let event = ClickEvent {
                window: window,
                button: button, 
                position: self.mouse.pos,
            };
            callback(event, actions);
        }
    }

    fn handle_left_input(
        &mut self,
        state: glutin::ElementState,
        actions: &mut Vec<Action>)
    {
        use glutin::ElementState::{
            Pressed,
            Released,
        };

        use LeftState::{
            Nothing,
            Dragging,
            Pressing,
        };

        let window = self.root_window;
        
        self.mouse.left_state = match state {
            Pressed => Pressing(self.mouse.pos),
            Released => match self.mouse.left_state {
                Nothing => Nothing,
                Pressing(_) => {
                    self.click(window, MouseButton::Left, actions);
                    Nothing
                },
                Dragging(old_pos) => {
                    if let Some(callback) = self.get_drag_callback(window) {
                        let event = DragEvent {
                            window: window, 
                            from: old_pos,
                            to: self.mouse.pos, 
                            completed: true,
                        };
                        callback(event, actions);
                    }
                    Nothing
                },
            },
        };
    }

    fn handle_right_input( 
        &mut self,
        state: glutin::ElementState,
        actions: &mut Vec<Action>)
    {
        use glutin::ElementState::{
            Pressed,
            Released,
        };

        self.mouse.right_state = match state {
            Pressed => Some(self.mouse.pos),
            Released => match self.mouse.right_state {
                None => None,
                Some(_) => {
                    self.click(self.root_window, MouseButton::Right, actions);
                    None
                },
            },
        };
    }

    fn handle_middle_input( 
        &mut self,
        state: glutin::ElementState,
        actions: &mut Vec<Action>)
    {
        use glutin::ElementState::{
            Pressed,
            Released,
        };

        self.mouse.middle_state = match state {
            Pressed => Some(self.mouse.pos),
            Released => match self.mouse.middle_state {
                None => None,
                Some(_) => {
                    self.click(self.root_window, MouseButton::Middle, actions);
                    None
                },
            },
        };
    }

    pub fn handle_mouse_input(
        &mut self,
        state: glutin::ElementState,
        button: glutin::MouseButton,
        actions: &mut Vec<Action>)
    {
        use glutin::MouseButton::{
            Left,
            Right,
            Middle
        };

        match button {
            Left => self.handle_left_input(state, actions),
            Right => self.handle_right_input(state, actions),
            Middle => self.handle_middle_input(state, actions),
            _ => (),
        }
    }

}

pub fn handle_event(
    context: &mut Context, 
    event: glutin::WindowEvent, 
    actions: &mut Vec<Action>) 
{
    use glutin::WindowEvent::{
        MouseWheel,
        CursorMoved,
        MouseInput,
    };

    let window = &mut context.window_system;

    match event {
        MouseWheel { delta, .. } => 
            window.handle_mouse_wheel(delta, actions),
        CursorMoved { position, .. } => 
            window.handle_cursor_moved(
                position.x, position.y, actions),
        MouseInput { state, button, .. } =>
            window.handle_mouse_input(state, button, actions),
        _ => (),
    }
}
