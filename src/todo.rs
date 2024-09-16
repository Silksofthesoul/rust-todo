#![allow(non_snake_case)]

use std::collections::HashMap;

use crate::fileWorker::FileWorker;
use crate::library::json::{parse, stringify};
use crate::scanner::Scanner;
use crate::settings::Settings;
use crate::terminaltablerenderer::terminaltablerenderer::{
    TerminalTableRenderer as TTR, TerminalTableRenderer,
};
use crate::tstring::tstring::{TString, TString as TStringStatic};

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
    #[serde(skip_serializing, skip_deserializing)]
    renderer: TTR,
}

impl Todo {
    pub fn new() -> Todo {
        let renderer = TTR::new();
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
            renderer,
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

    pub fn setPropery(&mut self, key: String, params: Vec<String>) -> &mut Self {
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

    pub fn addTask(&mut self, title: &str) -> &mut Self {
        let todoItem = TodoItem {
            status: false,
            title: String::from(title),
            created: String::from("0"),
            ended: String::from(""),
        };
        self.items.push(todoItem);
        self
    }

    pub fn done(&mut self, index: usize) -> &mut Self {
        let mut isOverflow = false;
        if index > self.items.len() {
            isOverflow = true;
        }
        if !isOverflow {
            self.items[index].status = true;
        }
        self
    }

    pub fn undone(&mut self, index: usize) -> &mut Self {
        let mut isOverflow = false;
        if index > self.items.len() {
            isOverflow = true;
        }
        if !isOverflow {
            self.items[index].status = false;
        }
        self
    }
    pub fn show(&mut self) -> &mut Self {
        let renderer = &mut self.renderer;
        let items = &self.items;
        let mut titleNumber = TString::new(String::from("#"));
        titleNumber.setAnsi(TStringStatic::getForeground("lightGray"));

        renderer.setHeader(vec![
            titleNumber,
            TString::new(String::from("Status")),
            TString::new(String::from("Title")),
            TString::new(String::from("Created")),
            TString::new(String::from("Ended")),
        ]);

        for (index, val) in items.iter().enumerate() {
            let number: &mut TString = &mut TString::new(index.to_string());
            number.setAnsi(TStringStatic::getForeground("lightGray"));
            number.setAnsi(TStringStatic::getForeground("lightGray"));
            let status: TString = if val.status {
                let status = &mut TString::new("[x]".to_string());
                status.setAnsi(TStringStatic::getForeground("green"));
                status.clone()
            } else {
                TString::new("[ ]".to_string())
            };
            let title = if val.status {
                let mut title = TString::new(val.title.to_string());
                title.setAnsi(TStringStatic::getForeground("lightGreen"));
                title
            } else {
                TString::new(val.title.to_string())
            };
            let created = TString::new(val.created.to_string());
            let ended = TString::new(val.ended.to_string());
            renderer.setRow(vec![number.clone(), status, title, created, ended]);
        }

        renderer.adaptColumnLengths().render();

        self
    }

    pub fn sync(&mut self) -> &mut Self {
        let json_string = stringify(&self);
        let fileNameDB: String = self
            .settings
            .get(String::from("fileNameDB"))
            .unwrap()
            .to_string();
        self.fileWorker.write(fileNameDB, json_string).unwrap();
        self
    }

    pub fn help(&mut self) -> &mut Self {
        let mut renderer = &mut self.renderer;
        renderer.setHeader(vec![
            TString::new(String::from("Command")),
            TString::new(String::from("Description")),
        ]);
        renderer.setRow(vec![
            TString::new(String::from("log")),
            TString::new(String::from("Show all tasks")),
        ]);
        renderer.setRow(vec![
            TString::new(String::from("show")),
            TString::new(String::from("Show all tasks")),
        ]);
        renderer.setRow(vec![
            TString::new(String::from("ls")),
            TString::new(String::from("Show all tasks")),
        ]);
        renderer.setRow(vec![
            TString::new(String::from("add")),
            TString::new(String::from("Add a task")),
        ]);
        renderer.setRow(vec![
            TString::new(String::from("push")),
            TString::new(String::from("Add a task")),
        ]);
        renderer.setRow(vec![
            TString::new(String::from("done")),
            TString::new(String::from("Mark a task as done")),
        ]);
        renderer.setRow(vec![
            TString::new(String::from("undone")),
            TString::new(String::from("Mark a task as undone")),
        ]);
        renderer.setRow(vec![
            TString::new(String::from("config")),
            TString::new(String::from("Show a property")),
        ]);
        renderer.setRow(vec![
            TString::new(String::from("set")),
            TString::new(String::from("Set a property")),
        ]);
        renderer.setRow(vec![
            TString::new(String::from("help")),
            TString::new(String::from("Show this help")),
        ]);
        renderer.adaptColumnLengths().render();
        self
    }

    pub fn run(&mut self) -> &mut Self {
        let scannerRef = &self.scanner;
        match scannerRef.command.as_str() {
            "log" => self.show(),
            "show" => self.show(),
            //"show2" => self.show2(),
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
            "help" => {
                self.help();
                self
            }
            &_ => self.show(),
        }
    }
}
