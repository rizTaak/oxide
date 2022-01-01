#![cfg_attr(debug_assertions, allow(dead_code))]

mod external;
mod oxide;
mod sandbox;

use std::path::Path;

use crate::oxide::app::Application;

fn main() {
    if Path::new("log4rs.yml").exists() {
        log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    } else {
        println!("Skipping log setup as log4rs.yml not found.")
    }
    oxide_info!("Starting Oxide");
    let mut app = sandbox::app::SandboxApp::new();
    app.run();
    oxide_info!("Stopping Oxide");
    app.close();
    oxide_info!("Oxide Stopped");
}
