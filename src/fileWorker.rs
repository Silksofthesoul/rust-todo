#![allow(non_snake_case)]

use std::error::Error;
use std::fs;
use std::io::{self, Write};

pub struct FileWorker {
    errorFile: String,
    errorFile404: String,
    msgCreateNewFile: String,
    isInitNewFile: bool,
}

impl FileWorker {
    pub fn new(isInitNewFile: bool) -> FileWorker {
        let errorFile = String::from("Ошибка, чтения файла.");
        let errorFile404 = String::from("Файл не найден");
        let msgCreateNewFile = String::from("Создаем новый файл");

        FileWorker {
            errorFile,
            errorFile404,
            msgCreateNewFile,
            isInitNewFile,
        }
    }

    pub fn allowNewFile(&mut self) -> &Self {
        self.isInitNewFile = true;
        self
    }
    pub fn disallowNewFile(&mut self) -> &mut Self {
        self.isInitNewFile = false;
        self
    }

    pub fn fileToString(&self, path: String, template: String) -> Result<String, Box<dyn Error>> {
        let mut value: String = String::from("");
        let path = path.as_str();
        let template = template.as_str();

        let commonError = self.errorFile.as_str();
        let notFoundError = self.errorFile404.as_str();
        let createMsg = self.msgCreateNewFile.as_str();

        match fs::read_to_string(path) {
            Ok(content) => {
                value = content;
            }
            Err(err) => {
                if err.kind() == io::ErrorKind::NotFound {
                    if self.isInitNewFile {
                        println!("{}", commonError);
                        println!("{}", notFoundError);
                        println!("{}", createMsg);
                        let mut file = fs::File::create(path)?;
                        value = String::from(template);
                        file.write_all(value.as_bytes())?;
                    } else {
                        value = String::from(template);
                    }
                }
            }
        }
        Ok(value)
    }

    pub fn write(&self, filename: String, data: String) -> Result<(), io::Error> {
        let mut file = fs::File::create(filename)?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }
}
