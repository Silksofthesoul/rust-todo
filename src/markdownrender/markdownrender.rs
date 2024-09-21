use serde_yaml;

use super::super::todo::Todo;

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
    pub fn todoToMarkdown(&mut self, todo: &mut Todo) -> String {
        let mut markdown = String::from("");
        let mut properties = String::from("");
        let mut items = String::from("");

        /* properties: */
        let todoProperties = todo.getProperties();
        let yamlProperties = serde_yaml::to_string(&todoProperties).unwrap();

        properties.push_str("---\n");
        properties.push_str(&yamlProperties);
        properties.push_str("---\n");

        /* title: */
        let commonTitle = todo.getProperty("title".to_string());

        if commonTitle != "" {
            markdown.push_str(&format!("# {}\n", commonTitle));
        } else {
            let defaultTitle = String::from("ToDo");
            markdown.push_str(&format!("# {}\n", defaultTitle));
        }

        /* items: */
        items.push_str("## Items\n");
        items.push_str("\n");
        for item in todo.getItems() {
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
