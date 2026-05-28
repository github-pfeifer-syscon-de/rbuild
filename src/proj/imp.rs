#![allow(non_snake_case)]

use glib::{BoxedAnyObject, Properties};
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use std::cell::RefCell;
use super::ProjData;

// ANCHOR: struct_and_subclass
// Object holding the state
#[derive(Properties, Default)]
#[properties(wrapper_type = super::BuildProject)]
pub struct BuildProject {
    #[property(name = "name", get, set, type = String, member = name)]
    #[property(name = "buildChanged", get, set, type = String, member = buildChanged)]
    #[property(name = "srcChanged", get, set, type = String, member = srcChanged)]
    #[property(name = "diff", get, set, type = String, member = diff)]
    #[property(name = "path", get, set, type = BoxedAnyObject, member = path)]
    #[property(name = "repo", get, set, type = BoxedAnyObject, member = repo)]
    #[property(name = "askpass", get, set, type = BoxedAnyObject, member = askpass)]
    pub data: RefCell<ProjData>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for BuildProject {
    const NAME: &'static str = "BuildProject";
    type Type = super::BuildProject;
}

// Trait shared by all GObjects
#[glib::derived_properties]
impl ObjectImpl for BuildProject {}
// ANCHOR_END: struct_and_subclass
