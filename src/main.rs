#![allow(non_snake_case)]
mod window;
mod proc;
mod proj;
mod proc_pack;
mod proc_run;
mod proc_repo;
mod proc_inst;

use gtk::prelude::*;
use gtk::{Application, gio, glib};
use window::Window;
const APP_ID: &str = "de.pfeifer_syscon.rbuild";

fn main() -> glib::ExitCode {
    // Register and include resources
    gio::resources_register_include!("rbuild.gresource")
        .expect("Failed to register resources.");

    // Create a new application
    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    // Create a new custom window and present it
    let window = Window::new(app);
    window.present();
}

