use crate::tstring::tstring::TString;
use std::vec::Vec;

pub struct TerminalTableRenderer {
    pub rows: Vec<Vec<TString>>,
}

impl TerminalTableRenderer {
    pub fn new() -> TerminalTableRenderer {
        let cells = vec![TString::new("header2".to_string())];
        let rows = vec![cells];
        TerminalTableRenderer { rows }
    }

    pub fn setHeader(&mut self, cell: Vec<TString>) -> &Self {
        self.rows.insert(0, cell);
        self
    }

    pub fn setRow(&mut self, cell: Vec<TString>) -> &Self {
        self.rows.push(cell);
        self
    }

    pub fn render(&self) {
        for row in &self.rows {
            let mut rowView = String::new();
            for cell in row {
                rowView.push_str(&cell.view());
            }
            println!("{}", rowView);
        }
    }
}
