#![allow(non_snake_case)]

mod fileWorker;
mod scanner;
mod settings;
mod terminaltablerenderer;
mod todo;
mod tstring;

mod library {
    pub mod json;
    pub mod math;
    pub mod print;
}

use todo::Todo;

fn main() {
    let mut todo = Todo::new();
    todo.run();
}
