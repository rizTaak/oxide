use glfw::{Context, Glfw, SwapInterval, WindowEvent};
use std::sync::mpsc::Receiver;

use crate::external::glad::gl;
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

pub trait Window {
    // using EventCallbackFn = std::function<void(Event&)>;
    // virtual void SetEventCallback(const EventCallbackFn& callback) = 0;
    fn new(props: WindowProps) -> Self;

    fn on_update(&mut self);
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn set_vsync(&mut self, enabled: bool);
    fn is_vsync(&self) -> bool;
}

pub struct GenericWindow {
    glfw: Glfw,
    window: glfw::Window,
    events: Receiver<(f64, WindowEvent)>,
}

impl Window for GenericWindow {
    fn new(props: WindowProps) -> Self {
        let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
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

        gl::load(|e| glfw.get_proc_address_raw(e) as *const std::os::raw::c_void);

        let mut window = GenericWindow {
            glfw,
            window,
            events,
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
}
