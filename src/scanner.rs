use std::env;

pub struct Scanner {
    pub args: Vec<String>,
    pub path: String,
    pub command: String,
    pub param: String,
}

impl Scanner {
    pub fn new() -> Scanner {
        let args: Vec<String> = env::args().collect();
        let path: String = args.get(0).map(|s| s.clone()).unwrap_or("".to_string());
        let command: String = args.get(1).map(|s| s.clone()).unwrap_or("".to_string());
        let param: String = args.get(2).map(|s| s.clone()).unwrap_or("".to_string());
        Scanner {
            args,
            path,
            command,
            param,
        }
    }
    pub fn log(&self) {
        println!("\n{:?}", self.args);
        println!("\n{:?}", self.path);
        println!("\n{:?}", self.command);
        println!("\n{:?}", self.param);
    }
}
