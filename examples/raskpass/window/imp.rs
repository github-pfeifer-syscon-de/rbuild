#![allow(non_snake_case)]

use std::cell::RefCell;
use glib::object_subclass;
use glib::subclass::InitializingObject;
use gtk::subclass::prelude::*;
use gtk::{glib, Button, Label, PasswordEntry, Entry, CompositeTemplate};

// ANCHOR: struct_and_subclass
// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/rask/AskWin.ui")]
pub struct Window {
    #[template_child]
    pub label:TemplateChild<Label>,
    #[template_child]
    pub pass: TemplateChild<PasswordEntry>,
    #[template_child]
    pub ok: TemplateChild<Button>,
    #[template_child]
    pub cancel: TemplateChild<Button>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for Window {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "askWin";
    type Type = super::Window;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}
// ANCHOR_END: struct_and_subclass

// ANCHOR: constructed
// Trait shared by all GObjects
impl ObjectImpl for Window {
    fn constructed(&self) {
        // Call "constructed" on parent
        self.parent_constructed();

        // Setup
        let obj = self.obj();
        obj.setup_buttons();
    }
}
// ANCHOR_END: constructed

// Trait shared by all widgets
impl WidgetImpl for Window {}

// Trait shared by all windows
impl WindowImpl for Window {}

// Trait shared by all application windows
impl ApplicationWindowImpl for Window {}