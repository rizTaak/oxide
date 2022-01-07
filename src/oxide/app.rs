use super::event::Event;
use super::layer::{Layer, LayerStack};
extern crate gl;

pub trait Application {
    fn new() -> Self;
    fn is_running(&self) -> bool;
    fn push_layer(&mut self, layer: Box<dyn Layer>);
    fn push_overlay(&mut self, layer: Box<dyn Layer>);
    fn close(&mut self);
    fn notify(&mut self, event: &Event);
    fn layers(&mut self) -> &mut LayerStack;
}
