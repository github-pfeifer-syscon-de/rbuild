#![allow(non_snake_case)]
mod imp;

use std::ffi::{OsStr, OsString};
use std::path::Path;

use crate::build::{checkAskPass, BuildModelTrait};
pub(crate) use crate::build_meson::imp::BuildModelMeson;
use crate::proc::BuildProcessTrait;
use crate::proc_inst;
use crate::proj::BuildProject;

impl BuildModelTrait for BuildModelMeson {
    fn new(name: &str) -> Box<impl BuildModelTrait> {
        let process = BuildModelMeson {
            name: name.into(),
        };
        Box::new(process)
    }

    fn build(&self, buildProject: &BuildProject) -> Result<Vec<Box<dyn BuildProcessTrait>>, String> {
        let mut tasks: Vec<Box<dyn BuildProcessTrait>> = Vec::new();
        let buildDir = buildProject.getBuildDir().clone();
        if !buildDir.is_dir() {
            let proc_setup = proc_inst::BuildProcessInst::new("meson");
            let mut parentDir = Path::new(buildDir.as_os_str());
            parentDir = parentDir.parent().unwrap();
            proc_setup.setBuildDir(parentDir.as_os_str());
            proc_setup.addArg(OsStr::new("setup"));
            proc_setup.addArg(OsStr::new("build"));
            let mut prefix= OsString::from("/usr");
            if let Some(mn_prefix) = std::env::var_os(BuildProject::MINGW_PREFIX) {
                prefix = mn_prefix;
            }
            let mut prefixParam = OsString::from("-Dprefix=");
            prefixParam.push(prefix);
            proc_setup.addArg(prefixParam.as_os_str());
            tasks.push(proc_setup);
        }
        else {
            let proc_uninstall = proc_inst::BuildProcessInst::new("sudo");
            proc_uninstall.setBuildDir(buildDir.as_os_str());
            checkAskPass(&proc_uninstall, buildProject);
            proc_uninstall.addArg(OsStr::new("ninja"));
            proc_uninstall.addArg(OsStr::new("uninstall"));
            tasks.push(proc_uninstall);
        }
        let proc_meson = proc_inst::BuildProcessInst::new("meson");
        proc_meson.setBuildDir(buildDir.as_os_str());
        proc_meson.addArg(OsStr::new("compile"));
        tasks.push(proc_meson);
        let proc_install = proc_inst::BuildProcessInst::new("sudo");
        proc_install.setBuildDir(buildDir.as_os_str());
        checkAskPass(&proc_install, buildProject);
        proc_install.addArg(OsStr::new("meson"));
        proc_install.addArg(OsStr::new("install"));
        tasks.push(proc_install);
        return Ok(tasks);
    }
}