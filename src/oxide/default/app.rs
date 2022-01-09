use std::collections::VecDeque;

use crate::oxide::{
    app::Application,
    event::{EventType, OxideEvent},
    layer::{Layer, LayerStack},
    window::WindowProps,
};

pub struct OxideApp {
    running: bool,
    props: WindowProps,
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

    fn notify(&mut self, event: &OxideEvent) {
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

    fn new(props: &WindowProps) -> Self {
        OxideApp {
            running: false,
            props: *props, // should be copy trait
            layers: LayerStack {
                stack: VecDeque::<Box<dyn Layer>>::new(),
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
