use std::marker::PhantomData;

use super::event::{Event, EventObserver};
use super::window::WindowProps;
use crate::external::glad::gl;
use crate::oxide::window::Window;
pub trait Application {
    fn run(&mut self);
}

pub struct OxideAppObserver {
    running: bool,
}

impl EventObserver for OxideAppObserver {
    fn notify(&self, event: &Event) {}

    fn can_handle(&self, event: &Event) -> bool {
        false
    }
}
pub struct OxideApp<'a, T: Window<'a, OxideAppObserver>> {
    pub observer: OxideAppObserver,
    pub window: T,
    phantom: PhantomData<&'a T>,
}

impl<'a, T: Window<'a, OxideAppObserver>> OxideApp<'a, T> {
    pub fn new(props: WindowProps) -> OxideApp<'a, T> {
        let app = OxideApp {
            observer: OxideAppObserver { running: true },
            window: T::new(props),
            phantom: PhantomData,
        };
        app
    }
}

impl<'a, T: Window<'a, OxideAppObserver>> Application for OxideApp<'a, T> {
    fn run(&mut self) {
        while self.observer.running {
            unsafe {
                gl::ClearColor(1., 0., 1., 1.);
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }
            self.window.on_update();
        }
    }
}
