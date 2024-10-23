#![allow(non_snake_case)]
use directories::UserDirs;
use include_dir::{include_dir, Dir};
use std::env;
use std::path::Path;
use std::path::PathBuf;

use std::collections::HashMap;

fn printTypeOf<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

#[derive(Clone)]
pub struct Settings {
    settings: HashMap<String, String>,
    templates: HashMap<String, String>,
}

impl Settings {
    pub fn new() -> Settings {
        static PROJECT_DIR: Dir = include_dir!("src/static");

        // home directory path and file name
        let appFileName: String = String::from("todo.exe");
        let mut strAppDir: String = String::new();
        let mut strAppFullPath: String = String::new();
        if let Some(user_dirs) = UserDirs::new() {
            let mut homeDir = user_dirs.home_dir().to_path_buf();
            homeDir.push(".bin");
            strAppDir = homeDir.to_string_lossy().into_owned();
            homeDir.push(&appFileName);
            strAppFullPath = homeDir.to_string_lossy().into_owned();
        }

        // the platform
        let platform = if cfg!(target_os = "windows") {
            "windows"
        } else if cfg!(target_os = "macos") {
            "macos"
        } else if cfg!(target_os = "linux") {
            "linux"
        } else {
            "unknown"
        };

        //current exec
        let mut execPath: String = String::new();
        match env::current_exe() {
            Ok(exe_path) => execPath = exe_path.display().to_string(),
            Err(e) => execPath = String::from("%unknown%"),
        }

        let mut settings: HashMap<String, String> = HashMap::new();
        settings.insert(String::from("fileNameDB"), String::from("todo.json"));
        settings.insert(String::from("mdFile"), String::from("todo.md"));
        settings.insert(String::from("htmlFile"), String::from("todo.html"));
        settings.insert(String::from("app-name"), String::from("ToDo"));
        settings.insert(String::from("platform"), String::from(platform));
        settings.insert(String::from("appFileName"), String::from(appFileName));
        settings.insert(String::from("appFullPath"), String::from(strAppFullPath));
        settings.insert(String::from("execPath"), String::from(execPath));
        settings.insert(
            String::from("appDir"),
            String::from(String::from(strAppDir)),
        );
        settings.insert(
            String::from("app-version"),
            String::from(env!("CARGO_PKG_VERSION")),
        );

        let mut templates: HashMap<String, String> = HashMap::new();

        let txtDescription = PROJECT_DIR
            .get_file("description.txt")
            .unwrap()
            .contents_utf8()
            .unwrap();
        settings.insert(String::from("description"), String::from(txtDescription));

        let jsonTemplate = PROJECT_DIR
            .get_file("todo.template.json")
            .unwrap()
            .contents_utf8()
            .unwrap();
        templates.insert(String::from("emptyDB"), String::from(jsonTemplate));

        let htmlTemplate = PROJECT_DIR
            .get_file("todo.template.html")
            .unwrap()
            .contents_utf8()
            .unwrap();
        templates.insert(String::from("todoHTML"), String::from(htmlTemplate));

        let cssStyles = PROJECT_DIR
            .get_file("styles.css")
            .unwrap()
            .contents_utf8()
            .unwrap();
        templates.insert(String::from("cssStyles"), String::from(cssStyles));

        Settings {
            settings,
            templates,
        }
    }

    pub fn get(&self, key: String) -> Option<&String> {
        self.settings.get(&key)
    }
    pub fn template(&self, key: String) -> Option<&String> {
        self.templates.get(&key)
    }
}
