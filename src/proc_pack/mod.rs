#![allow(non_snake_case)]
mod imp;

use crate::proc::{findPackageFiles, BuildProcessTrait};
use crate::proc_run::BuildRunner;
pub(crate) use imp::BuildProcess;
use glib::property::PropertySet;
use std::boxed::Box;
use std::cell::RefCell;
use std::ffi::{OsStr, OsString};
use std::path::Path;
use std::process::{Command, Stdio};
use std::format;
use std::fs::DirEntry;

impl BuildProcessTrait for BuildProcess {
    fn new(scmd:&str) -> Box<impl BuildProcessTrait> {
        let cmd = OsString::from(scmd);
        let argsVec = Vec::new();
        let args = RefCell::new(argsVec);
        let buildDir = RefCell::new(OsString::new());
        let process = BuildProcess {
            cmd
            ,args
            ,buildDir
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
    fn setAskpass(&self,_:&OsStr) {
    }
    fn run(&self) -> Result<BuildRunner,String> {
        let build = self.buildDir.borrow();
        let buildDir = Path::new(build.as_os_str());
        let args = self.args.borrow().to_vec();
        let cmd = self.cmd.clone();
        println!("Command \"{}\" dir \"{}\"", cmd.display(), buildDir.display());
        let mut argsVec = Vec::new();
        // as a preparation step remove any leftover package files
        if let Err(res) = findPackageFiles(buildDir, &mut argsVec, |path: &DirEntry, argVec:&mut Vec<()>| -> Result<(),String> {
            let fileName = path.file_name();
            let fileString = fileName.to_str().unwrap();
            println!("removing: {}", fileString);
            let pathPath = path.path();
            if let Err(err) = std::fs::remove_file(pathPath.as_path()) {
                return Err(format!("Error {} removing {} form build dir", err, fileName.display()));
            }
            Ok(())
        }) {
            return Err(res);
        }
        let result = Command::new(cmd)
            .args(args)
            .stderr(Stdio::piped())
            .stdout(Stdio::piped())
            .current_dir(buildDir)
            .spawn();
        if let Ok(child) = result {
            return Ok(BuildRunner::new(child));
        }
        else {
            let msg = format!("Error {} running {}\n"
                              , result.unwrap_err(), self.cmd.display());
            return Err(msg);
        }
    }
    fn cmd(&self) -> OsString {
        return self.cmd.clone();
    }

}

