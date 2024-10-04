#![allow(non_snake_case)]
use include_dir::{include_dir, Dir};

use std::collections::HashMap;

#[derive(Clone)]
pub struct Settings {
    settings: HashMap<String, String>,
    templates: HashMap<String, String>,
}

impl Settings {
    pub fn new() -> Settings {
        static PROJECT_DIR: Dir = include_dir!("src/static");

        let mut settings: HashMap<String, String> = HashMap::new();
        settings.insert(String::from("fileNameDB"), String::from("todo.json"));
        settings.insert(String::from("mdFile"), String::from("todo.md"));
        settings.insert(String::from("htmlFile"), String::from("todo.html"));
        settings.insert(String::from("app-name"), String::from("ToDo"));
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
