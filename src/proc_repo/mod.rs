#![allow(non_snake_case)]
mod imp;

use crate::proc::BuildProcessTrait;
use crate::proc_run::BuildRunner;
pub(crate) use imp::BuildProcessRepo;
use glib::property::PropertySet;
use std::boxed::Box;
use std::cell::RefCell;
use std::ffi::{OsStr, OsString};
use std::format;
use std::fs::{copy, remove_file};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

impl BuildProcessTrait for BuildProcessRepo {
    fn new(scmd:&str) -> Box<impl BuildProcessTrait> {
        let cmd = OsString::from(scmd);
        let argsVec = Vec::new();
        let args = RefCell::new(argsVec);
        let buildDir = RefCell::new(OsString::new());
        let repo = RefCell::new(OsString::new());
        let process = BuildProcessRepo {
            cmd
            ,args
            ,buildDir
            ,repo
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
    fn setRepo(&self,repo:&OsStr) {
        self.repo.set(repo.to_os_string());
    }
    fn setAskpass(&self,_askpass:&OsStr) {
    }
    fn run(&self) -> Result<BuildRunner,String> {
        let build = self.buildDir.borrow();
        let repo = self.repo.borrow();
        let buildDir = Path::new(build.as_os_str());
        let repoDbPath = Path::new(repo.as_os_str());
        let repoPath = repoDbPath.parent()
            .expect("The parent of repo-db was not found!");
        let mut argsVec = Vec::new();
        argsVec.push(repo.to_os_string());
        let paths = std::fs::read_dir(buildDir).unwrap();
        for buildEntry in paths {
            if buildEntry.is_ok() {
                let path = buildEntry.unwrap();
                let pathPath = path.path();
                let fileName = path.file_name();
                let fileString = fileName.to_str().unwrap();
                let fileType = path.file_type().unwrap();
                if fileString.ends_with(".zst")
                    && fileType.is_file() {
                    let mut destPath = PathBuf::new();
                    destPath.push(repoPath.as_os_str());
                    destPath.push(fileName.as_os_str());
                    let copyRes = copy(&pathPath, destPath.as_path());  // not as comfortable as python, rename refuses to move between filesystems
                    if let Err(err ) = copyRes {
                        let msg = format!("Error {} copying package file: {}", err, fileString);
                        return Err(msg);
                    }
                    else {
                        let _removeErr = remove_file(&pathPath);
                        println!("adding: {}", fileString);
                        argsVec.push(destPath.into_os_string());
                    }
                }
            }
        }
        let cmd = self.cmd.clone();
        let result = Command::new(cmd)
            .args(argsVec)
            .stderr(Stdio::piped())
            .stdout(Stdio::piped())
            .current_dir(buildDir)
            .spawn();
        if let Ok(child) = result {
            return Ok(BuildRunner::new(child));
        }
        else {
            let msg = format!("Error {} running {} \n"
                              , result.unwrap_err(), self.cmd.display());
            return Err(msg);
        }
    }
    fn cmd(&self) -> OsString {
        return self.cmd.clone();
    }
}

