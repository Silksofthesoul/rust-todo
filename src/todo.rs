#![allow(non_snake_case)]

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

#[derive(Serialize, Deserialize, Debug)]
struct Properties {
    ownerName: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoItem {
    status: bool,
    title: String,
    created: String,
    ended: String,
}

#[derive(Deserialize, Debug)]
pub struct TodoJSON {
    properties: Properties,
    items: Vec<TodoItem>,
}

#[derive(Serialize)]
pub struct Todo {
    properties: Properties,
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

    fn initProperties(data: String) -> Properties {
        let parsed: Result<TodoJSON> = parse(data);
        parsed.unwrap().properties
    }

    fn initItems(data: String) -> Vec<TodoItem> {
        let parsed: Result<TodoJSON> = parse(data);
        parsed.unwrap().items
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

    pub fn show(&self) -> &Self {
        println!("\n\n");
        line();
        let json = serde_json::to_value(&self.properties).unwrap();
        println!("properties");
        if let Value::Object(map) = json {
            for (key, value) in map {
                println!("{}:\t{}", key, value);
            }
        }
        println!("items:");
        for (index, val) in self.items.iter().enumerate() {
            let status = if val.status { "[x]" } else { "[ ]" };
            let title = &val.title;
            let created = &val.created;
            let ended = &val.ended;
            println!(
                "  {} \t{} \t{} \t{} \t{}",
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
            &_ => self,
        }
    }
}
