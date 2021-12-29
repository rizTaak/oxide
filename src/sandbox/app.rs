use crate::oxide::{
    app::{Application, OxideApp, OxideAppObserver},
    window::{GenericWindow, Window, WindowProps},
};

pub struct SandboxApp<'a> {
    app: OxideApp<'a, GenericWindow<'a, OxideAppObserver>>,
}

impl<'a> SandboxApp<'a> {
    pub fn new() -> SandboxApp<'a> {
        SandboxApp {
            app: OxideApp::<GenericWindow<'a, OxideAppObserver>>::new(WindowProps::new(
                "Oxide Window",
                1024,
                720,
            )),
        }
    }

    pub fn register_callback(&'a mut self) {
        self.app.window.set_callback(&mut self.app.observer);
    }
}

impl<'a> Application for SandboxApp<'a> {
    fn run(&mut self) {
        self.app.run();
    }
}
