use super::event::{Event, EventObserver, EventType};
use super::layer::{LayerCollection, LayerStack};
use super::window::WindowProps;
use crate::external::glad::gl;
use crate::oxide::window::Window;
use std::cell::RefCell;
use std::rc::Rc;
pub trait Application {
    fn run(&mut self);
}

pub struct OxideAppObserver {
    running: bool,
}

impl EventObserver for OxideAppObserver {
    fn notify(&mut self, event: &Event) {
        match event.data {
            EventType::WindowClose => {
                self.running = false;
            }
            _ => {}
        }
    }

    fn can_handle(&self, event: &Event) -> bool {
        match event.data {
            EventType::WindowClose => true,
            _ => false,
        }
    }
}
pub struct OxideApp<T: Window<OxideAppObserver>> {
    pub observer: Rc<RefCell<OxideAppObserver>>,
    pub window: T,
    pub layers: LayerStack,
}

impl<T: Window<OxideAppObserver>> OxideApp<T> {
    pub fn new(props: WindowProps) -> OxideApp<T> {
        let mut app = OxideApp {
            observer: Rc::new(RefCell::new(OxideAppObserver { running: true })),
            window: T::new(props),
            layers: LayerStack {
                stack: LayerCollection::new(),
            },
        };
        app.window.set_callback(Some(app.observer.clone()));
        app
    }
}

impl<T: Window<OxideAppObserver>> Application for OxideApp<T> {
    fn run(&mut self) {
        while self.observer.borrow().running {
            unsafe {
                gl::ClearColor(1., 0., 1., 1.);
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }
            self.window.on_update();
        }
    }
}
