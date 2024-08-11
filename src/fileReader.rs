#![allow(non_snake_case)]

use std::fs;
use std::io::{self, Write};
use std::error::Error;

pub struct FileReader {
    path: String,
    template: String,
    errorFile: String,
    errorFile404: String,
    msgCreateNewFile: String,
}

impl FileReader {
    pub fn new(path: &str, template: &str) -> FileReader {
        FileReader {
            path: String::from(path),
            template: String::from(template),
            errorFile: String::from("Ошибка, чтения файла."),
            errorFile404: String::from("Файл не найден"),
            msgCreateNewFile: String::from("Создаем новый файл"),
        }
    }

    pub fn read(&self) -> Result<String, Box<dyn Error>> {
        let mut value: String = String::from("");
        let path = self.path.as_str();
        let template = self.template.as_str();

        let commonError = self.errorFile.as_str();
        let notFoundError = self.errorFile404.as_str();
        let createMsg = self.msgCreateNewFile.as_str();

        match fs::read_to_string(path) {
            Ok(content) => {
                value = content;
            }
            Err(err) => {
                if err.kind() == io::ErrorKind::NotFound {
                    println!("{}", commonError);
                    println!("{}", notFoundError);
                    println!("{}",createMsg);
                    let mut file = fs::File::create(path)?;
                    value = String::from(template);
                    file.write_all(value.as_bytes())?;
                }
            }
        }
        Ok(value)
    }
}
