use super::renderer::Renderer;

// DESCRIPTION:
// Управление стилями текста в консоли.

impl Renderer {
    // Flow: foregrounds
    pub fn red(&mut self) -> &mut Self {
        self.bufferLine.push(self.getColor("red"));
        self
    }
    pub fn green(&mut self) -> &mut Self {
        self.bufferLine.push(self.getColor("green"));
        self
    }
    pub fn yellow(&mut self) -> &mut Self {
        self.bufferLine.push(self.getColor("yellow"));
        self
    }
    pub fn blue(&mut self) -> &mut Self {
        self.bufferLine.push(self.getColor("blue"));
        self
    }
    pub fn magenta(&mut self) -> &mut Self {
        self.bufferLine.push(self.getColor("magenta"));
        self
    }
    pub fn cyan(&mut self) -> &mut Self {
        self.bufferLine.push(self.getColor("cyan"));
        self
    }
    pub fn white(&mut self) -> &mut Self {
        self.bufferLine.push(self.getColor("white"));
        self
    }
    pub fn lightGray(&mut self) -> &mut Self {
        self.bufferLine.push(self.getColor("lightGray"));
        self
    }
    pub fn lightRed(&mut self) -> &mut Self {
        self.bufferLine.push(self.getColor("lightRed"));
        self
    }
    pub fn lightGreen(&mut self) -> &mut Self {
        self.bufferLine.push(self.getColor("lightGreen"));
        self
    }
    pub fn lightYellow(&mut self) -> &mut Self {
        self.bufferLine.push(self.getColor("lightYellow"));
        self
    }
    pub fn lightBlue(&mut self) -> &mut Self {
        self.bufferLine.push(self.getColor("lightBlue"));
        self
    }
    pub fn lightMagenta(&mut self) -> &mut Self {
        self.bufferLine.push(self.getColor("lightMagenta"));
        self
    }
    pub fn lightCyan(&mut self) -> &mut Self {
        self.bufferLine.push(self.getColor("lightCyan"));
        self
    }
    pub fn lightWhite(&mut self) -> &mut Self {
        self.bufferLine.push(self.getColor("lightWhite"));
        self
    }
    pub fn reset(&mut self) -> &mut Self {
        self.bufferLine.push(self.getAttribute("reset"));
        self
    }
}
