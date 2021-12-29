use crate::oxide::{
    app::{Application, OxideApp, OxideAppObserver},
    window::{GenericWindow, Window, WindowProps},
};

pub struct SandboxApp {
    app: OxideApp<GenericWindow<OxideAppObserver>>,
}

impl SandboxApp {
    pub fn new() -> SandboxApp {
        SandboxApp {
            app: OxideApp::<GenericWindow<OxideAppObserver>>::new(WindowProps::new(
                "Oxide Window",
                1024,
                720,
            )),
        }
    }

    pub fn register_callback(&mut self) {
        self.app
            .window
            .set_callback(Some(self.app.observer.clone()));
    }
}

impl<'a> Application for SandboxApp {
    fn run(&mut self) {
        self.app.run();
    }
}
