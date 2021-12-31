use crate::oxide::{
    app::{Application, OxideApp, OxideAppObserver},
    imgui::ImGuiLayer,
    window::{GenericWindow, WindowProps},
};

use super::example_layer::ExampleLayer;

pub struct SandboxApp {
    app: OxideApp<GenericWindow<OxideAppObserver>>,
}

impl SandboxApp {
    pub fn new() -> SandboxApp {
        let mut sandbox = SandboxApp {
            app: OxideApp::<GenericWindow<OxideAppObserver>>::new(WindowProps::new(
                "Oxide Window",
                1024,
                720,
            )),
        };
        sandbox.push_layer(Box::new(ImGuiLayer::new()));
        sandbox.push_layer(Box::new(ExampleLayer {}));
        sandbox
    }
}

impl Application for SandboxApp {
    fn run(&mut self) {
        self.app.run();
    }

    fn push_layer(&mut self, layer: Box<dyn crate::oxide::layer::Layer>) {
        self.app.push_layer(layer);
    }

    fn push_overlay(&mut self, layer: Box<dyn crate::oxide::layer::Layer>) {
        self.app.push_overlay(layer);
    }
}
