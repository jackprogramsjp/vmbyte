pub struct VmbTranspiler {
    text: String
}

impl VmbTranspiler {
    pub fn new(text: String) -> Self {
        Self {
            text
        }
    }

    pub fn change(&mut self, text: string) {
        self.text = text;
    }

    pub fn transpile()
}
