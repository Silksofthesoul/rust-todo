#![allow(non_snake_case)]

use std::collections::HashMap;

use crate::fileWorker::FileWorker;
use crate::library::json::{parse, stringify};
use crate::library::print::line;
use crate::scanner::Scanner;
use crate::settings::Settings;

// use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::Result;
use serde_json::Value;
// use std::collections::HashMap;
// use std::vec::Vec;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TodoItem {
    status: bool,
    title: String,
    created: String,
    ended: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TodoJSON {
    properties: HashMap<String, String>,
    items: Vec<TodoItem>,
}

#[derive(Serialize)]
pub struct Todo {
    properties: HashMap<String, String>,
    items: Vec<TodoItem>,
    #[serde(skip_serializing, skip_deserializing)]
    fileWorker: FileWorker,
    #[serde(skip_serializing, skip_deserializing)]
    settings: Settings,
    #[serde(skip_serializing, skip_deserializing)]
    scanner: Scanner,
}

impl Todo {
    pub fn new() -> Todo {
        let scanner = Scanner::new();
        let settings = Settings::new();
        let fileWorker = FileWorker::new();
        let fileNameDB: String = settings
            .get(String::from("fileNameDB"))
            .unwrap()
            .to_string();
        let tmplEmpty: String = settings
            .template(String::from("emptyDB"))
            .unwrap()
            .to_string();

        let content = fileWorker
            .fileToString(fileNameDB.clone(), tmplEmpty.clone())
            .unwrap();

        Todo {
            properties: Self::initProperties(content.clone()),
            items: Self::initItems(content.clone()),
            fileWorker,
            settings,
            scanner,
        }
    }

    fn initProperties(data: String) -> HashMap<String, String> {
        let parsed: Result<TodoJSON> = parse(data);
        parsed.unwrap().properties
    }

    fn initItems(data: String) -> Vec<TodoItem> {
        let parsed: Result<TodoJSON> = parse(data);
        parsed.unwrap().items
    }

    pub fn setPropery(&mut self, key: String, params: Vec<String>) -> &Self {
        let value: String = params.get(1).unwrap().to_string();
        self.properties.insert(key.clone(), value.clone());
        println!("----{}: {}--------", key.clone(), value.clone());
        self
    }

    pub fn getPropery(&self, key: String) {
        //let val = self.properties.get(&key.clone()).unwrap().to_string();
        let undefined = String::from("undefined");
        let mut isNotExist = false;
        let val = self
            .properties
            .get(&key.clone().to_string())
            .map(|v| v.to_string())
            .unwrap_or_else(|| {
                isNotExist = true;
                undefined.clone()
            });
        if val.clone() == undefined.clone() {
            println!("properties: ");
            for (key, value) in &self.properties {
                println!("{}: {}", key, value);
            }
        } else {
            let displayKey = (|| {
                if isNotExist {
                    format!("{} ( not exist )", key.clone())
                } else {
                    key.clone().to_string()
                }
            })();
            println!("{}: {}", displayKey, val);
        }
    }

    pub fn addTask(&mut self, title: &str) -> &Self {
        let todoItem = TodoItem {
            status: false,
            title: String::from(title),
            created: String::from("0"),
            ended: String::from(""),
        };
        self.items.push(todoItem);
        self
    }

    pub fn done(&mut self, index: usize) -> &Self {
        let mut isOverflow = false;
        if index > self.items.len() {
            isOverflow = true;
        }
        if !isOverflow {
            self.items[index].status = true;
        }
        self
    }

    pub fn undone(&mut self, index: usize) -> &Self {
        let mut isOverflow = false;
        if index > self.items.len() {
            isOverflow = true;
        }
        if !isOverflow {
            self.items[index].status = false;
        }
        self
    }

    pub fn show(&self) -> &Self {
        println!("\n\n");
        line();
        let json = serde_json::to_value(&self.properties).unwrap();
        println!("\x1b[90mproperties:\x1b[0m");
        if let Value::Object(map) = json {
            for (key, value) in map {
                println!("{}:\t\x1b[33m{}\x1b[0m", key, value);
            }
        }
        println!("\x1b[90mitems:\x1b[0m");
        for (index, val) in self.items.iter().enumerate() {
            let status = if val.status {
                "\x1b[32m[x]\x1b[0m"
            } else {
                "[ ]"
            };
            let title = &val.title;
            let created = &val.created;
            let ended = &val.ended;
            println!(
                "  \x1b[90m{}\x1b[0m \t{} \t\x1b[93m{}\x1b[0m \t{} \t{}",
                index, status, title, created, ended
            );
        }
        line();
        &self
    }

    pub fn sync(&self) -> &Self {
        let json_string = stringify(&self);
        let fileNameDB: String = self
            .settings
            .get(String::from("fileNameDB"))
            .unwrap()
            .to_string();
        self.fileWorker.write(fileNameDB, json_string).unwrap();
        &self
    }

    pub fn run(&mut self) -> &Self {
        let scannerRef = &self.scanner;
        match scannerRef.command.as_str() {
            "log" => self.show(),
            "show" => self.show(),
            "ls" => self.show(),
            "add" => self
                .addTask(scannerRef.param.clone().as_str())
                .sync()
                .show(),
            "push" => self
                .addTask(scannerRef.param.clone().as_str())
                .sync()
                .show(),
            "done" => self
                .done(scannerRef.param.clone().parse().unwrap())
                .sync()
                .show(),
            "undone" => self
                .undone(scannerRef.param.clone().parse().unwrap())
                .sync()
                .show(),
            "config" => {
                self.getPropery(scannerRef.param.clone().to_string());
                self
            }
            "set" => {
                self.setPropery(
                    scannerRef.param.clone().to_string(),
                    scannerRef.params.clone(),
                )
                .sync()
                .show();
                self
            }
            &_ => self.show(),
        }
    }
}
