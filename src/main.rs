#![cfg_attr(debug_assertions, allow(dead_code))]

mod external;
mod oxide;
mod sandbox;

use crate::oxide::Application;

fn main() {
    let app = sandbox::SandboxApp::new();
    app.run();
}
