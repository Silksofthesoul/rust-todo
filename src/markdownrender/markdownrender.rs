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
        //items.push_str("## Items\n");
        //items.push_str("\n");
        //    for item in todoItems {
        //        let status = item.status;
        //        let checkbox = if status { "[x]" } else { "[ ]" };
        //        let title = item.title;
        //        let created = item.created;
        //        let ended = item.ended;
        //        items.push_str(&format!(
        //            "- {} {} | {} - {}\n",
        //            checkbox, title, created, ended
        //        ));
        //    }

        /* items: */
        items.push_str("## Items\n");
        items.push_str("\n");
        items.push_str("| Status | Title | Created | Ended |\n");
        items.push_str("| --- | --- | --- | --- |\n");
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
            items.push_str(&format!(
                "| {} | {} | {} | {} |\n",
                checkbox, title, created, ended
            ));
            //items.push_str(&format!(
            //    "- {} {} | {} - {}\n",
            //    checkbox, title, created, ended
            //));
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
