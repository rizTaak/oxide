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
}

impl<'a> Application for SandboxApp {
    fn run(&mut self) {
        self.app.run();
    }
}
