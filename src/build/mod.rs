#![allow(non_snake_case)]

use std::cell::Ref;
use std::ffi::{OsStr, OsString};

use crate::proc::BuildProcessTrait;
use crate::proj::BuildProject;

pub trait BuildModelTrait  {
    fn new(_:&str) -> Box<impl crate::build::BuildModelTrait> where Self: Sized;

    fn build(&self,_:&BuildProject) -> Result<Vec<Box<dyn BuildProcessTrait>>,String>;

}

pub fn checkAskPass(proc_install: &Box<impl BuildProcessTrait>, buildProject: &BuildProject) {
    if let None = std::env::var_os(BuildProject::SUDO_ASKPASS) {
        let boxAskpass = buildProject.askpass();
        let refAskpass: Ref<OsString> = boxAskpass.borrow();
        let askPass = OsString::from(refAskpass.as_os_str());
        if askPass.is_empty() {
            println!("A existing environment var SUDO_ASKPASS was not found, \
                          and the setting ~/.config/pyBuild.conf Askpass=path_askpass_executable was missing as well, \
                          this might work if the user can use sudo directly (not recommended)");
        } else {
            proc_install.setAskpass(askPass.as_os_str());   // use config if not preconfigured
            proc_install.addArg(OsStr::new("--askpass"));
        }
    } else {  // if env exists use it
        proc_install.addArg(OsStr::new("--askpass"));
    }
}
