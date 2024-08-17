#![allow(non_snake_case)]

mod todo;
use todo::Todo;

fn main() {
    line();

    let todo = Todo::new();
    todo
        .show()
        .addTask("Помыть посуду")
        .addTask("Вынести мусор")
        .done(0)
        .show()
        .sync();
    line();
}

fn line() {
    println!("[==================]")
}
