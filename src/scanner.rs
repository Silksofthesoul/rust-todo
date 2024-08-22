use std::env;

pub struct Scanner {
    pub command: String,
    pub param: String,
    pub params: Vec<String>,
}

impl Scanner {
    pub fn new() -> Scanner {
        let args: Vec<String> = env::args().collect();
        // let path: String = args.get(0).map(|s| s.clone()).unwrap_or("".to_string());
        let command: String = args.get(1).map(|s| s.clone()).unwrap_or("".to_string());
        let param: String = args.get(2).map(|s| s.clone()).unwrap_or("".to_string());
        let mut params: Vec<String> = Vec::new();
        if args.len() > 2 {
            params.extend(args[2..].to_vec());
        }
        Scanner {
            command,
            param,
            params,
        }
    }
}
