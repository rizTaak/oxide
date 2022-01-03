use super::event::{Event, EventObserver, EventType};
use super::layer::{Layer, LayerCollection, LayerStack};
use super::window::WindowProps;
extern crate gl;
use crate::oxide::window::Window;
use std::cell::RefCell;
use std::rc::Rc;
pub trait Application {
    fn run(&mut self);
    fn push_layer(&mut self, layer: Box<dyn Layer>);
    fn push_overlay(&mut self, layer: Box<dyn Layer>);
    fn close(&mut self);
}

pub struct OxideAppObserver {
    running: bool,
    layers: LayerStack,
}

impl EventObserver for OxideAppObserver {
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

    fn can_handle(&self, _: &Event) -> bool {
        true
    }
}
pub struct OxideApp<T: Window<OxideAppObserver>> {
    pub observer: Rc<RefCell<OxideAppObserver>>,
    pub window: T,
}

impl<T: Window<OxideAppObserver>> OxideApp<T> {
    pub fn new(props: WindowProps) -> OxideApp<T> {
        let mut app = OxideApp {
            observer: Rc::new(RefCell::new(OxideAppObserver {
                running: true,
                layers: LayerStack {
                    stack: LayerCollection::new(),
                },
            })),
            window: T::new(props),
        };
        app.window.set_callback(Some(app.observer.clone()));
        app
    }
}

impl<'a, T: Window<OxideAppObserver>> Application for OxideApp<T> {
    fn run(&mut self) {
        while !self.window.should_close() {
            unsafe {
                gl::ClearColor(1., 0., 1., 1.);
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }
            for layer in self.observer.borrow_mut().layers.layers() {
                layer.on_update(self);
            }
            self.window.on_update();
        }
    }

    fn push_layer(&mut self, layer: Box<dyn Layer>) {
        self.observer.borrow_mut().layers.push_layer(layer);
    }

    fn push_overlay(&mut self, layer: Box<dyn Layer>) {
        self.observer.borrow_mut().layers.push_overlay(layer);
    }

    fn close(&mut self) {
        self.observer.borrow_mut().layers.layers().clear();
    }
}
