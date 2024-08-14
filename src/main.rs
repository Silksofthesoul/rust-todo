#![allow(non_snake_case)]

mod fileReader;
mod settings;
mod todo;

use fileReader::FileReader;
use settings::Settings;
use todo::Todo;

fn main() {
    line();
    let settings = Settings::new();
    let reader = FileReader::new(settings.fileDataBase.as_str(), settings.template.as_str());
    let content = reader.read().unwrap();
    let todo = Todo::new(content);
    todo.show()
        .addTask("Помыть посуду")
        .addTask("Вынести мусор")
        .done(0)
        .show();
    line();
}

fn line() {
    println!("[==================]")
}
