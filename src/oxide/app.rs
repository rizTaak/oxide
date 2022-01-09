use super::event::OxideEvent;
use super::layer::{Layer, LayerStack};
use super::window::WindowProps;
extern crate gl;

pub trait Application {
    fn new(props: &WindowProps) -> Self;
    fn is_running(&self) -> bool;
    fn push_layer(&mut self, layer: Box<dyn Layer>);
    fn push_overlay(&mut self, layer: Box<dyn Layer>);
    fn close(&mut self);
    fn notify(&mut self, event: &OxideEvent);
    fn layers(&mut self) -> &mut LayerStack;
}
