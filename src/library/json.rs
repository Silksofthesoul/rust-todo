use serde_json;
use serde_json::Result;

use crate::todo::TodoJSON;
use crate::todo::Todo;

pub fn parse(data: String) -> Result<TodoJSON> {
    let parsed: TodoJSON = serde_json::from_str(data.as_str())?;
    Ok(parsed)
}

pub fn stringify(data: &Todo) -> String {
    serde_json::to_string(&data).unwrap()
}
