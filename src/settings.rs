#![allow(non_snake_case)]

use std::collections::HashMap;

#[derive(Clone)]
pub struct Settings {
    settings: HashMap<String, String>,
    templates: HashMap<String, String>,
}

impl Settings {
    pub fn new() -> Settings {
        let mut settings: HashMap<String, String> = HashMap::new();
        settings.insert(String::from("fileNameDB"), String::from("todo.json"));

        let mut templates: HashMap<String, String> = HashMap::new();
        templates.insert(
            String::from("emptyDB"),
            String::from(r#"{ "properties": { "test": "test-val" }, "items": [] }"#),
        );

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
