use std::collections::VecDeque;

use super::{app::Application, event::Event};

pub trait Layer {
    fn on_attach(&mut self);
    fn on_detach(&mut self);
    fn on_update(&mut self, app: &dyn Application);
    fn on_event(&mut self, event: &Event) -> bool;
    fn name(&self) -> &str;
}

pub type LayerCollection = VecDeque<Box<dyn Layer>>;

pub struct LayerStack {
    pub(crate) stack: LayerCollection,
}

impl LayerStack {
    pub fn push_layer(&mut self, layer: Box<dyn Layer>) {
        let mut layer = layer;
        layer.on_attach();
        self.stack.push_front(layer);
    }

    pub fn pop_layer(&mut self) -> Box<dyn Layer> {
        self.stack.pop_front().unwrap()
    }

    pub fn push_overlay(&mut self, layer: Box<dyn Layer>) {
        let mut layer = layer;
        layer.on_attach();
        self.stack.push_back(layer);
    }

    pub fn pop_overlay(&mut self) -> Box<dyn Layer> {
        self.stack.pop_back().unwrap()
    }

    pub fn layers(&mut self) -> &mut LayerCollection {
        &mut self.stack
    }
}
