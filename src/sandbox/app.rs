use crate::oxide::{Application, OxideApp};

pub struct SandboxApp {
    app: OxideApp
}

impl SandboxApp {
    pub fn new() -> SandboxApp {
        SandboxApp {
            app: OxideApp::new()
        }
    }
}

impl Application for SandboxApp {
    fn run(&self) {
        self.app.run();
    }
}