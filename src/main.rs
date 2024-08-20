#![allow(non_snake_case)]

mod fileWorker;
mod scanner;
mod settings;
mod todo;
mod library {
    pub mod json;
    pub mod print;
}

use todo::Todo;

fn main() {
    let mut todo = Todo::new();
    todo.run();
}
