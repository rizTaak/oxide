use crate::oxide::{
    app::{Application, OxideApp},
    window::{GenericWindow, WindowProps},
};

pub struct SandboxApp {
    app: OxideApp<GenericWindow>,
}

impl SandboxApp {
    pub fn new() -> SandboxApp {
        SandboxApp {
            app: OxideApp::<GenericWindow>::new(WindowProps::new("Oxide Window", 1024, 720)),
        }
    }
}

impl Application for SandboxApp {
    fn run(&mut self) {
        self.app.run();
    }
}
