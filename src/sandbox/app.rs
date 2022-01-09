use crate::oxide::{
    app::Application, default::app::OxideApp, imgui::layer::ImGuiLayer, window::WindowProps,
};

pub struct SandboxApp {
    app: OxideApp,
}

impl Application for SandboxApp {
    fn new(props: &WindowProps) -> SandboxApp {
        let mut sandbox = SandboxApp {
            app: OxideApp::new(props),
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

    fn notify(&mut self, event: &crate::oxide::event::OxideEvent) {
        self.app.notify(event);
    }

    fn layers(&mut self) -> &mut crate::oxide::layer::LayerStack {
        self.app.layers()
    }

    /*
    fn width(&self) -> i32 {
        self.app.width()
    }

    fn height(&self) -> i32 {
        self.app.height()
    }

    fn set_width(&mut self, width: i32) {
        self.app.set_width(width);
    }

    fn set_height(&mut self, height: i32) {
        self.app.set_height(height);
    }

    fn name(&self) -> &'static str {
        todo!()
    } */
}
