use std::ffi::{OsStr, OsString};
use crate::proc_run::BuildRunner;


pub trait BuildProcessTrait  {
    fn new(_:&str, ) -> Box<impl BuildProcessTrait> where Self: Sized;
    
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