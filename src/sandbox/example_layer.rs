use crate::oxide::layer::Layer;
use crate::oxide_info;
pub struct ExampleLayer {}

impl Layer for ExampleLayer {
    fn on_attach(&mut self) {}

    fn on_detach(&mut self) {}

    fn on_update(&mut self) {
        // oxide_info!("ExampleLayer: on_update");
    }

    fn on_event(&mut self, event: &crate::oxide::event::Event) -> bool {
        oxide_info!("ExampleLayer: {:?}", event);
        false
    }

    fn name(&self) -> &str {
        "Example Layer"
    }
}
