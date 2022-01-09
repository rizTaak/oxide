use crate::oxide::{app::Application, default::app::OxideApp, imgui::layer::ImGuiLayer};


pub struct SandboxApp {
    app: OxideApp,
}

impl Application for SandboxApp {
    fn new() -> SandboxApp {
        let mut sandbox = SandboxApp {
            app: OxideApp::new(),
        };
        sandbox.push_layer(Box::new(ImGuiLayer::new()));
        sandbox
    }

    fn push_layer(&mut self, layer: Box<dyn crate::oxide::layer::Layer>) {
        self.app.push_layer(layer);
    }

    fn push_overlay(&mut self, layer: Box<dyn crate::oxide::layer::Layer>) {
        self.app.push_overlay(layer);
    }

    fn close(&mut self) {
        self.app.close();
    }

    fn is_running(&self) -> bool {
        self.app.is_running()
    }

    fn notify(&mut self, event: &crate::oxide::event::Event) {
        self.app.notify(event);
    }

    fn layers(&mut self) -> &mut crate::oxide::layer::LayerStack {
        self.app.layers()
    }
}
