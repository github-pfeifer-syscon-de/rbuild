#![allow(non_snake_case)]
use glib::property::PropertySet;
mod imp;

use crate::proc::ShowEnum;
use crate::proj::BuildProject;
use glib::{clone, ControlFlow, KeyFile, KeyFileFlags, Object};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib, gdk::Display, Application, CssProvider, Inscription, SingleSelection, TextTag};
use std::cell::RefCell;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::time::Duration;
use std::{env, ffi::OsStr, ffi::OsString, fs};

// ANCHOR: glib_wrapper
glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}
// ANCHOR_END: glib_wrapper


impl Window {
    pub fn new(app: &Application) -> Self {
        // Create new window
        Object::builder().property("application", app).build()
    }

    // ANCHOR: projects
    fn projects(&self) -> gio::ListStore {
        // Get projects
        self.imp()
            .projects
            .borrow()
            .clone()
            .expect("Could not get current projects.")
    }
    // ANCHOR_END: projects
    fn getConfig(&self) -> KeyFile {
        let mut refKeyFile = self.imp().keyFile.borrow_mut();
        if refKeyFile.is_none() {
            let confDir:OsString =
            match env::var_os("XDG_CONFIG_HOME") {
                Some(path) => { path},
                None => {
                    println ! ("Not set XDG_CONFIG_HOME");
                    match env::home_dir() {
                        Some(home) => {
                            let mut path: OsString;
                            path = home.as_os_str().to_os_string();
                            path.push("/.config");
                            path
                        },
                        None => {
                            let path = OsString::from( "~/.config");
                            path
                        }
                    }
                }};
            let mut confFile = PathBuf::new();
            confFile.push(confDir);
            confFile.push("pyBuild.conf");
            let confTemp = confFile.clone();
            //println!("Loading config {}", confFile.display());
            let keyFile = KeyFile::new();
            let keyLoad = keyFile.load_from_file( &confFile.into_boxed_path()
                , KeyFileFlags::KEEP_COMMENTS);
            if let Err(err) = keyLoad {
                println!("The confFile {} was not loaded {}", confTemp.display(), err);
                let mut path: OsString = match env::home_dir() {
                    Some(home) => {
                        home.into_os_string()
                    },
                    None => {
                        OsString::from("[replace with parent for .config e.g. your home dir]")
                    }
                };
                path.push("/csrc.git");
                keyFile.set_string("Main", "BuildDir", path.to_str().unwrap());
                keyFile.set_string("Main", "Repo", "/var/local/pacman/custom.db.tar.gz");
                let mut askPass = OsString::new();
                askPass.push(path.to_str().unwrap());
                askPass.push("/.cargo/bin/raskpass");   // requires preparation in rbuild with cargo install --path . --examples
                keyFile.set_string("Main", "Askpass", askPass.to_str().unwrap());
                keyFile.set_comment(Some("Main"), Some("Askpass"), "Alternative provide SUDO_ASKPASS in user environment, if left empty presume askpass not needed...").unwrap();
                if !confTemp.exists() {  // only save if not existing
                    if let Err(err) = keyFile.save_to_file(&confTemp.into_boxed_path()) {
                        println!("Error {} saving default config", err);
                    }
                    else {
                        println!("Created config with defaults -> check");
                    }
                }
            }
            refKeyFile.replace(keyFile);
        }
        if let Some(keyFile) = refKeyFile.deref() {
            return keyFile.clone();
        }
        panic!("this is not expected to happen when loading the config file!");
    }
    fn list_proj(&self) -> Vec<BuildProject> {
        let keyFile = self.getConfig();
        let confVal = keyFile.string("Main", "BuildDir")
            .expect("The group main and key BuildDir not found.");
        let buildDir:&OsStr = confVal.as_ref();
        let repo = keyFile.string("Main", "Repo")
            .expect("The group main and key Repo not found.");
        let osRepo: OsString = repo.into();
        let mut osAskpass = OsString::new();
        if let Ok(askpass) = keyFile.string("Main", "Askpass") {
            osAskpass = askpass.into();
        }
        let mut vector = Vec::new();
        println!("Config for path {}", buildDir.display());
        let buildPath = Path::new(&buildDir);
        let paths = fs::read_dir(buildPath).unwrap();
        for errPath in paths {
            if let Ok(path) = errPath {
                let fileName = path.file_name();
                let fileString = fileName.into_string().unwrap();
                if !fileString.starts_with('.')
                 && path.file_type().unwrap().is_dir() {
                    //println!("Name: {}", fileString);
                    let proj1 = BuildProject::new(fileString, path, osRepo.as_os_str(), osAskpass.as_os_str());
                    vector.push(proj1);
                }
            }
            else  {
                let err = errPath.unwrap_err();
                println!("Error {} the path was not usable.", err);
            }
        }
        return vector;
    }

