use super::super::todo::TodoItem;
use pulldown_cmark::{html, Options, Parser};
use serde_yaml;
use std::collections::HashMap;

pub struct MarkdownRender {}

impl MarkdownRender {
    pub fn new() -> MarkdownRender {
        MarkdownRender {}
    }

    pub fn todoToMarkdown(
        &mut self,
        todoProperties: HashMap<String, String>,
        todoItems: Vec<TodoItem>,
        isHtmlRender: bool,
    ) -> String {
        let mut markdown = String::from("");
        let mut properties = String::from("");
        let mut items = String::from("");

        let excludedColumns = todoProperties
            .get("excludedColumns")
            .map(|v| v.to_string())
            .unwrap_or_else(|| String::from(""));
        let excludedColumns: Vec<&str> = excludedColumns.split(",").collect();

        /* properties: */
        let yamlProperties = serde_yaml::to_string(&todoProperties).unwrap();

        //properties.push_str("---\n");
        properties.push_str(&yamlProperties);
        properties.push_str("---\n");

        markdown.push_str(&properties);

        /* title: */
        let mut commonTitle: String = String::from("");
        match todoProperties.get("title") {
            Some(value) => commonTitle = value.to_string(),
            None => println!(""),
        }

        if commonTitle != "" {
            markdown.push_str(&format!("# {}\n", commonTitle));
        } else {
            let defaultTitle = String::from("ToDo");
            markdown.push_str(&format!("# {}\n", defaultTitle));
        }

        /* items: */
        items.push_str("## Items\n");
        items.push_str("\n");

        let mut vecColumns = vec!["Status", "Title", "Created", "Edited", "Ended"];
        vecColumns.retain(|item| !excludedColumns.contains(&item));

        let mut strColumns = String::from("|");
        let mut strUnderColumns = String::from("|");
        for column in &vecColumns {
            strColumns.push_str(&format!(" {} |", column));
            strUnderColumns.push_str(&format!("--- |"));
        }
        strColumns.push_str("\n");
        strUnderColumns.push_str("\n");

        items.push_str(strColumns.as_str());
        items.push_str(strUnderColumns.as_str());

        for item in todoItems {
            let status = item.status;
            let mut checkbox = "";
            if isHtmlRender {
                checkbox = if status {
                    "<input type=\"checkbox\" checked disabled>"
                } else {
                    "<input type=\"checkbox\" disabled>"
                };
            } else {
                checkbox = if status { "[x]" } else { "[ ]" };
            }

            let title = item.title;
            let created = item.created;
            let ended = item.ended;
            let mut row: Vec<String> = Vec::new();
            let mut strRow = String::from("|");

            if vecColumns.contains(&"#") {
                row.push(checkbox.to_string());
            }
            if vecColumns.contains(&"Status") {
                row.push(checkbox.to_string());
            }
            if vecColumns.contains(&"Title") {
                row.push(title.to_string());
            }
            if vecColumns.contains(&"Created") {
                row.push(created.to_string());
            }
            if vecColumns.contains(&"Edited") {
                row.push(created.to_string());
            }
            if vecColumns.contains(&"Ended") {
                row.push(ended.to_string());
            }

            for column in row {
                strRow.push_str(&format!(" {} |", column));
            }
            strRow.push_str("\n");
            items.push_str(strRow.as_str());
        }

        markdown.push_str(&items);

        markdown
    }

    pub fn mdToHtml(&mut self, mdString: String) -> String {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_TASKLISTS);
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
        options.insert(Options::ENABLE_YAML_STYLE_METADATA_BLOCKS);
        let parser = Parser::new_ext(&mdString, options);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        html_output
    }
}
