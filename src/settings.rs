#![allow(non_snake_case)]

pub struct Settings {
    pub fileDataBase: String,
    pub template: String
}

impl Settings {
    pub fn new() -> Settings {
        Settings {
            fileDataBase: String::from("todo.json"),
            template: String::from(
r#"{
  properties: {},
  items: []
}"#
            ),
        }
    }
}
