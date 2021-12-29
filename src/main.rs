#![cfg_attr(debug_assertions, allow(dead_code))]
#![feature(once_cell)]

mod external;
mod oxide;
mod sandbox;

use crate::oxide::app::Application;

fn main() {
    let mut app = sandbox::app::SandboxApp::new();
    app.run();
}
