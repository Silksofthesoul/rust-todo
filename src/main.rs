#![allow(non_snake_case)]

mod fileWorker;
mod renderer;
mod scanner;
mod settings;
mod todo;

mod library {
    pub mod json;
    pub mod math;
    pub mod print;
    pub mod strHelpers;
}

use todo::Todo;

fn main() {
    let mut todo = Todo::new();
    todo.run();
}
