use crate::external::glad::gl;
use glfw::{Context, Glfw, SwapInterval, WindowEvent};

use super::event::EventObserver;
use crate::oxide_error;
use std::{cell::Cell, rc::Rc, sync::mpsc::Receiver};
extern crate glfw;

pub struct WindowProps {
    title: &'static str,
    width: u32,
    height: u32,
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

pub trait Window<T: EventObserver> {
    // using EventCallbackFn = std::function<void(Event&)>;
    // virtual void SetEventCallback(const EventCallbackFn& callback) = 0;
    fn new(props: WindowProps) -> Self;
    fn set_callback(&mut self, observer: Option<Rc<T>>);
    fn on_update(&mut self);
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn set_vsync(&mut self, enabled: bool);
    fn is_vsync(&self) -> bool;
}

pub struct GenericWindow<T: EventObserver> {
    glfw: Glfw,
    window: glfw::Window,
    events: Receiver<(f64, WindowEvent)>,
    callback: Option<Rc<T>>,
}

fn error_callback(_: glfw::Error, description: String, error_count: &Cell<usize>) {
    oxide_error!("GLFW error {}: {}", error_count.get(), description);
    error_count.set(error_count.get() + 1);
}

impl<'a, T: EventObserver> Window<T> for GenericWindow<T> {
    fn new(props: WindowProps) -> Self {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        let (mut window, events) = glfw
            .create_window(
                props.width,
                props.height,
                props.title,
                glfw::WindowMode::Windowed,
            )
            .expect("Failed to create GLFW window.");
        window.set_key_polling(true);
        window.make_current();

        glfw.set_error_callback(Some(glfw::Callback {
            f: error_callback,
            data: Cell::new(0),
        }));

        gl::load(|e| glfw.get_proc_address_raw(e) as *const std::os::raw::c_void);

        let mut window = GenericWindow {
            glfw,
            window,
            events,
            callback: None,
        };

        window.set_vsync(true);
        window
    }

    fn on_update(&mut self) {
        self.glfw.poll_events();
        self.window.swap_buffers();
    }

    fn width(&self) -> u32 {
        0
    }

    fn height(&self) -> u32 {
        0
    }

    fn set_vsync(&mut self, enabled: bool) {
        if enabled {
            self.glfw.set_swap_interval(SwapInterval::Sync(1));
        } else {
            self.glfw.set_swap_interval(SwapInterval::None);
        }
    }

    fn is_vsync(&self) -> bool {
        true
    }

    fn set_callback(&mut self, observer: Option<Rc<T>>) {
        self.callback = observer;
    }
}
