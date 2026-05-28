#![allow(non_snake_case)]

use std::cell::RefCell;
use std::ffi::OsString;

// ANCHOR: BuildProcessRepo
pub struct BuildProcessRepo {
    pub cmd: OsString,
    pub args: RefCell<Vec<OsString>>,
    pub buildDir: RefCell<OsString>,
    pub repo: RefCell<OsString>,
}
// ANCHOR: BuildProcessRepo