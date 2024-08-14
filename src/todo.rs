#![allow(non_snake_case)]

// use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::Result;
use serde_json::Value;
// use std::collections::HashMap;
use std::vec::Vec;

fn parse(data: String) -> Result<TodoJSON> {
    let parsed: TodoJSON = serde_json::from_str(data.as_str())?;
    Ok(parsed)
}

#[derive(Serialize, Deserialize, Debug)]
struct Properties {
    ownerName: String,
}

#[derive(Deserialize, Debug)]
struct TodoItem {
    status: bool,
    title: String,
    created: String,
    ended: String,
}

#[derive(Deserialize, Debug)]
struct TodoJSON {
    properties: Properties,
    items: Vec<TodoItem>,
}

pub struct Todo {
    properties: Properties,
    items: Vec<TodoItem>,
}

impl Todo {
    pub fn new(data: String) -> Todo {
        Todo {
            properties: Self::initProperties(data.clone()),
            items: Self::initItems(data.clone()),
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

    pub fn addTask(mut self, title: &str) -> Self {
        let todoItem = TodoItem {
            status: false,
            title: String::from(title),
            created: String::from("0"),
            ended: String::from(""),
        };
        self.items.push(todoItem);
        self
    }
    pub fn done(mut self, index: usize) -> Self {
        let mut isOverflow = false;
        if index > self.items.len() {
            isOverflow = true;
        }
        if !isOverflow {
            self.items[index].status = true;
        }
        self
    }

    pub fn show(self) -> Self {
        println!("\n\n");
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
        self
    }
}
