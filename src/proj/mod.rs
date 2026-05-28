#![allow(non_snake_case)]
mod imp;

use crate::proc::BuildProcessTrait;
use crate::{proc_inst, proc_pack, proc_repo};
use chrono::{DateTime, Local};
use glib::{BoxedAnyObject, Object};
use gtk::glib;
use gtk::prelude::{ObjectExt};
use std::cell::Ref;
use std::ffi::{OsStr, OsString};
use std::format;
use std::fs::{read_to_string, DirEntry};
use std::ops::{Add};
use std::os::unix::fs;
use std::path::{Path, PathBuf};

// ANCHOR: glib_wrapper_and_new
glib::wrapper! {
    pub struct BuildProject(ObjectSubclass<imp::BuildProject>);
}


impl BuildProject {
    const PKGBUILD:&'static str = "PKGBUILD";
    const BUILD_DIR:&'static str = "build";
    const SRC_DIR:&'static str = "src";
    pub const SUDO_ASKPASS:&'static str = "SUDO_ASKPASS";

    pub fn changeDateOf(&self, subDir: &str) -> String {
        let boxed = self.path();
        let path: Ref<DirEntry> = boxed.borrow();
        let mut buildDirBuf = PathBuf::new();
        buildDirBuf.push(path.path());
        buildDirBuf.push(subDir);
        let buildDir = buildDirBuf.as_path();
        let mut strChanged= String::from("");
        if buildDir.is_dir() {
            let metaResult = buildDir.metadata();
            if let Ok(meta) =  metaResult {
                if let Ok(time) = meta.modified() {
                    let datetime:  DateTime::<Local> = time.into();
                    let custom_format = datetime.format("%Y:%m:%d %T");
                    strChanged = custom_format.to_string();
                }
            }
        }
        return strChanged;
    }
    pub fn diffDate(&self, subDir1: &str, subDir2: &str) -> String {
        let boxed = self.path();
        let path: Ref<DirEntry> = boxed.borrow();
        let mut buildDirBuf1 = PathBuf::new();
        buildDirBuf1.push(path.path());
        buildDirBuf1.push(subDir1);
        let buildDir1 = buildDirBuf1.as_path();
        let mut buildDirBuf2 = PathBuf::new();
        buildDirBuf2.push(path.path());
        buildDirBuf2.push(subDir2);
        let buildDir2 = buildDirBuf2.as_path();
        let mut strDiff = String::from("");
        if buildDir1.is_dir() && buildDir2.is_dir() {
            let metaResult1 = buildDir1.metadata();
            let metaResult2 = buildDir2.metadata();
            if metaResult1.is_ok() && metaResult2.is_ok() {
                let meta1 = metaResult1.unwrap();
                let meta2 = metaResult2.unwrap();
                if let Ok(time1) = meta1.modified()
                    && let Ok(time2) = meta2.modified() {
                    let datetime1: DateTime::<Local> = time1.into();
                    let datetime2: DateTime::<Local> = time2.into();
                    let mut diff = datetime2 - datetime1;   // use positive for ahead of time
                    diff = diff.add( -chrono::Duration::nanoseconds(diff.num_nanoseconds().unwrap() % 1000000000)); // eliminate nanos
                    if diff.num_seconds() < 0 {
                        diff = -diff;
                        strDiff.push_str("-");
                    }
                    let d = diff.num_days();
                    if d > 0 {
                        strDiff = strDiff + &format!("{}d ", d);
                        diff = diff.add( -chrono::Duration::days(d));
                    }
                    let h = diff.num_hours();
                    diff = diff.add( -chrono::Duration::hours(h));
                    let m = diff.num_minutes();
                    diff = diff.add( -chrono::Duration::minutes(m));
                    let s = diff.num_seconds();
                    let hms = format!("{}:{}:{}", h, m, s);
                    strDiff = strDiff + &hms;
                }
            }
        }
        return strDiff;
    }
    pub fn getCsrcPkgbuild(&self) -> Box<Path> {
        let boxed = self.path();
        let path: Ref<DirEntry> = boxed.borrow();
        let pathPath = path.path();
        let csrcDir = pathPath.parent().unwrap();
        let mut csrcPkgBuildBuf:PathBuf = PathBuf::new();
        csrcPkgBuildBuf.push(csrcDir);
        let pkgBuildName:String;
        pkgBuildName = self.name() + BuildProject::PKGBUILD;
        csrcPkgBuildBuf.push(pkgBuildName);
        return csrcPkgBuildBuf.into_boxed_path();
    }
    pub fn getBuildDir(&self) -> Box<Path> {
        let boxed = self.path();
        let path: Ref<DirEntry> = boxed.borrow();
        let mut buildDirBuf:PathBuf = PathBuf::new();
        buildDirBuf.push(path.path());
        buildDirBuf.push(BuildProject::BUILD_DIR);
        return buildDirBuf.into_boxed_path();
    }
    pub fn getBuildPkgbuild(&self) -> Box<Path> {
        let buildDirBuf = self.getBuildDir();
        let mut buildPkgbuildBuf = PathBuf::new();
        buildPkgbuildBuf.push(buildDirBuf);
        buildPkgbuildBuf.push(BuildProject::PKGBUILD);
        return buildPkgbuildBuf.into_boxed_path();
    }
    pub fn readPackageName(&self) -> Option<OsString> {
        let srcPkgbuild = self.getCsrcPkgbuild();
        for line in read_to_string(srcPkgbuild.as_os_str()).unwrap().lines() {
            if !line.starts_with("#") {
                if let Some((key, val)) = line.split_once("=") {
                    if key == "pkgname" {
                        return Some(OsString::from(val));
                    }
                }
            }
        }
        None
    }
    pub fn buildProj(&self) -> Result<Vec<Box<dyn BuildProcessTrait>>,String>  {
        println!("Started building {}", self.name());
        let csrcPkgBuild = self.getCsrcPkgbuild();
        if !csrcPkgBuild.is_file() {
            let err = format!("The pkgbuild script {} was not found", csrcPkgBuild.as_os_str().display());
            return Err(err);
        }
        let buildDir = self.getBuildDir().clone();
        if !buildDir.is_dir() {
            let err = format!("The build dir {} was not found or is not a directory", buildDir.as_os_str().display());
            return Err(err);
        }
        let buildPkgbuild: Box<Path> = self.getBuildPkgbuild();
        if !buildPkgbuild.is_symlink() {
            let linkErr = fs::symlink(csrcPkgBuild, buildPkgbuild.as_ref());
            if linkErr.is_err() {
                let err = format!("The link {} was not created", buildPkgbuild.as_os_str().display());
                return Err(err);
            }
        }
        let boxRepo = self.repo();
        let refRepo:Ref<OsString> = boxRepo.borrow();
        let repo:OsString = refRepo.to_os_string();
        let mut tasks : Vec<Box<dyn BuildProcessTrait>> = Vec::new();
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
        let boxAskpass = self.askpass();
        let refAskpass: Ref<OsString> = boxAskpass.borrow();
        let askPass= OsString::from(refAskpass.as_os_str());
        proc_install.setBuildDir(buildDir.as_os_str());
        if let None = std::env::var_os(BuildProject::SUDO_ASKPASS) {
            if askPass.is_empty() {
                println!("A existing environment var SUDO_ASKPASS was not found, and the setting ~/.config/pyBuild.conf Askpass=path_askpass_executable was missing as well, this might work if the user can use sudo directly (not recommended)");
            }
            else {
                proc_install.setAskpass(askPass.as_os_str());   // use config if not preconfigured
                proc_install.addArg(OsStr::new("--askpass"));
            }
        }
        else {  // if env exists use it
            proc_install.addArg(OsStr::new("--askpass"));
        }
        proc_install.addArg(OsStr::new("pacman"));
        proc_install.addArg(OsStr::new("--noconfirm"));
        proc_install.addArg(OsStr::new("--sync"));
        proc_install.addArg(OsStr::new("--refresh"));
        let mut packName =
            if let Some(packname) = self.readPackageName() {
                packname
            }
            else {
                OsString::from(self.name().as_str())
            };
        proc_install.addArg(packName.as_os_str());
        packName.push("-debug");
        proc_install.addArg(packName.as_os_str());
        tasks.push(proc_install);
        return Ok(tasks);
    }
    pub fn new(name: String, path: std::fs::DirEntry, repo: &OsStr, askpass: &OsStr) -> Self {
        let pathBoxed= BoxedAnyObject::new(path);
        let repoBoxed = BoxedAnyObject::new(repo.to_os_string());
        let askpassBoxed = BoxedAnyObject::new(askpass.to_os_string());
        let obj : BuildProject = Object::builder()
            .property("name", name)
            .property("path", pathBoxed)
            .property("repo", repoBoxed)
            .property("askpass", askpassBoxed)
            .build();
        let buildChanged = obj.changeDateOf(BuildProject::BUILD_DIR);
        obj.set_property("buildChanged", buildChanged);
        let srcChanged = obj.changeDateOf(BuildProject::SRC_DIR);
        obj.set_property("srcChanged", srcChanged);
        let diff = obj.diffDate(BuildProject::SRC_DIR, BuildProject::BUILD_DIR);
        obj.set_property("diff", diff);
        return obj;
    }
}
// ANCHOR_END: glib_wrapper_and_new


// ANCHOR: ProjData
//#[derive(Default)]
impl Default for ProjData {
    fn default() -> ProjData {
        ProjData {
            name: String::new(),
            buildChanged: String::new(),
            srcChanged: String::new(),
            diff: String::new(),
            path: BoxedAnyObject::new(None::<DirEntry>),
            repo: BoxedAnyObject::new(None::<OsString>),
            askpass: BoxedAnyObject::new(None::<OsString>),
        }
    }
}


pub struct ProjData {
    pub name: String,
    pub buildChanged: String,
    pub srcChanged: String,
    pub diff: String,
    pub path: BoxedAnyObject,
    pub repo: BoxedAnyObject,
    pub askpass: BoxedAnyObject,
}
// ANCHOR: ProjData
