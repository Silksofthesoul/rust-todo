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
    params: HashMap<String, String>,
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
    #[serde(skip_serializing, skip_deserializing)]
    todoMode: i32, // 0: show all, 1: show done, 2: show undone
    #[serde(skip_serializing, skip_deserializing)]
    todoIndexes: Vec<i32>, // -1: show all / default, value greater then -1 -- items for show
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
            todoMode: 0,
            todoIndexes: vec![-1],
        }
    }

    pub fn resetTodoMode(&mut self) -> &mut Self {
        self.todoMode = 0;
        self
    }

    pub fn setTodoMode(&mut self, mode: i32) -> &mut Self {
        self.todoMode = mode;
        self
    }

    pub fn resetTodoIndexes(&mut self) -> &mut Self {
        self.todoIndexes = vec![-1];
        self
    }

    pub fn setTodoIndexes(&mut self, indexes: Vec<i32>) -> &mut Self {
        self.todoIndexes = indexes;
        self
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
        self
    }

    pub fn showPropery(&mut self, key: String) {
        //let val = self.properties.get(&key.clone()).unwrap().to_string();
        let renderer = &mut self.renderer;
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
        let mut tsProps: TString = TString::new(String::from("properties"));
        tsProps.setAnsi(TStringStatic::getForeground("lightGray"));
        let mut tsValue: TString = TString::new(String::from("value"));
        tsValue.setAnsi(TStringStatic::getForeground("lightGray"));
        if val.clone() == undefined.clone() {
            for (key, value) in &self.properties {
                let mut tsKey: TString = TString::new(format!("{}{}", key.clone(), ":"));
                tsKey.setAnsi(TStringStatic::getForeground("white"));
                let mut tsVal: TString = TString::new(value.clone());
                tsVal.setAnsi(TStringStatic::getForeground("yellow"));
                renderer.setRow(vec![tsKey.clone(), tsVal.clone()]);
            }
        } else {
            let displayKey = (|| {
                if isNotExist {
                    format!("{} ( not exist )", key.clone())
                } else {
                    key.clone().to_string()
                }
            })();
            let mut tsKey: TString = TString::new(format!("{}{}", displayKey.clone(), ":"));
            tsKey.setAnsi(TStringStatic::getForeground("white"));
            let mut tsVal: TString = TString::new(val.clone());
            tsVal.setAnsi(TStringStatic::getForeground("yellow"));
            renderer.setRow(vec![tsKey.clone(), tsVal.clone()]);
        }

        renderer
            .setHeader(vec![tsProps.clone(), tsValue.clone()])
            .adaptColumnLengths()
            .render()
            .flush();
    }

    pub fn addTask(&mut self, title: &str) -> &mut Self {
        let todoItem = TodoItem {
            status: false,
            title: String::from(title),
            created: String::from("0"),
            ended: String::from(""),
            params: HashMap::new(),
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

        if self.todoIndexes.contains(&(-1 as i32)) {
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
        } else {
            for (index, val) in items.iter().enumerate() {
                let number: &mut TString = &mut TString::new(index.to_string());
                number.setAnsi(TStringStatic::getForeground("lightGray"));
                match self.todoIndexes.contains(&(index as i32)) {
                    true => {
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
                    false => {
                        continue;
                    }
                }
            }
        }

        renderer.adaptColumnLengths().render().flush();

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

        let fileNameDB: String = self
            .settings
            .get(String::from("fileNameDB"))
            .unwrap()
            .to_string();

        let tmplEmpty: String = self
            .settings
            .template(String::from("emptyDB"))
            .unwrap()
            .to_string();

        let content = self
            .fileWorker
            .fileToString(fileNameDB.clone(), tmplEmpty.clone())
            .unwrap();
        self.properties = Self::initProperties(content.clone());
        self.items = Self::initItems(content.clone());
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
            TString::new(String::from("rm or delete or del")),
            TString::new(String::from("Delete a task")),
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

    pub fn rmTaskByIndex(&mut self, indexes: Vec<String>) -> &mut Self {
        let mut vi32ItemsForDelete: Vec<i32> = Vec::new();
        for index in indexes {
            let indexVal: usize = index.parse().unwrap();
            if indexVal > self.items.len() {
                println!("index out of range");
                continue;
            } else {
                vi32ItemsForDelete.push(indexVal as i32);
                let mut item = self.items[indexVal].clone();
                item.params
                    .insert(String::from("deleted"), String::from("true"));
                self.items[indexVal] = item;
            }
        }
        println!("\nThe next items is deleted:");
        self.setTodoIndexes(vi32ItemsForDelete)
            .show()
            .resetTodoIndexes();
        self.items
            .retain(|item| !item.params.contains_key(&String::from("deleted")));
        println!("\n\n");
        self
    }

    pub fn run(&mut self) -> &mut Self {
        let scannerRef = &self.scanner;
        match scannerRef.command.as_str() {
            "log" => self.show(),
            "show" => self.show(),
            "ls" => self.show(),
            "add" => self
                .addTask(scannerRef.param.clone().as_str())
                .sync()
                .show(),
            "rm" => self.rmTaskByIndex(scannerRef.params.clone()).sync().show(),
            "delete" => self.rmTaskByIndex(scannerRef.params.clone()).sync().show(),
            "del" => self.rmTaskByIndex(scannerRef.params.clone()).sync().show(),
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
                self.showPropery(scannerRef.param.clone().to_string());
                self
            }
            "set" => {
                self.setPropery(
                    scannerRef.param.clone().to_string(),
                    scannerRef.params.clone(),
                )
                .sync()
                .showPropery("".to_string());
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
