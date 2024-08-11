#![allow(non_snake_case)]

mod settings;
mod fileReader;
use settings::Settings;
use fileReader::FileReader;

fn main() {
    line();
    let settings = Settings::new();
    let reader = FileReader::new(settings.fileDataBase.as_str(), settings.template.as_str());
    let content = reader.read().unwrap();

    println!("{}", content);

    line();
}

fn line() { println!("[==================]") }
