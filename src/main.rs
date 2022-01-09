#![cfg_attr(debug_assertions, allow(dead_code))]

mod external;
mod oxide;
mod sandbox;

use std::path::Path;

use crate::{
    oxide::{glfw::window::GenericWindow, oxide::Oxide, window::WindowProps},
    sandbox::app::SandboxApp,
};

fn main() {
    if Path::new("log4rs.yml").exists() {
        log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    } else {
        println!("Skipping log setup as log4rs.yml not found.")
    }
    oxide_info!("Starting Oxide");
    let mut oxide = Oxide::<SandboxApp, GenericWindow<SandboxApp>>::new(WindowProps::new(
        "Oxide Window",
        1536,
        864,
    ));
    oxide.run();
    oxide_info!("Stopping Oxide");
    oxide.close();
    oxide_info!("Oxide Stopped");
}
