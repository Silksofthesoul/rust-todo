use super::renderer::Renderer;

// DESCRIPTION:
// Управление стилями текста в консоли.
// Методы берут по ключу из коллекций: foreground, background, attributes.
// Если ключ не найден, возвращается стандартный цвет или атрибут.

impl Renderer {
    pub fn color(&self, text: &str, key: &str) -> String {
        let mut result = String::from("");
        if let Some(color) = self.foreground.get(key) {
            result.push_str(color);
        }
        result.push_str(text);
        if let Some(color) = self.foreground.get("default") {
            result.push_str(color);
        }
        result
    }

    pub fn getColor(&self, key: &str) -> String {
        self.foreground.get(key).unwrap().to_string()
    }

    pub fn background(&self, text: &str, key: &str) -> String {
        let mut result = String::from("");
        if let Some(color) = self.background.get(key) {
            result.push_str(color);
        }
        result.push_str(text);
        if let Some(color) = self.background.get("default") {
            result.push_str(color);
        }
        result
    }

    pub fn getBackground(&self, key: &str) -> String {
        self.background.get(key).unwrap().to_string()
    }

    pub fn attribute(&self, text: &str, key: &str) -> String {
        let mut result = String::from("");
        if let Some(color) = self.attributes.get(key) {
            result.push_str(color);
        }
        result.push_str(text);
        if let Some(color) = self.attributes.get("reset") {
            result.push_str(color);
        }
        result
    }
    pub fn getAttribute(&self, key: &str) -> String {
        self.attributes.get(key).unwrap().to_string()
    }
}
