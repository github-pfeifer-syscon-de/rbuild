#![allow(non_snake_case)]

use std::cell::RefCell;
use std::ffi::{OsString};

// ANCHOR: BuildProcessInst
pub struct BuildProcessInst {
    pub cmd: OsString,
    pub args: RefCell<Vec<OsString>>,
    pub buildDir: RefCell<OsString>,
    pub askpass: RefCell<OsString>,
}
// ANCHOR: BuildProcessInst