use crate::oxide::{
    app::Application,
    layer::{Layer, LayerStack, LayerCollection}, event::{Event, EventType},
};

pub struct OxideApp {
    running: bool,
    layers: LayerStack,
}

impl Application for OxideApp {
    fn push_layer(&mut self, layer: Box<dyn Layer>) {
        self.layers.push_layer(layer);
    }

    fn push_overlay(&mut self, layer: Box<dyn Layer>) {
        self.layers.push_overlay(layer);
    }

    fn close(&mut self) {
        self.layers.layers().clear();
    }

    fn notify(&mut self, event: &Event) {
        match event.data {
            EventType::WindowClose => {
                self.running = false;
            }
            _ => {}
        }

        for layer in self.layers.layers() {
            layer.on_event(&event);
        }
    }

    fn new() -> Self {
        OxideApp {
            running: false,
            layers: LayerStack {
                stack: LayerCollection::new(),
            },
        }
    }

    fn is_running(&self) -> bool {
        self.running
    }

    fn layers(&mut self) -> &mut LayerStack {
        &mut self.layers
    }
}
