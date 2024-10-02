#![allow(non_snake_case)]

use chrono::{DateTime, Local};
use std::collections::HashMap;

use crate::fileWorker::FileWorker;
use crate::library::json::{parse, stringify};
use crate::markdownrender::markdownrender::MarkdownRender;
use crate::scanner::Scanner;
use crate::settings::Settings;
use crate::terminaltablerenderer::terminaltablerenderer::TerminalTableRenderer as TTR;
use crate::tstring::tstring::{TString, TString as TStringStatic};

use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::Result;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TodoItem {
    pub status: bool,
    pub title: String,
    pub created: String,
    pub ended: String,
    pub edited: String,
    pub params: HashMap<String, String>,
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
    markdownRender: MarkdownRender,
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

        let markdownRender = MarkdownRender::new();
        let now: DateTime<Local> = Local::now();
        let nowStr = now.format("%Y-%m-%d %H:%M:%S").to_string();
        let mut tmpl = tmplEmpty.clone();
        tmpl = tmpl.replace("%now%", nowStr.as_str());

        let content = fileWorker
            .fileToString(fileNameDB.clone(), tmpl.clone())
            .unwrap();

        Todo {
            properties: Self::initProperties(content.clone()),
            items: Self::initItems(content.clone()),
            fileWorker,
            settings,
            scanner,
            renderer,
            markdownRender,
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

    pub fn setPropery(
        &mut self,
        key: String,
        params: Vec<String>,
        skipMarkIsEdited: bool,
    ) -> &mut Self {
        let value: String = params.get(1).unwrap().to_string();
        self.properties.insert(key.clone(), value.clone());
        if skipMarkIsEdited == false {
            self.setupLastEdited();
        }
        self
    }

    pub fn unsetProperty(&mut self, key: String) -> &mut Self {
        let mut isNotExist = false;
        let renderer = &mut self.renderer;
        let undefined = String::from("undefined");
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

        let mut tsKey: TString = TString::new(format!("{}{}", key.clone(), ":"));
        tsKey.setAnsi(TStringStatic::getForeground("white"));
        let mut tsVal: TString = TString::new(val.clone());
        tsVal.setAnsi(TStringStatic::getForeground("yellow"));

        if isNotExist {
            println!("\nThe key \"{}\" was is not found", &key);
            print!("\n\n");
        } else {
            println!("\nThe next item is deleted:");

            renderer
                .setHeader(vec![tsProps.clone(), tsValue.clone()])
                .setRow(vec![tsKey.clone(), tsVal.clone()])
                .adaptColumnLengths()
                .render()
                .flush();
            println!("\n\n");
            self.properties.remove(&key);
            self.setupLastEdited();
        }

        self
    }

    pub fn getProperties(&mut self) -> HashMap<String, String> {
        self.properties.clone()
    }
    pub fn getProperty(&mut self, key: String) -> String {
        let undefined = String::from("");
        let val = self
            .properties
            .get(&key.clone().to_string())
            .map(|v| v.to_string())
            .unwrap_or_else(|| undefined.clone());
        val
    }

    pub fn showProperty(&mut self, key: String) -> &mut Self {
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
        self
    }

    pub fn getItems(&mut self) -> Vec<TodoItem> {
        self.items.clone()
    }

    pub fn addTask(&mut self, title: &str) -> &mut Self {
        let now: DateTime<Local> = Local::now();
        let created: String = now.format("%Y-%m-%d %H:%M:%S").to_string();
        let todoItem = TodoItem {
            status: false,
            title: String::from(title),
            created,
            ended: String::from(""),
            edited: String::from(""),
            params: HashMap::new(),
        };
        self.items.push(todoItem);
        self.setupLastEdited();
        self
    }

    pub fn setupLastEdited(&mut self) -> &mut Self {
        let now: DateTime<Local> = Local::now();
        let edited: String = now.format("%Y-%m-%d %H:%M:%S").to_string();
        self.setPropery("updated".to_string(), vec!["".to_string(), edited], true);
        self
    }

    pub fn done(&mut self, indexes: Vec<String>) -> &mut Self {
        let mut isEdited = false;
        for index in indexes {
            let indexVal: usize = index.parse().unwrap();
            if indexVal > self.items.len() {
                println!("index out of range");
                continue;
            } else {
                let now: DateTime<Local> = Local::now();
                self.items[indexVal].status = true;
                self.items[indexVal].ended = now.format("%Y-%m-%d %H:%M:%S").to_string();
                self.items[indexVal].edited = now.format("%Y-%m-%d %H:%M:%S").to_string();
            }
        }

        if isEdited {
            self.setupLastEdited();
        }

        self
    }

    pub fn undone(&mut self, indexes: Vec<String>) -> &mut Self {
        let mut isEdited = false;
        for index in indexes {
            let indexVal: usize = index.parse().unwrap();
            if indexVal > self.items.len() {
                println!("index out of range");
                continue;
            } else {
                let now: DateTime<Local> = Local::now();
                self.items[indexVal].status = false;
                self.items[indexVal].ended = String::from("");
                self.items[indexVal].edited = now.format("%Y-%m-%d %H:%M:%S").to_string();
                isEdited = true;
            }
        }
        if isEdited {
            self.setupLastEdited();
        }

        self
    }
    pub fn show(&mut self) -> &mut Self {
        let renderer = &mut self.renderer;
        let items = &self.items;
        let mut titleNumber = TString::new(String::from("#"));
        titleNumber.setAnsi(TStringStatic::getForeground("lightGray"));

        let mut headerVec = vec![
            titleNumber,
            TString::new(String::from("Status")),
            TString::new(String::from("Title")),
            TString::new(String::from("Created")),
            TString::new(String::from("Edited")),
            TString::new(String::from("Ended")),
        ];

        let excludedColumns = &self
            .properties
            .get("excludedColumns")
            .map(|v| v.to_string())
            .unwrap_or_else(|| String::from(""));

        let excludedColumns: Vec<&str> = excludedColumns.split(",").collect();
        headerVec.retain(|item| !excludedColumns.contains(&item.getText().as_str()));

        renderer.setHeader(headerVec.clone());

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
                let edited = TString::new(val.edited.to_string());

                let mut row = vec![];
                if headerVec
                    .iter()
                    .any(|item| item.getText() == "Number".to_string())
                {
                    row.push(number.clone());
                }
                if headerVec
                    .iter()
                    .any(|item| item.getText() == "Status".to_string())
                {
                    row.push(status);
                }
                if headerVec
                    .iter()
                    .any(|item| item.getText() == "Title".to_string())
                {
                    row.push(title);
                }
                if headerVec
                    .iter()
                    .any(|item| item.getText() == "Created".to_string())
                {
                    row.push(created);
                }
                if headerVec
                    .iter()
                    .any(|item| item.getText() == "Edited".to_string())
                {
                    row.push(edited);
                }
                if headerVec
                    .iter()
                    .any(|item| item.getText() == "Ended".to_string())
                {
                    row.push(ended);
                }
                println!("11{:?}", row);
                renderer.setRow(row);
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
                        let edited = TString::new(val.edited.to_string());
                        let mut row = vec![];
                        if headerVec
                            .iter()
                            .any(|item| item.getText() == "Number".to_string())
                        {
                            row.push(number.clone());
                        }
                        if headerVec
                            .iter()
                            .any(|item| item.getText() == "Status".to_string())
                        {
                            row.push(status);
                        }
                        if headerVec
                            .iter()
                            .any(|item| item.getText() == "Title".to_string())
                        {
                            row.push(title);
                        }
                        if headerVec
                            .iter()
                            .any(|item| item.getText() == "Created".to_string())
                        {
                            row.push(created);
                        }
                        if headerVec
                            .iter()
                            .any(|item| item.getText() == "Edited".to_string())
                        {
                            row.push(edited);
                        }
                        if headerVec
                            .iter()
                            .any(|item| item.getText() == "Ended".to_string())
                        {
                            row.push(ended);
                        }
                        println!("22{:?}", row);
                        renderer.setRow(row);
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
        let mut tsCommand = TString::new(String::from("Command"));
        let mut tsDescr = TString::new(String::from("Description"));
        let mut tsAliases = TString::new(String::from("Aliases"));
        tsCommand.setAnsi(TStringStatic::getForeground("lightGray"));
        tsDescr.setAnsi(TStringStatic::getForeground("lightGray"));
        tsAliases.setAnsi(TStringStatic::getForeground("lightGray"));
        renderer.setHeader(vec![tsCommand.clone(), tsDescr.clone(), tsAliases.clone()]);
        renderer.setRow(vec![
            TString::new(String::from("ls")),
            TString::new(String::from("Show all tasks")),
            TString::new(String::from("log, show, list")),
        ]);
        renderer.setRow(vec![
            TString::new(String::from("add")),
            TString::new(String::from("Add a task")),
            TString::new(String::from("")),
        ]);
        renderer.setRow(vec![
            TString::new(String::from("rm")),
            TString::new(String::from("Delete a task")),
            TString::new(String::from("delete, del")),
        ]);
        renderer.setRow(vec![
            TString::new(String::from("push")),
            TString::new(String::from("Add a task")),
            TString::new(String::from("")),
        ]);
        renderer.setRow(vec![
            TString::new(String::from("done")),
            TString::new(String::from("Mark a task as done")),
            TString::new(String::from("")),
        ]);
        renderer.setRow(vec![
            TString::new(String::from("undone")),
            TString::new(String::from("Mark a task as undone")),
            TString::new(String::from("")),
        ]);
        renderer.setRow(vec![
            TString::new(String::from("config")),
            TString::new(String::from("Show a property")),
            TString::new(String::from("")),
        ]);
        renderer.setRow(vec![
            TString::new(String::from("set")),
            TString::new(String::from("Set a property")),
            TString::new(String::from("")),
        ]);
        renderer.setRow(vec![
            TString::new(String::from("unset")),
            TString::new(String::from("Unset a property")),
            TString::new(String::from("")),
        ]);
        renderer.setRow(vec![
            TString::new(String::from("help")),
            TString::new(String::from("Show this help")),
            TString::new(String::from("?")),
        ]);

        renderer.setRow(vec![
            TString::new(String::from("version")),
            TString::new(String::from("Show version")),
            TString::new(String::from("v")),
        ]);

        renderer.setRow(vec![
            TString::new(String::from("md")),
            TString::new(String::from("Save markdown file")),
            TString::new(String::from("")),
        ]);

        renderer.setRow(vec![
            TString::new(String::from("html")),
            TString::new(String::from("Save html file")),
            TString::new(String::from("")),
        ]);

        renderer.adaptColumnLengths().render();
        self
    }

    pub fn showVersion(&mut self) -> &mut Self {
        let mut renderer = &mut self.renderer;
        let mut c1 = TString::new(String::from(""));
        let mut c2 = TString::new(String::from(""));
        renderer.setHeader(vec![c1.clone(), c2.clone()]);
        renderer.setRow(vec![
            TString::new(String::from("ToDo:")),
            TString::new(String::from(env!("CARGO_PKG_VERSION"))),
        ]);
        renderer.adaptColumnLengths().render();
        self
    }

    pub fn getMd(&mut self, isToHtml: bool) -> String {
        let mdRef = &mut self.markdownRender;
        let properties = self.properties.clone();
        let items = self.items.clone();
        let strval: String = (mdRef.todoToMarkdown(properties, items, isToHtml)).to_string();
        strval
    }

    pub fn getHtml(&mut self) -> String {
        let markdownString: &String = &self.getMd(true);
        let mut strval: String = String::from("");
        {
            let mdRef = &mut self.markdownRender;
            strval = mdRef.mdToHtml(markdownString.as_str().to_string());
        }
        strval
    }

    pub fn saveMd(&mut self) -> &mut Self {
        let markdownString: String = self.getMd(false);
        let settings = &self.settings;
        let mdFile: String = settings.get(String::from("mdFile")).unwrap().to_string();
        self.fileWorker.write(mdFile, markdownString).unwrap();
        self
    }

    pub fn saveHtml(&mut self) -> &mut Self {
        let htmlString: String = self.getHtml();
        let settings = &self.settings;
        let htmlTemplate: String = settings
            .template(String::from("todoHTML"))
            .unwrap()
            .to_string();
        let cssStyles: String = settings
            .template(String::from("cssStyles"))
            .unwrap()
            .to_string();
        let htmlFile: String = settings.get(String::from("htmlFile")).unwrap().to_string();
        let title = self.getProperty("title".to_string());
        let mut htmlRes = htmlTemplate.replace("%title%", title.as_str());
        htmlRes = htmlRes.replace("%content%", htmlString.as_str());
        htmlRes = htmlRes.replace("%style%", cssStyles.as_str());
        self.fileWorker.write(htmlFile, htmlRes).unwrap();
        self
    }

    pub fn rmTaskByIndex(&mut self, indexes: Vec<String>) -> &mut Self {
        let mut vi32ItemsForDelete: Vec<i32> = Vec::new();
        let mut isEdited = false;
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
                isEdited = true;
            }
        }
        if isEdited {
            self.setupLastEdited();
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
            "list" => self.show(),
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
            "done" => self.done(scannerRef.params.clone()).sync().show(),
            "undone" => self.undone(scannerRef.params.clone()).sync().show(),
            "config" => {
                self.showProperty(scannerRef.param.clone().to_string());
                self
            }
            "unset" => self
                .unsetProperty(scannerRef.param.clone().to_string())
                .sync()
                .showProperty("".to_string()),
            "set" => {
                self.setPropery(
                    scannerRef.param.clone().to_string(),
                    scannerRef.params.clone(),
                    false,
                )
                .sync()
                .showProperty("".to_string());
                self
            }
            "help" => {
                self.help();
                self
            }
            "md" => {
                self.saveMd();
                self
            }
            "html" => {
                self.saveHtml();
                self
            }
            "?" => {
                self.help();
                self
            }
            "version" => self.showVersion(),
            "v" => self.showVersion(),
            &_ => self.show(),
        }
    }
}