    fn setup_build(&self) {
        //Create new model
        let model = gio::ListStore::new::<BuildProject>();

        // Add the vector to the model
        let vector= self.list_proj();
        model.extend_from_slice(&vector);
        //let proj = ProjObject::new(String::from("abc"), String::from("def"));
        //model.append(&proj);

        //println!("setup_build model size {}.", model.n_items());
        // Get state and set model
        self.imp().projects.replace(Some(model));

        // Wrap model with selection and pass it to the list view
        let selection_model = SingleSelection::new(Some(self.projects()));
        self.imp().table.set_model(Some(&selection_model));
    }

    // ANCHOR: factory_setup
    fn setup_table(&self) {
        // The CSS "magic" happens here.
        let provider = CssProvider::new();
        provider.load_from_string(include_str!("style.css"));
        // We give the CssProvided to the default screen so the CSS rules we added
        // can be applied to our window.
        gtk::style_context_add_provider_for_display(
            &Display::default()
                .expect("Could not connect to a display.")
            ,&provider
            ,gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
        let colNameFactory = gtk::SignalListItemFactory::new();
        colNameFactory.connect_setup(move |_factory, item| {
            let item = item.downcast_ref::<gtk::ListItem>().unwrap();
            //let row = GridCell::default();   // use this for a customizable cell with multiple nested widgets
            //item.set_child(Some(&row));
            let cell = Inscription::new(None);
            item.set_child(Some(&cell));
        });
        colNameFactory.connect_bind(move |_factory, item| {
            let item = item.downcast_ref::<gtk::ListItem>().unwrap();
            //let child = item.child().and_downcast::<GridCell>().unwrap();
            let r = item.item().and_downcast::<BuildProject>().unwrap();
            //println!("connect_bind 1 {}.", child.widget_name());
            //let ent = Entry {
            //    name: r.name().to_string(),
            //};
            //child.set_entry(&ent);
            let child = item.child().and_downcast::<Inscription>().unwrap();
            child.set_text(Some(&*r.name()));
        });
        let colName = gtk::ColumnViewColumn::new(Some("Name"), Some(colNameFactory));
        colName.set_expand(true);
        self.imp().table.append_column(&colName);

        let colBuildchgFactory = gtk::SignalListItemFactory::new();
        colBuildchgFactory.connect_setup(move |_factory, item| {
            let item = item.downcast_ref::<gtk::ListItem>().unwrap();
            //let row = GridCell::default();
            //item.set_child(Some(&row));
            let cell = Inscription::new(Some(""));
            item.set_child(Some(&cell));
        });
        colBuildchgFactory.connect_bind(move |_factory, item| {
            let item = item.downcast_ref::<gtk::ListItem>().unwrap();
            let child = item.child().and_downcast::<Inscription>().unwrap();
            let buildProject = item.item().and_downcast::<BuildProject>().unwrap();
            child.set_text(Some(&*buildProject.buildChanged()));
            //let p: Option<std::fs::DirEntry> = &*r.path();
            //child.set_text(p.expect("not filled").file_name().into_string());
        });
        let colBuildchg = gtk::ColumnViewColumn::new(Some("Build changed"), Some(colBuildchgFactory));
        colBuildchg.set_expand(true);
        self.imp().table.append_column(&colBuildchg);

        let colSrcchgFactory = gtk::SignalListItemFactory::new();
        colSrcchgFactory.connect_setup(move |_factory, item| {
            let item = item.downcast_ref::<gtk::ListItem>().unwrap();
            //let row = GridCell::default();
            //item.set_child(Some(&row));
            let cell = Inscription::new(Some(""));
            item.set_child(Some(&cell));
        });
        colSrcchgFactory.connect_bind(move |_factory, item| {
            let item = item.downcast_ref::<gtk::ListItem>().unwrap();
            let child = item.child().and_downcast::<Inscription>().unwrap();
            let r: BuildProject = item.item().and_downcast::<BuildProject>().unwrap();
            child.set_text(Some(&*r.srcChanged()));
            //let p: Option<std::fs::DirEntry> = &*r.path();
            //child.set_text(p.expect("not filled").file_name().into_string());
        });
        let colSrcchg = gtk::ColumnViewColumn::new(Some("Src changed"), Some(colSrcchgFactory));
        colSrcchg.set_expand(true);
        self.imp().table.append_column(&colSrcchg);

        let colDiffFactory = gtk::SignalListItemFactory::new();
        colDiffFactory.connect_setup(move |_factory, item| {
            let item = item.downcast_ref::<gtk::ListItem>().unwrap();
            //let row = GridCell::default();
            //item.set_child(Some(&row));
            let cell = Inscription::new(Some(""));
            item.set_child(Some(&cell));
        });
        colDiffFactory.connect_bind(move |_factory, item| {
            let item = item.downcast_ref::<gtk::ListItem>().unwrap();
            let child = item.child().and_downcast::<Inscription>().unwrap();
            let r: BuildProject = item.item().and_downcast::<BuildProject>().unwrap();
            let diff = &*r.diff();
            child.set_text(Some(diff));
            child.add_css_class(
                match !diff.starts_with("-") {
                    true  => "positive"
                  , false =>"negative"});
        });
        let colDiff = gtk::ColumnViewColumn::new(Some("Diff"), Some(colDiffFactory));
        colDiff.set_expand(true);
        self.imp().table.append_column(&colDiff);
    }

    fn setup_text(&self) {
        let text = self.imp().text.get();
        let buff = text.buffer();
        let _warnTag = self.imp().textTagWarn.set(
            buff.create_tag(Some("warnTag"),
                            &[("foreground", &"orange")]));
        let _setGood = self.imp().textTagGood.set(
            buff.create_tag(Some("goodTag"),
                            &[("foreground", &"green")]));
        let _setFail = self.imp().textTagFail.set(
            buff.create_tag(Some("failTag"),
                            &[("foreground", &"red")]));
        //let warnTtb =  TextTagBuilder.new()
        //    .name("failTag")
        //    .foreground_rgba(&RGBA::new(1.0_f32, 0.0_f32, 0.0_f32, 1.0_f32))
        //    .build();
    }
    pub fn showErr(&self, msg: &String, showKind: ShowEnum) {
        let text = &self.imp().text.get();
        let buff = text.buffer();
        let textTag:&RefCell<Option<TextTag>> =
            match showKind {
                ShowEnum::Normal => &RefCell::new(None::<TextTag>),
                ShowEnum::Warn => &self.imp().textTagWarn,
                ShowEnum::Good => &self.imp().textTagGood,
                ShowEnum::Error => &self.imp().textTagFail,
            };
        let mut end =  buff.end_iter();
        if let Ok(refTag) = textTag.try_borrow() &&
            let Some(tag) = refTag.deref() {
            buff.insert_with_tags(&mut end, msg, &[&tag]);
        }
        else {
             buff.insert(&mut end, &msg);
        }
        let end = &mut buff.end_iter();  // does move
        text.scroll_to_iter(end, 0.0, false, 0.0, 1.0);
    }

    // log with date
    // pub fn showErr(&self, text_view: & TextView, err:&String) {
    //     let datetime  = Local::now();
    //     let custom_format = datetime.format("%Y:%m:%d %T");
    //     let mut str = custom_format.to_string();
    //     str = str + " " + err;
    //     let mut buffer = text_view.buffer();
    //     let _errWr = buffer.write_str(&str);
    //     let end = &mut buffer.end_iter();  // does move, but leaves out the last bits
    //     text_view.scroll_to_iter(end, 0.0, false, 0.0, 0.9);
    // }

    // this takes a task from the queue and executes it
    //   the output is watched by a timer and appears in the text view
    fn runTask(&self) {
        let mut tasks = self.imp().tasks.borrow_mut();
        if let Some(qtask) = tasks.pop_front() {
            let task= qtask.deref();
            //println!("Task {} starting", task.cmd().display());
            let runner = task.run();
            if let Ok(running) = runner {
                let build = self.imp().build.get();
                build.set_sensitive(false);
                //let text = self.imp().text.get();
                // check for result with timer, as we need to handle components on event-thread
                glib::timeout_add_local(Duration::from_millis(100), clone!(
                     #[weak(rename_to = window)] self,
                     #[upgrade_or] ControlFlow::Break,  // hack allow closure return value (is there sth. better ?)
                     move || -> ControlFlow {
                     if let Some(msg) = running.read(
                            |str, show_enum| {
                                window.showErr(str, show_enum);
                            }) {
                         if let Ok(ok) = msg {
                            println!("Task succeeded {}", ok);
                            //window.showErr(&msg, ShowEnum::Good);
                            window.runTask();
                         } 
                         else {
                            println!("Task failed {}", msg.unwrap_err());
                            //window.showErr(&msg, ShowEnum::Error);
                         }
                         build.set_sensitive(true);
                         return ControlFlow::Break;
                     }
                     return ControlFlow::Continue;
                }));
            }
            else if let Err(msg) = runner {
                println!("Error {}", msg);
                self.showErr(&msg, ShowEnum::Error);
            }
        }
    }
    fn build(& self) {
        let model= self.imp().table.model();
        let singleModel:SingleSelection = model.and_downcast::<SingleSelection>().unwrap();
        if let Some(selObj) = singleModel.selected_item() {
            let projObj = selObj.dynamic_cast::<BuildProject>().unwrap();
            let build = projObj.buildProj();
            if let Ok(tasks) = build {
                {   // important to avoid double borrow for tasks
                    let mut taskRef = self.imp().tasks.borrow_mut();
                    //tasks.splice(0.., taskRef.back().unwrap()); more efficent
                    for  task in tasks {
                        let _res = taskRef.push_back(task);
                    }
                }
                self.runTask();
            }
            else if let Err(err) = build {
                println!("Error {} starting", err);
                self.showErr(&err, ShowEnum::Error);
            }
        }
    }
    // ANCHOR: setup_callbacks
    fn setup_callbacks(&self) {
        self.imp().build.connect_clicked(clone!(
             #[weak(rename_to = window)]
             self,
             move |_| {
                 window.build();
             }
         ));
    }
    // ANCHOR_END: setup_callbacks


    // ANCHOR: setup_factory
    fn setup_factory(&self) {
        // Create a new factory
        //let factory = SignalListItemFactory::new();

        // Create an empty `TaskRow` during setup
        //factory.connect_setup(move |_, list_item| {
        //    // Create `TaskRow`
        //    let task_row = TaskRow::new();
        //    list_item
        //        .downcast_ref::<ListItem>()
        //        .expect("Needs to be ListItem")
        //        .set_child(Some(&task_row));
        //});

        // Tell factory how to bind `TaskRow` to a `TaskObject`
        // factory.connect_bind(move |_, list_item| {
        //     // Get `TaskObject` from `ListItem`
        //     let task_object = list_item
        //         .downcast_ref::<ListItem>()
        //         .expect("Needs to be ListItem")
        //         .item()
        //         .and_downcast::<TaskObject>()
        //         .expect("The item has to be an `TaskObject`.");
        //
        //     // Get `TaskRow` from `ListItem`
        //     let task_row = list_item
        //         .downcast_ref::<ListItem>()
        //         .expect("Needs to be ListItem")
        //         .child()
        //         .and_downcast::<TaskRow>()
        //         .expect("The child has to be a `TaskRow`.");
        //
        //     task_row.bind(&task_object);
        // });
        //
        // // Tell factory how to unbind `TaskRow` from `TaskObject`
        // factory.connect_unbind(move |_, list_item| {
        //     // Get `TaskRow` from `ListItem`
        //     let task_row = list_item
        //         .downcast_ref::<ListItem>()
        //         .expect("Needs to be ListItem")
        //         .child()
        //         .and_downcast::<TaskRow>()
        //         .expect("The child has to be a `TaskRow`.");
        //
        //     task_row.unbind();
        // });
        //
        // // Set the factory of the list view
        // self.imp().table.set_factory(Some(&factory));
    }
    // ANCHOR_END: setup_factory
}