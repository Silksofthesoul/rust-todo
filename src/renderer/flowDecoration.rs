use super::renderer::Renderer;
use crate::library::math;

// DESCRIPTION:
// Управление стилями текста в консоли.

impl Renderer {
    // Flow: decorations
    //pub fn topLine(&mut self) -> &mut Self {
    //    let buffer = self.buffer.clone();
    //    let mut sizes: Vec<i32> = Vec::new();
    //    for item in &buffer {
    //        sizes.push(item.len() as i32);
    //    }
    //    let mut padding = String::from("");
    //    padding.push_str(&self.hSpacer.repeat(self.paddingLeft));
    //    let max = math::max(sizes);
    //    let mut topLine = self.hSymbolFrame.repeat(max as usize);
    //    topLine.push_str("\n");
    //    topLine.push_str(&self.vSpacer.repeat(self.paddingBottom));
    //    topLine.push_str(&padding);
    //    self.buffer.insert(0, topLine);
    //    self
    //}
    //pub fn bottomLine(&mut self) -> &mut Self {
    //    let buffer = self.buffer.clone();
    //    let mut sizes: Vec<i32> = Vec::new();
    //    for item in &buffer {
    //        sizes.push(item.len() as i32);
    //    }
    //    let max = math::max(sizes);
    //    let mut bottomLine = self.hSymbolFrame.repeat((max - 1) as usize);
    //    bottomLine.insert_str(0, "\n");
    //    bottomLine.insert_str(0, &self.vSpacer.repeat(self.paddingBottom));
    //    self.buffer.push(bottomLine);
    //    self
    //}
}
