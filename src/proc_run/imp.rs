#![allow(non_camel_case_types)]

use std::cell::RefCell;
use std::process::Child;
use std::sync::mpsc::Receiver;
use std::thread::JoinHandle;
use crate::proc::MessageItem;

pub struct BuildRunner {
    pub procChild: RefCell<Child>,
    pub stderr_thread: RefCell<Option<JoinHandle<()>>>,
    pub stdout_thread: RefCell<Option<JoinHandle<()>>>,
    pub receiver: Receiver<MessageItem>,

}