use super::super::todo::TodoItem;
use serde_yaml;
use std::collections::HashMap;

pub struct MarkdownRender {
    fileName: String,
    content: String,
}

impl MarkdownRender {
    pub fn new(fileName: &str) -> MarkdownRender {
        MarkdownRender {
            fileName: String::from(fileName),
            content: String::from(""),
        }
    }

    pub fn todoToMarkdown(
        &mut self,
        todoProperties: HashMap<String, String>,
        todoItems: Vec<TodoItem>,
    ) -> String {
        let mut markdown = String::from("");
        let mut properties = String::from("");
        let mut items = String::from("");

        /* properties: */
        let yamlProperties = serde_yaml::to_string(&todoProperties).unwrap();

        //properties.push_str("---\n");
        properties.push_str(&yamlProperties);
        properties.push_str("---\n");

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
        for item in todoItems {
            let status = item.status;
            let checkbox = if status { "[x]" } else { "[ ]" };
            let title = item.title;
            let created = item.created;
            let ended = item.ended;
            items.push_str(&format!(
                "- {} {} | {} - {}\n",
                checkbox, title, created, ended
            ));
        }

        markdown.push_str(&properties);
        markdown.push_str(&items);

        markdown
    }
}
