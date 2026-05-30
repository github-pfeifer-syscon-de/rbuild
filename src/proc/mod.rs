use std::ffi::{OsStr, OsString};
use std::fs::DirEntry;
use std::path::{Path};
use crate::proc_run::BuildRunner;


pub trait BuildProcessTrait  {
    fn new(_:&str) -> Box<impl BuildProcessTrait> where Self: Sized;
    
    fn setBuildDir(&self,_:&OsStr);
    fn setRepo(&self,_:&OsStr);
    fn setAskpass(&self,_:&OsStr);
    fn addArg(&self, _:&OsStr);
    fn run(&self) -> Result<BuildRunner,String>;
    fn cmd(&self) -> OsString;
}

pub enum ShowEnum {
    Normal,
    Good,
    Warn,
    Error
}

pub struct MessageItem {
    pub msg: String,
    pub kind: ShowEnum,
}

pub fn findPackageFiles<T,F:Fn(&DirEntry,&mut T) -> Result<(),String> >(buildDir:&Path, t:&mut T, f:F) -> Result<(),String> {
    let paths = std::fs::read_dir(buildDir).unwrap();
    for buildEntry in paths {
        if buildEntry.is_ok() {
            let path = buildEntry.unwrap();
            let fileName = path.file_name();
            let fileString = fileName.to_str().unwrap();
            let fileType = path.file_type().unwrap();
            if fileString.ends_with(".zst")
                && fileType.is_file() {
                if let Err(result) = f(&path, t) {
                    return Err(result);
                }
            }
        }
    }
    return Ok(());
}