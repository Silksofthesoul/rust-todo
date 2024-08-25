use super::renderer::Renderer;
use crate::library::math;
use crate::library::strHelpers;
use std::collections::HashMap;

// DESCRIPTION:
// Управление стилями текста в консоли.

impl Renderer {
    // Flow: Line
    pub fn setLine(&mut self, str: &str) -> &mut Self {
        self.bufferLine.push(str.to_string());
        self
    }

    pub fn n(&mut self) -> &mut Self {
        self.bufferLine.push("\n".to_string());
        self
    }

    pub fn flushLine(&mut self) -> String {
        self.reset();
        let res = self.bufferLine.join("");
        self.bufferLine = Vec::new();
        res
    }

    // Flow: Row
    pub fn setRow(&mut self, str: String) -> &mut Self {
        self.bufferRow.push(str);
        self
    }
    pub fn flushRow(&mut self) -> Vec<String> {
        let res = self.bufferRow.clone();
        self.bufferRow = Vec::new();
        res
    }

    //Flow: Table
    pub fn setTable(&mut self, row: Vec<String>) -> &mut Self {
        self.bufferTable.push(row);
        self
    }
    pub fn flushTable(&mut self) -> String {
        let bufferTable = self.bufferTable.clone();

        let mut sizes: HashMap<usize, Vec<i32>> = HashMap::new();
        let mut columnMax: Vec<usize> = Vec::new(); // INFO: максимальные значения длинны строк

        for row in &bufferTable {
            for (index, item) in row.iter().enumerate() {
                if sizes.contains_key(&index) {
                    sizes.get_mut(&index).unwrap().push(item.len() as i32);
                } else {
                    sizes.insert(index, vec![item.len() as i32]);
                }
            }
        }

        for sizes in sizes.iter() {
            let columnSizes = sizes.1.clone();
            let max = math::max(columnSizes);
            columnMax.push(max as usize);
        }

        println!("columnMax: {:?}", columnMax);
        let mut strTable = String::new();
        for row in &bufferTable {
            let mut strRow = String::new();
            for (index, item) in row.iter().enumerate() {
                let cellLen: usize = item.len();
                let maxColumnLen = columnMax[index];
                if maxColumnLen > cellLen {
                    let newitem = strHelpers::strPadToEnd(item, ' ', maxColumnLen - cellLen);
                    strRow.push_str(&newitem);
                } else {
                    strRow.push_str(&item);
                }
            }
            strTable.push_str(&strRow);
            strTable.push_str("\n");
        }
        return String::from(strTable);
    }
}
