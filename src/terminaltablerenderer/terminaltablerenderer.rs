use std::vec::Vec;

use crate::library::math::max;
use crate::tstring::tstring::TString;

pub struct TerminalTableRenderer {
    pub rows: Vec<Vec<TString>>,
}

impl TerminalTableRenderer {
    pub fn new() -> TerminalTableRenderer {
        let cells = vec![TString::new("".to_string())];
        let rows = vec![cells];
        TerminalTableRenderer { rows }
    }

    pub fn setHeader(&mut self, cell: Vec<TString>) -> &mut Self {
        self.rows[0] = cell;
        self
    }

    pub fn setRow(&mut self, cell: Vec<TString>) -> &mut Self {
        self.rows.push(cell);
        self
    }

    pub fn getColumnMaxLengthByIndex(&self, index: usize) -> usize {
        let mut vLength: Vec<i32> = Vec::new();
        for row in &self.rows {
            vLength.push(row[index].clone().getLength() as i32);
        }
        max(vLength) as usize
    }

    pub fn adaptColumnLengths(&mut self) -> &mut Self {
        if self.rows.is_empty() {
            return self;
        }

        let mut maxLength: Vec<usize> = vec![0; self.rows[0].len()];

        for index in 0..maxLength.len() {
            maxLength[index] = self.getColumnMaxLengthByIndex(index);
        }

        for row in self.rows.iter_mut() {
            for index in 0..row.len() {
                let cell = &mut row[index];
                cell.setParam("width".to_string(), maxLength[index].to_string())
                    .setParam("align".to_string(), "left".to_string())
                    .setParam("padStart".to_string(), "1".to_string())
                    .setParam("padEnd".to_string(), "1".to_string());
                //row[index] = cell;
            }
        }
        self
    }

    pub fn flush(&mut self) -> &mut Self {
        let cells = vec![TString::new("".to_string())];
        let rows = vec![cells];
        self.rows = rows;
        self
    }

    pub fn render(&mut self) -> &mut Self {
        for row in &self.rows {
            let mut rowView = String::new();
            for cell in row {
                rowView.push_str(&cell.view());
            }
            println!("{}", rowView);
        }
        self
    }
}
