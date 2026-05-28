#![allow(non_snake_case)]

use std::cell::RefCell;
use std::ffi::OsString;

// ANCHOR: BuildProcess
pub struct BuildProcess {
    pub cmd: OsString,
    pub args: RefCell<Vec<OsString>>,
    pub buildDir: RefCell<OsString>,
}
// ANCHOR: BuildProcess