#![allow(non_snake_case)]
use directories::UserDirs;
use include_dir::{include_dir, Dir};
use std::env;
use std::path::Path;

use std::collections::HashMap;

fn print_type_of<T>(_: &T) {
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
        let mut homeDir: &Path = Path::new("");
        if let Some(user_dirs) = UserDirs::new() {
            homeDir = user_dirs.home_dir().;
        }
        println!("homeDir   {:?}", homeDir);
        // let mut appDir: PathBuf = match env::home_dir() {
        //     Some(path) => path,
        //     None => PathBuf::from("/"),
        // };
        //
        // appDir.push(".bin");
        // let appDirOSString = appDir.into_os_string();
        // println!("userDirs  {:?}", userDirs);
        // println!("homeDir   {:?}", homeDir);
        // let appDirString = appDirOSString.to_str().unwrap();

        let mut settings: HashMap<String, String> = HashMap::new();
        settings.insert(String::from("fileNameDB"), String::from("todo.json"));
        settings.insert(String::from("mdFile"), String::from("todo.md"));
        settings.insert(String::from("htmlFile"), String::from("todo.html"));
        settings.insert(String::from("app-name"), String::from("ToDo"));
        settings.insert(
            String::from("appDir"),
            String::from(String::from("appDirString")),
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
