use std::{
    ffi::{c_void, CStr},
    time::Instant,
};

use glfw::ffi::GLFWwindow;
use imgui::{BackendFlags, Context, Key};

use super::{app::Application, layer::Layer};
use crate::{external::gl_renderer::renderer::Renderer, oxide::event::EventType, oxide_info};
pub struct ImGuiLayer {
    imgui: imgui::Context,
    renderer: Renderer,
    last_frame: Instant,
    mouse_buttons: [bool; 5],
}

struct GlfwClipboardBackend(*mut c_void);

impl imgui::ClipboardBackend for GlfwClipboardBackend {
    fn get(&mut self) -> Option<imgui::ImString> {
        let char_ptr = unsafe { glfw::ffi::glfwGetClipboardString(self.0 as *mut GLFWwindow) };
        let c_str = unsafe { CStr::from_ptr(char_ptr) };
        Some(imgui::ImString::new(c_str.to_str().unwrap()))
    }

    fn set(&mut self, value: &imgui::ImStr) {
        unsafe {
            glfw::ffi::glfwSetClipboardString(self.0 as *mut GLFWwindow, value.as_ptr());
        };
    }
}

impl ImGuiLayer {
    pub fn new() -> ImGuiLayer {
        let mut imgui = Context::create();

        unsafe {
            let window_ptr = glfw::ffi::glfwGetCurrentContext() as *mut c_void;
            imgui.set_clipboard_backend(Box::new(GlfwClipboardBackend(window_ptr)));
        }

        let io = imgui.io_mut();
        io.backend_flags |= BackendFlags::HAS_MOUSE_CURSORS;
        io.backend_flags |= BackendFlags::HAS_SET_MOUSE_POS;

        io.key_map[Key::Tab as usize] = glfw::Key::Tab as u32;
        io.key_map[Key::LeftArrow as usize] = glfw::Key::Left as u32;
        io.key_map[Key::RightArrow as usize] = glfw::Key::Right as u32;
        io.key_map[Key::UpArrow as usize] = glfw::Key::Up as u32;
        io.key_map[Key::DownArrow as usize] = glfw::Key::Down as u32;
        io.key_map[Key::PageUp as usize] = glfw::Key::PageUp as u32;
        io.key_map[Key::PageDown as usize] = glfw::Key::PageDown as u32;
        io.key_map[Key::Home as usize] = glfw::Key::Home as u32;
        io.key_map[Key::End as usize] = glfw::Key::End as u32;
        io.key_map[Key::Insert as usize] = glfw::Key::Insert as u32;
        io.key_map[Key::Delete as usize] = glfw::Key::Delete as u32;
        io.key_map[Key::Backspace as usize] = glfw::Key::Backspace as u32;
        io.key_map[Key::Space as usize] = glfw::Key::Space as u32;
        io.key_map[Key::Enter as usize] = glfw::Key::Enter as u32;
        io.key_map[Key::Escape as usize] = glfw::Key::Escape as u32;
        io.key_map[Key::A as usize] = glfw::Key::A as u32;
        io.key_map[Key::C as usize] = glfw::Key::C as u32;
        io.key_map[Key::V as usize] = glfw::Key::V as u32;
        io.key_map[Key::X as usize] = glfw::Key::X as u32;
        io.key_map[Key::Y as usize] = glfw::Key::Y as u32;
        io.key_map[Key::Z as usize] = glfw::Key::Z as u32;
        let renderer = Renderer::new(&mut imgui);

        /*
        unsafe {
            let window_ptr = glfw::ffi::glfwGetCurrentContext() as *mut c_void;
            imgui.set_clipboard_backend(Box::new(GlfwClipboardBackend(window_ptr)));
        }*/

        ImGuiLayer {
            imgui,
            last_frame: Instant::now(),
            renderer,
            mouse_buttons: [false, false, false, false, false],
        }
    }
}

impl Layer for ImGuiLayer {
    fn on_attach(&mut self) {
        oxide_info!("{}: on_attach", self.name());
    }

    fn on_detach(&mut self) {
        oxide_info!("{}: on_detach", self.name());
    }

    fn on_update(&mut self, _: &dyn Application) {
        let io = self.imgui.io_mut();

        let now = Instant::now();
        let delta = now - self.last_frame;
        let delta_s = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1_000_000_000.0;
        self.last_frame = now;
        io.delta_time = delta_s;

        let window_size = (1536, 864);
        io.display_size = [window_size.0 as f32, window_size.1 as f32];

        let ui = self.imgui.frame();
        let mut opened = true;
        ui.show_demo_window(&mut opened);
        self.renderer.render(ui);
    }

    fn on_event(&mut self, event: &super::event::Event) -> bool {
        oxide_info!("{}: {:?}", self.name(), event);
        match event.data {
            EventType::MouseMoved { x_mouse, y_mouse } => {
                let io = self.imgui.io_mut();
                io.mouse_pos = [x_mouse as f32, y_mouse as f32]
            }
            EventType::MouseButtonPressed { button } => {
                let io = self.imgui.io_mut();
                self.mouse_buttons[button as usize] = true;
                io.mouse_down = self.mouse_buttons;
            }
            EventType::MouseButtonReleased { button } => {
                let io = self.imgui.io_mut();
                self.mouse_buttons[button as usize] = false;
                io.mouse_down = self.mouse_buttons;
            }
            EventType::MouseScrolled { x_offset, y_offset } => {
                let io = self.imgui.io_mut();
                io.mouse_wheel_h += x_offset as f32;
                io.mouse_wheel += y_offset as f32;
            }
            EventType::KeyPressed { key_code } => {
                let io = self.imgui.io_mut();
                io.key_ctrl = io.keys_down[glfw::Key::LeftControl as usize]
                    || io.keys_down[glfw::Key::RightControl as usize];
                io.key_shift = io.keys_down[glfw::Key::LeftShift as usize]
                    || io.keys_down[glfw::Key::RightShift as usize];
                io.key_alt = io.keys_down[glfw::Key::LeftAlt as usize]
                    || io.keys_down[glfw::Key::RightAlt as usize];
                io.key_super = io.keys_down[glfw::Key::LeftSuper as usize]
                    || io.keys_down[glfw::Key::RightSuper as usize];
                io.keys_down[key_code as usize] = true;
            }
            EventType::KeyReleased { key_code } => {
                let io = self.imgui.io_mut();
                io.keys_down[key_code as usize] = false;
            }
            EventType::KeyTyped { key_code } => {
                if key_code > 0 && key_code < 0x10000 {
                    let io = self.imgui.io_mut();
                    io.add_input_character(key_code as u8 as char);
                }
            }
            EventType::WindowClose => {}
            _ => {}
        }
        false
    }

    fn name(&self) -> &str {
        "ImGuiLayer"
    }
}
