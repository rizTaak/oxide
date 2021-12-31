use std::{ffi::c_void, time::Instant};

use glfw::{ffi, with_c_str};
use imgui::{BackendFlags, Context};

use super::layer::Layer;
use crate::oxide_info;
use imgui_opengl_renderer::Renderer;
pub struct ImGuiLayer {
    imgui: imgui::Context,
    renderer: Renderer,
    last_frame: Instant,
}

impl ImGuiLayer {
    pub fn new() -> ImGuiLayer {
        let mut imgui = Context::create();
        let renderer = Renderer::new(&mut imgui, |s| {
            with_c_str(s, |procname| unsafe { ffi::glfwGetProcAddress(procname) })
        });

        /*
        unsafe {
            let window_ptr = glfw::ffi::glfwGetCurrentContext() as *mut c_void;
            imgui.set_clipboard_backend(Box::new(GlfwClipboardBackend(window_ptr)));
        }*/

        ImGuiLayer {
            imgui,
            last_frame: Instant::now(),
            renderer,
        }
    }
}

impl Layer for ImGuiLayer {
    fn on_attach(&mut self) {
        let io = self.imgui.io_mut();
        io.backend_flags |= BackendFlags::HAS_MOUSE_CURSORS;
        io.backend_flags |= BackendFlags::HAS_SET_MOUSE_POS;
        // theme ?
    }

    fn on_detach(&mut self) {
        oxide_info!("{}: on_detach", self.name());
    }

    fn on_update(&mut self) {
        oxide_info!("{}: on_update", self.name());

        let io = self.imgui.io_mut();

        let now = Instant::now();
        let delta = now - self.last_frame;
        let delta_s = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1_000_000_000.0;
        self.last_frame = now;
        io.delta_time = delta_s;

        let window_size = (1024, 720); //window.get_size();
        io.display_size = [window_size.0 as f32, window_size.1 as f32];

        let ui = self.imgui.frame();
        let mut opened = true;
        ui.show_demo_window(&mut opened);
        self.renderer.render(ui);
        /*
        Application& app = Application::Get();
        io.DisplaySize = ImVec2(app.GetWindow().GetWidth(), app.GetWindow().GetHeight());

        float time = (float)glfwGetTime();
        io.DeltaTime = m_Time > 0.0f ? (time - m_Time) : (1.0f / 60.0f);
        m_Time = time;

        ImGui_ImplOpenGL3_NewFrame();
        ImGui::NewFrame();

        static bool show = true;
        ImGui::ShowDemoWindow(&show);

        ImGui::Render();
        ImGui_ImplOpenGL3_RenderDrawData(ImGui::GetDrawData()); */
    }

    fn on_event(&mut self, event: &super::event::Event) {
        oxide_info!("{}: {:?}", self.name(), event);
    }

    fn name(&self) -> &str {
        "ImGuiLayer"
    }
}
