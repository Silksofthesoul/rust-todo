#![allow(non_snake_case)]

use std::collections::HashMap;

struct tString {
    pub text: String,
    pub props: Vec<String>,
    pub isClosed: bool,

    // properties collections:
    pub attributes: HashMap<String, String>,
    pub foreground: HashMap<String, String>,
    pub background: HashMap<String, String>,
}

impl tString {
    pub fn new(text: String) -> tString {
        let attributes: HashMap<String, String> = HashMap::from([
            (String::from("reset"), String::from("\x1b[0m")),
            (String::from("bold"), String::from("\x1b[1m")),
            (String::from("underline"), String::from("\x1b[4m")),
            (String::from("blink"), String::from("\x1b[5m")),
            (String::from("boldOff"), String::from("\x1b[21m")),
            (String::from("underlineOff"), String::from("\x1b[24m")),
            (String::from("blinkOff"), String::from("\x1b[25m")),
        ]);

        let foreground: HashMap<String, String> = HashMap::from([
            (String::from("black"), String::from("\x1b[30m")),
            (String::from("red"), String::from("\x1b[31m")),
            (String::from("green"), String::from("\x1b[32m")),
            (String::from("yellow"), String::from("\x1b[33m")),
            (String::from("blue"), String::from("\x1b[34m")),
            (String::from("magenta"), String::from("\x1b[35m")),
            (String::from("cyan"), String::from("\x1b[36m")),
            (String::from("white"), String::from("\x1b[37m")),
            (String::from("default"), String::from("\x1b[39m")),
            (String::from("lightGray"), String::from("\x1b[90m")),
            (String::from("lightRed"), String::from("\x1b[91m")),
            (String::from("lightGreen"), String::from("\x1b[92m")),
            (String::from("lightYellow"), String::from("\x1b[93m")),
            (String::from("lightBlue"), String::from("\x1b[94m")),
            (String::from("lightMagenta"), String::from("\x1b[95m")),
            (String::from("lightCyan"), String::from("\x1b[96m")),
            (String::from("lightWhite"), String::from("\x1b[97m")),
        ]);

        let background: HashMap<String, String> = HashMap::from([
            (String::from("black"), String::from("\x1b[40m")),
            (String::from("red"), String::from("\x1b[41m")),
            (String::from("green"), String::from("\x1b[42m")),
            (String::from("yellow"), String::from("\x1b[43m")),
            (String::from("blue"), String::from("\x1b[44m")),
            (String::from("magenta"), String::from("\x1b[45m")),
            (String::from("cyan"), String::from("\x1b[46m")),
            (String::from("white"), String::from("\x1b[47m")),
            (String::from("default"), String::from("\x1b[49m")),
            (String::from("lightGray"), String::from("\x1b[100m")),
            (String::from("lightRed"), String::from("\x1b[101m")),
            (String::from("lightGreen"), String::from("\x1b[102m")),
            (String::from("lightYellow"), String::from("\x1b[103m")),
            (String::from("lightBlue"), String::from("\x1b[104m")),
            (String::from("lightMagenta"), String::from("\x1b[105m")),
            (String::from("lightCyan"), String::from("\x1b[106m")),
            (String::from("lightWhite"), String::from("\x1b[107m")),
        ]);

        tString {
            text,
            isClosed: true,
            props: Vec::new(),

            // properties collections:
            attributes,
            foreground,
            background,
        }
    }

    // pub fn addAttribute(mut self, attribute: String) -> tString {
    //     self.attributes.push(attribute);
    //     self
    // }

    pub fn addAttributes(mut self, attributes: Vec<String>) -> tString {
        self.attributes.extend(attributes);
        self
    }

    pub fn close(mut self) -> tString {
        self.isClosed = true;
        self
    }

    pub fn open(mut self) -> tString {
        self.isClosed = false;
        self
    }
}
