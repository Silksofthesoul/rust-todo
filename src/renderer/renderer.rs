pub struct Renderer {
    // buffers:
    pub bufferLine: Vec<String>,
    pub bufferRow: Vec<String>,
    pub bufferTable: Vec<Vec<String>>,

    // symbols: frames
    pub hSymbolFrame: String,
    pub vSymbolFrame: String,

    // symbols: spacers
    pub hSpacer: String,
    pub vSpacer: String,

    // params: padding
    pub paddingLeft: usize,
    pub paddingRight: usize,
    pub paddingTop: usize,
    pub paddingBottom: usize,
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
            // buffers:
            bufferLine: Vec::new(),
            bufferRow: Vec::new(),
            bufferTable: Vec::new(),

            // symbols: frames
            hSymbolFrame: String::from("="),
            vSymbolFrame: String::from("|"),

            // symbols: spacers
            hSpacer: String::from(" "),
            vSpacer: String::from("\n\n"),

            // params: padding
            paddingLeft: 2,
            paddingRight: 2,
            paddingTop: 0,
            paddingBottom: 0,
        }
    }
}
