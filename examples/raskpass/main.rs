#![allow(non_snake_case)]
mod window;

use std::env;
use std::env::args;
use std::ffi::{OsStr, OsString};
use std::fs::File;
use std::io::{Error, Write};
use std::path::PathBuf;
use glib::{ExitCode, GStr};
use glib::ffi::GIOFlags;
use gtk::prelude::*;
use gtk::{Application, gio, glib};
use gtk::gio::{ApplicationCommandLine, ApplicationFlags};
use window::Window;
const APP_ID: &str = "de.pfeifer_syscon.raskpass";


fn main() -> glib::ExitCode {
    // Register and include resources
    gio::resources_register_include!("rask.gresource")
        .expect("Failed to register rask.gresource.");

    // Create a new application
    let app = Application::builder()
        .application_id(APP_ID)
        .flags(ApplicationFlags::HANDLES_COMMAND_LINE)
        .build();
    // need to handle open as su is passing args, I guess
    //println!("connect_open");
    app.connect_command_line(handle_cmdline);
    // Connect to "activate" signal of `app` -> works without args
    //println!("connect_activate");
    //app.connect_activate(build_ui);   with args this will never be called
    // Run the application
    //let osArgs = env::args_os();
    //let args: Vec<OsString> = osArgs.collect();
    //app.run_with_args(args.as_array().unwrap() )
    app.run()
}

fn build_ui(app: &Application) {
    println!("build_ui");
}


fn handle_cmdline(app: &Application, cmd:&ApplicationCommandLine) -> ExitCode {
    // Create a new custom window and present it
    // if let Some(home)  = env::home_dir() {
    //     let mut homeBuf = PathBuf::new();
    //     homeBuf.push(home);
    //     homeBuf.push("ask.log");
    //     let mut file = File::create(homeBuf)
    //         .expect("Error opening file");
    //     match file.write_all(b"---\n") {
    //         Ok(size)  => {},
    //         Err(err) => {
    //             println!("Error {} writing ", err);
    //         }
    //     };
    //    for arg in cmd.arguments() {
            //let str = format!("Arg {}\n", arg.display());
            //match file.write_all(str.as_str().as_bytes()) {
            //    Ok(size) => {},
            //    Err(err) => {
            //        println!("Error {} writing ", err);
            //    }
            //};
    //    }
    //}

    let window = Window::new(app);
    let args = cmd.arguments();
    if args.len() > 1 {
        let hint = args.get(1).unwrap();
        window.set_hint(hint);
    }
    window.present();
    ExitCode::SUCCESS
}
