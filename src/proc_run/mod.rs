#![allow(non_snake_case)]
mod imp;

use crate::proc::{MessageItem, ShowEnum};
pub(crate) use imp::BuildRunner;
use std::cell::RefCell;
use std::io::{BufRead, BufReader};
use std::os::unix::prelude::ExitStatusExt;
use std::process::Child;
use std::sync::mpsc;
use std::thread;
use std::thread::JoinHandle;

impl BuildRunner {
    pub fn new(mut child: Child) -> Self {
        // using threads seems the best way to capture output, or we have to go to tokyo?
        let (txOut, receiver) = mpsc::sync_channel(1000);
        let txStdout = txOut.clone();
        let stdout = child.stdout.take()
            .expect("Failed to capture stderr");
        let stdout_reader = BufReader::new(stdout);
        // Create threads for reading stdout and stderr
        //   the trick is to keep child.stdout on the outside and only "move" BufReader !!!
        let stdout_thread = thread::spawn(move||{
            let stdout_iter = stdout_reader.lines();
            for line in stdout_iter {
                if let Ok(line) = line {
                    txStdout.send(MessageItem {
                        msg: line+"\n"
                        , kind: ShowEnum::Normal
                    }).unwrap();
                }
            }
        });
        let txStderr = txOut.clone();
        let stderr = child.stderr.take()
            .expect("Failed to capture stderr");
        let stderr_reader = BufReader::new(stderr);
        let stderr_thread = thread::spawn(move||{
            let stderr_iter = stderr_reader.lines();
            for line in stderr_iter {
                if let Ok(line) = line {
                    txStderr.send(MessageItem {
                        msg: line+"\n"
                        , kind: ShowEnum::Warn
                    }).unwrap();
                }
            }
        });
        drop(txOut);    // drop the original (or we will wait forever)
        let procChild = RefCell::new(child);
        let stdout_thread = RefCell::new(Some(stdout_thread));
        let stderr_thread = RefCell::new(Some(stderr_thread));
        BuildRunner {
            procChild
            , stdout_thread
            , stderr_thread
            , receiver
        }
    }

    // intended to be called from std-thread (try to be non-blocking)
    pub fn read<F:Fn(&String, ShowEnum)>(&self, func: F) -> Option<Result<String, String>> {
        // try reading as many messages as we can
        for item in self.receiver.try_iter() {
            func(&item.msg, item.kind);
        }
        // check the process ended...
        let mut child = self.procChild.borrow_mut();
        if let Ok(someChildExit) = child.try_wait() {
            if let Some(ret) = someChildExit {
                let msg :Result<String, String>;
                if let Some(sign) = ret.signal() {
                    let txt = format!("Signal {}\n", sign);
                    func(&txt,ShowEnum::Error);
                    msg = Err(txt);
                }
                else if let Some(exit) = ret.code() {
                    if exit != 0 {
                        let txt = format!("Returncode {}\n", exit);
                        func(&txt,ShowEnum::Error);
                        msg = Err(txt);
                    } else {
                        let txt = String::from("Ok\n");
                        func(&txt,ShowEnum::Good);
                        msg = Ok(txt);
                    }
                }
                else {
                    let txt = String::from("No signal no exit ?");
                    func(&txt,ShowEnum::Warn);
                    msg = Err(txt);
                }
                //let out = msg.clone();
                //if let Ok(ok) = out {
                    //self.showErr(text_view, &ok);
                //}
                //else {
                    //self.showErr(text_view, &out.unwrap_err());
                //}
                let stderr_empty = None::<JoinHandle<()>>;
                let mstdErr = self.stderr_thread.replace(stderr_empty)
                    .unwrap();
                mstdErr.join()
                    .expect("stdout thread panicked");
                let stdout_empty = None::<JoinHandle<()>>;
                let mstdOut = self.stdout_thread.replace(stdout_empty)
                    .unwrap();
                mstdOut.join()
                    .expect("stderr thread panicked");
                return Some(msg);    // we are done
            }
            else {// not yet exited
            }
        }
        return None;   // keep asking
    }

}