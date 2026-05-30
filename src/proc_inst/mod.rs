#![allow(non_snake_case)]
mod imp;

use crate::proc::BuildProcessTrait;
use crate::proc_run::BuildRunner;
pub(crate) use imp::BuildProcessInst;
use glib::property::{PropertySet};
use std::boxed::Box;
use std::cell::RefCell;
use std::ffi::{OsStr, OsString};
use std::format;
use std::path::Path;
use std::process::{Command, Stdio};
use crate::proj::BuildProject;

impl BuildProcessTrait for BuildProcessInst {
    fn new(scmd:&str) -> Box<impl BuildProcessTrait> {
        let cmd = OsString::from(scmd);
        let argsVec = Vec::new();
        let args = RefCell::new(argsVec);
        let buildDir = RefCell::new(OsString::new());
        let askpass = RefCell::new(OsString::new());
        let process = BuildProcessInst {
            cmd
            ,args
            ,buildDir
            ,askpass
        };
        Box::new(process)
    }
    fn addArg(&self, str:&OsStr) {
        let mut mutArgs = self.args.borrow_mut();
        mutArgs.push(str.to_os_string());
    }
    fn setBuildDir(&self,buildDir: &OsStr) {
        self.buildDir.set(buildDir.to_os_string());
    }
    fn setRepo(&self,_:&OsStr) {
    }
    fn setAskpass(&self,askpass:&OsStr) {
        self.askpass.set(askpass.to_os_string());
    }
    fn run(&self) -> Result<BuildRunner,String> {
        let build = self.buildDir.borrow();
        let buildDir = Path::new(build.as_os_str());
        let cmd = self.cmd.clone();
        println!("Command \"{}\" dir \"{}\"", cmd.display(), buildDir.display());
        let args = self.args.borrow().to_vec();
        //for arg in args.clone() {
        //    println!("   arg \"{}\"", arg.display());
        //}
        let mut command = Command::new(cmd);
        command.args(args)
            .stderr(Stdio::piped())
            .stdout(Stdio::piped())
            .current_dir(buildDir);
        let refAskpass = self.askpass.borrow();
        let askpass = refAskpass.as_os_str();
        if !askpass.is_empty() {
            //println!("   setting {}=\"{}\"", BuildProject::SUDO_ASKPASS, askpass.display());
            command.env(BuildProject::SUDO_ASKPASS, askpass.to_os_string());
        }
        let result = command.spawn();
        if let Ok(child) = result {
            return Ok(BuildRunner::new(child));
        }
        else {
            let msg = format!("Error {} running {}"
                              , result.unwrap_err(), self.cmd.display());
            return Err(msg);
        }
    }
    fn cmd(&self) -> OsString {
        return self.cmd.clone();
    }
    
    
}

