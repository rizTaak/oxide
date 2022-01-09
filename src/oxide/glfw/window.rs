use glfw::{Action, Context, Glfw, SwapInterval, WindowEvent};
extern crate gl;
use crate::oxide::app::Application;
use crate::oxide::event::{EventDispatcher, OxideEvent};
use crate::oxide::window::{Window, WindowProps};
use crate::oxide_error;
use std::cell::Cell;
use std::marker::PhantomData;
use std::sync::mpsc::Receiver;

pub struct GlWindow<A: Application> {
    glfw: Glfw,
    window: glfw::Window,
    events: Receiver<(f64, WindowEvent)>,
    phantom: PhantomData<A>,
}

fn error_callback(_: glfw::Error, description: String, error_count: &Cell<usize>) {
    oxide_error!("GLFW error {}: {}", error_count.get(), description);
    error_count.set(error_count.get() + 1);
}

impl<A: Application> Window<A> for GlWindow<A> {
    fn new(props: WindowProps) -> Self {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));

        let (mut window, events) = glfw
            .create_window(
                props.width as u32,
                props.height as u32,
                props.title,
                glfw::WindowMode::Windowed,
            )
            .expect("Failed to create GLFW window.");
        window.make_current();
        window.set_all_polling(true);

        glfw.set_error_callback(Some(glfw::Callback {
            f: error_callback,
            data: Cell::new(0),
        }));

        gl::load_with(|s| window.get_proc_address(s) as *const _);

        let window = GlWindow {
            glfw,
            window,
            events,
            phantom: PhantomData::<A>,
        };

        //window.set_vsync(true);
        window
    }

    fn on_update(&mut self, app: &mut A) {
        self.window.swap_buffers();
        self.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&self.events) {
            self.handle_window_event(app, &event);
        }
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

    fn should_close(&self) -> bool {
        self.window.should_close()
    }
}

impl<A: Application> GlWindow<A> {
    fn handle_window_event(&self, app: &mut A, event: &WindowEvent) {
        match event {
            WindowEvent::Close => {
                let evt = OxideEvent::close();
                let dispatcher = EventDispatcher::new(&evt);
                dispatcher.dispatch(app);
            }
            WindowEvent::CursorPos(x, y) => {
                let evt = OxideEvent::mouse_move(*x, *y);
                let dispatcher = EventDispatcher::new(&evt);
                dispatcher.dispatch(app);
            }
            WindowEvent::MouseButton(b, a, _) => match a {
                Action::Repeat | Action::Press => {
                    let evt = OxideEvent::mouse_button_pressed(*b as i32);
                    let dispatcher = EventDispatcher::new(&evt);
                    dispatcher.dispatch(app);
                }
                Action::Release => {
                    let evt = OxideEvent::mouse_button_released(*b as i32);
                    let dispatcher = EventDispatcher::new(&evt);
                    dispatcher.dispatch(app);
                }
            },
            WindowEvent::Scroll(x, y) => {
                let evt = OxideEvent::mouse_scrolled(*x, *y);
                let dispatcher = EventDispatcher::new(&evt);
                dispatcher.dispatch(app);
            }
            WindowEvent::Char(c) => {
                let evt = OxideEvent::key_typed(*c as i32);
                let dispatcher = EventDispatcher::new(&evt);
                dispatcher.dispatch(app);
            }
            WindowEvent::Key(key, _, a, _) => match a {
                Action::Repeat | Action::Press => {
                    let evt = OxideEvent::key_pressed(*key as i32);
                    let dispatcher = EventDispatcher::new(&evt);
                    dispatcher.dispatch(app);
                }
                Action::Release => {
                    let evt = OxideEvent::key_release(*key as i32);
                    let dispatcher = EventDispatcher::new(&evt);
                    dispatcher.dispatch(app);
                }
            },
            WindowEvent::Size(width, height) => {
                // let size = self.window.get_size();
                // println!("{:?}", size);
                // let evt = OxideEvent::size(&size.0, &size.1);
                app.set_width(*width);
                app.set_height(*height);
                let evt = OxideEvent::size(width, height);
                let dispatcher = EventDispatcher::new(&evt);
                dispatcher.dispatch(app);
            }   
            _ => {}
        }
    }
}
