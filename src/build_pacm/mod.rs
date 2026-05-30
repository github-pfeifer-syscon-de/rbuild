#![allow(non_snake_case)]
mod imp;

use crate::build::{checkAskPass, BuildModelTrait};
pub(crate) use crate::build_pacm::imp::BuildModelPacman;
use crate::proc::BuildProcessTrait;
use crate::proj::BuildProject;
use crate::{proc_inst, proc_pack, proc_repo};
use std::cell::Ref;
use std::ffi::{OsStr, OsString};
use std::os::unix::fs;
use std::path::Path;

impl BuildModelTrait for BuildModelPacman {
    fn new(name: &str) -> Box<impl BuildModelTrait> {
        let process = BuildModelPacman {
            name: name.into(),
        };
        Box::new(process)
    }


    fn build(&self, buildProject: &BuildProject) -> Result<Vec<Box<dyn BuildProcessTrait>>, String> {
        let csrcPkgBuild = buildProject.getCsrcPkgbuild();
        if !csrcPkgBuild.is_file() {
            let err = format!("The pkgbuild script {} was not found", csrcPkgBuild.as_os_str().display());
            return Err(err);
        }
        let buildDir = buildProject.getBuildDir().clone();
        if !buildDir.is_dir() {
            let err = format!("The build dir {} was not found or is not a directory", buildDir.as_os_str().display());
            return Err(err);
        }
        let buildPkgbuild: Box<Path> = buildProject.getBuildPkgbuild();
        if !buildPkgbuild.is_symlink() {
            let linkErr = fs::symlink(csrcPkgBuild, buildPkgbuild.as_ref());
            if linkErr.is_err() {
                let err = format!("The link {} was not created", buildPkgbuild.as_os_str().display());
                return Err(err);
            }
        }
        let boxRepo = buildProject.repo();
        let refRepo: Ref<OsString> = boxRepo.borrow();
        let repo: OsString = refRepo.to_os_string();
        let mut tasks: Vec<Box<dyn BuildProcessTrait>> = Vec::new();
        let proc_makepkg = proc_pack::BuildProcess::new("makepkg");
        proc_makepkg.setBuildDir(buildDir.as_os_str());
        proc_makepkg.addArg(OsStr::new("--syncdeps"));
        proc_makepkg.addArg(OsStr::new("--force"));
        tasks.push(proc_makepkg);
        let proc_repo_add = proc_repo::BuildProcessRepo::new("repo-add");
        proc_repo_add.setBuildDir(buildDir.as_os_str());
        proc_repo_add.setRepo(repo.as_os_str());
        tasks.push(proc_repo_add);
        let proc_install = proc_inst::BuildProcessInst::new("sudo");
        checkAskPass(&proc_install, buildProject);
        proc_install.setBuildDir(buildDir.as_os_str());
        proc_install.addArg(OsStr::new("pacman"));
        proc_install.addArg(OsStr::new("--noconfirm"));
        proc_install.addArg(OsStr::new("--sync"));
        proc_install.addArg(OsStr::new("--refresh"));
        let mut packName =
            if let Some(packname) = buildProject.readPackageName() {
                packname
            } else {
                OsString::from(buildProject.name().as_str())
            };
        proc_install.addArg(packName.as_os_str());
        packName.push("-debug");
        proc_install.addArg(packName.as_os_str());
        tasks.push(proc_install);
        return Ok(tasks);
    }
}