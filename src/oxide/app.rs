use super::window::WindowProps;
use crate::external::glad::gl;
use crate::oxide::window::Window;
pub trait Application {
    fn run(&mut self);
}

pub struct OxideApp<T: Window> {
    running: bool,
    window: T,
}

impl<T: Window> OxideApp<T> {
    pub fn new(props: WindowProps) -> OxideApp<T> {
        OxideApp {
            running: true,
            window: T::new(props),
        }
    }
}

impl<T: Window> Application for OxideApp<T> {
    fn run(&mut self) {
        while self.running {
            unsafe {
                gl::ClearColor(1., 0., 1., 1.);
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }
            self.window.on_update();
        }
    }
}
