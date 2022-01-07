extern crate gl;
use super::app::Application;

extern crate glfw;

pub struct WindowProps {
    pub title: &'static str,
    pub width: u32,
    pub height: u32,
}

impl WindowProps {
    pub fn new(title: &'static str, width: u32, height: u32) -> WindowProps {
        WindowProps {
            title,
            width,
            height,
        }
    }
}

pub trait Window<A: Application> {
    fn new(props: WindowProps) -> Self;
    fn on_update(&mut self, app: &mut A);
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn set_vsync(&mut self, enabled: bool);
    fn is_vsync(&self) -> bool;
    fn should_close(&self) -> bool;
}
