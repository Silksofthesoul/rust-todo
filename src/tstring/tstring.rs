#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use once_cell::sync::Lazy;
use std::{collections::HashMap, fmt::write, usize};

use crate::library::print;

// INFO: TString -- terminal string
// концепт: строка содержащая текст, и аттрибуты.
// при рендеринге, мы применяем аттрибуты для строки.

#[derive(Debug, Clone)]
pub struct TString {
    pub text: String,
    pub ansi: Vec<String>,
    pub params: HashMap<String, String>,
}

static SYMBOLS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("space", " ");
    map.insert("tab", "\t");
    map.insert("enter", "\n");
    map.insert("vspace", map.get("enter").unwrap());
    map.insert("hspace", map.get("space").unwrap());
    map
});

static ATTRIBUTES: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("reset", "\x1b[0m");
    map.insert("bold", "\x1b[1m");
    map.insert("underline", "\x1b[4m");
    map.insert("blink", "\x1b[5m");
    map.insert("boldOff", "\x1b[21m");
    map.insert("underlineOff", "\x1b[24m");
    map.insert("blinkOff", "\x1b[25m");
    map
});

static FOREGROUND: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("black", "\x1b[30m");
    map.insert("red", "\x1b[31m");
    map.insert("green", "\x1b[32m");
    map.insert("yellow", "\x1b[33m");
    map.insert("blue", "\x1b[34m");
    map.insert("magenta", "\x1b[35m");
    map.insert("cyan", "\x1b[36m");
    map.insert("white", "\x1b[37m");
    map.insert("default", "\x1b[39m");
    map.insert("lightGray", "\x1b[90m");
    map.insert("lightRed", "\x1b[91m");
    map.insert("lightGreen", "\x1b[92m");
    map.insert("lightYellow", "\x1b[93m");
    map.insert("lightBlue", "\x1b[94m");
    map.insert("lightMagenta", "\x1b[95m");
    map.insert("lightCyan", "\x1b[96m");
    map.insert("lightWhite", "\x1b[97m");
    map
});

static BACKGROUND: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("black", "\x1b[40m");
    map.insert("red", "\x1b[41m");
    map.insert("green", "\x1b[42m");
    map.insert("yellow", "\x1b[43m");
    map.insert("blue", "\x1b[44m");
    map.insert("magenta", "\x1b[45m");
    map.insert("cyan", "\x1b[46m");
    map.insert("white", "\x1b[47m");
    map.insert("default", "\x1b[49m");
    map.insert("lightGray", "\x1b[100m");
    map.insert("lightRed", "\x1b[101m");
    map.insert("lightGreen", "\x1b[102m");
    map.insert("lightYellow", "\x1b[103m");
    map.insert("lightBlue", "\x1b[104m");
    map.insert("lightMagenta", "\x1b[105m");
    map.insert("lightCyan", "\x1b[106m");
    map.insert("lightWhite", "\x1b[107m");
    map
});

fn substractUsize(a: usize, b: usize) -> usize {
    if a > b {
        (a - b) as usize
    } else {
        0 as usize
    }
}

impl TString {
    pub fn new(text: String) -> TString {
        TString {
            text,
            ansi: Vec::new(),
            params: HashMap::new(),
        }
    }

    pub fn setAnsi(mut self, prop: String) -> TString {
        self.ansi.push(prop);
        self
    }

    pub fn setParam(mut self, key: String, value: String) -> TString {
        self.params.insert(key, value);
        self
    }

    pub fn getLength(&self) -> usize {
        self.text.len()
    }
    pub fn getText(&self) -> String {
        self.text.clone()
    }
    pub fn getAnsi(&self) -> Vec<String> {
        self.ansi.clone()
    }
    pub fn view(&self) -> String {
        // ANSI:
        let mut result = String::new();
        for a in &self.ansi {
            result.push_str(a);
        }

        result.push_str(&self.text);
        result.push_str(TString::getAttributes("reset").as_str());

        // PARAMS: Align & Width

        let width = &self
            .params
            .get("width")
            .unwrap_or(&String::from("0"))
            .parse::<usize>()
            .unwrap();

        let binding = String::from("left");
        let align = &self.params.get("align").unwrap_or(&binding);

        let currentLength = self.getLength();
        let pad = TString::getSymbol("hspace");

        if width > &(0 as usize) && width > &(currentLength) {
            if *align == "right" {
                let len: usize = substractUsize(*width, currentLength);
                for _ in 0..len {
                    result.insert_str(0, pad.as_str());
                }
            } else if *align == "left" {
                let len: usize = substractUsize(*width, currentLength);
                for _ in 0..len {
                    result.push_str(pad.as_str());
                }
            }
        }

        // PARAMS: InnerPadding

        let padStart = &self
            .params
            .get("padStart")
            .unwrap_or(&String::from("0"))
            .parse::<usize>()
            .unwrap();

        let padEnd = &self
            .params
            .get("padEnd")
            .unwrap_or(&String::from("0"))
            .parse::<usize>()
            .unwrap();
        if padStart > &(0 as usize) {
            for _ in 0..*padStart {
                result.insert_str(0, pad.as_str());
            }
        }
        if padEnd > &(0 as usize) {
            for _ in 0..*padEnd {
                result.push_str(pad.as_str());
            }
        }

        result
    }

    // STATIC
    pub fn getSymbol(key: &str) -> String {
        if let Some(val) = SYMBOLS.get(key) {
            val.to_string()
        } else {
            String::from("")
        }
    }

    pub fn getAttributes(key: &str) -> String {
        if let Some(val) = ATTRIBUTES.get(key) {
            val.to_string()
        } else {
            String::from("")
        }
    }
    pub fn getForeground(key: &str) -> String {
        if let Some(val) = FOREGROUND.get(key) {
            val.to_string()
        } else {
            String::from("")
        }
    }
    pub fn getBackground(key: &str) -> String {
        if let Some(val) = BACKGROUND.get(key) {
            val.to_string()
        } else {
            String::from("")
        }
    }
}
