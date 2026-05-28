#![allow(non_snake_case)]

use std::cell::RefCell;
use std::collections::VecDeque;
use glib::KeyFile;
use glib::subclass::InitializingObject;
use gtk::subclass::prelude::*;
use gtk::{gio, glib, Button, ColumnView, CompositeTemplate, TextTag, TextView};
use crate::proc::BuildProcessTrait;

// ANCHOR: struct_and_subclass
// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/rbuild/BuildWin.ui")]
pub struct Window {
    #[template_child]
    pub table: TemplateChild<ColumnView>,
    #[template_child]
    pub build: TemplateChild<Button>,
    #[template_child]
    pub text: TemplateChild<TextView>,
    pub projects: RefCell<Option<gio::ListStore>>,
    pub tasks: RefCell<VecDeque<Box<dyn BuildProcessTrait>>>,
    pub keyFile: RefCell<Option<KeyFile>>,
    pub textTagWarn: RefCell<Option<TextTag>>,
    pub textTagGood: RefCell<Option<TextTag>>,
    pub textTagFail: RefCell<Option<TextTag>>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for Window {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "buildWin";
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
        obj.setup_build();
        obj.setup_table();
        obj.setup_text();
        obj.setup_callbacks();
        obj.setup_factory();
    }
}
// ANCHOR_END: constructed

// Trait shared by all widgets
impl WidgetImpl for Window {}

// Trait shared by all windows
impl WindowImpl for Window {}

// Trait shared by all application windows
impl ApplicationWindowImpl for Window {}