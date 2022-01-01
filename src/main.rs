#![cfg_attr(debug_assertions, allow(dead_code))]

mod external;
mod oxide;
mod sandbox;

use crate::oxide::app::Application;

fn main() {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    oxide_info!("Starting Oxide");
    let mut app = sandbox::app::SandboxApp::new();
    app.run();
    oxide_info!("Stopping Oxide");
    app.close();
}
