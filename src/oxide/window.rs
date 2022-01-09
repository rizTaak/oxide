extern crate gl;
use super::app::Application;

extern crate glfw;
#[derive(Debug, Copy, Clone)]
pub struct WindowProps {
    pub title: &'static str,
    pub width: i32,
    pub height: i32,
}

impl WindowProps {
    pub fn new(title: &'static str, width: i32, height: i32) -> WindowProps {
        WindowProps {
            title,
            width,
            height,
        }
    }
}

pub trait Window<A: Application> {
    fn new(props: &WindowProps) -> Self;
    fn on_update(&mut self, app: &mut A, props: &mut WindowProps);
    fn set_vsync(&mut self, enabled: bool);
    fn is_vsync(&self) -> bool;
    fn should_close(&self) -> bool;
}
