#![allow(non_snake_case)]
mod imp;

use std::ffi::OsStr;
use glib::{clone, Object};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib, Application, Label, PasswordEntry, TextDirection};
use std::fmt::Write;
use std::ops::Deref;

// ANCHOR: glib_wrapper
glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}
// ANCHOR_END: glib_wrapper


impl Window {
    pub fn new(app: &Application) -> Self {
        // Create new window
        Object::builder()
            .property("application", app)
            .build()
    }

    pub fn set_hint(&self, hint: &OsStr) {
        let txt = hint.display().to_string();
        self.imp().label.set_text(&txt);
    }
    
    fn printText(&self) {
        let text = self.imp().pass.text();
        println!("{}", text.as_str());
    }

    fn setup_buttons(&self) {
        self.imp().ok.connect_clicked(clone!(
             #[weak(rename_to = window)]
             self,
             move |_| {
                 window.printText();
                 window.close();
             }
        ));
        self.imp().cancel.connect_clicked(clone!(
             #[weak(rename_to = window)]
             self,
             move |_| {
                 window.close();
                // unsure use exit code for failed ?
                //   using is blocked by glib.run std::process::exit(exit_code); ???
             }
        ));
        self.imp().pass.connect_activate(clone!(
             #[weak(rename_to = window)]
             self,
             move |_| {
                 window.printText();
                 window.close();
             }
        ));
        if let Ok(askpass) = std::env::var("ASKPASS") {
            let str = askpass.to_string();
            self.imp().label.set_text(&str);
        }
    }

}