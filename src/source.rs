#[derive(Debug, Clone)]
pub struct SourceCodeLocation {
    pub text: String,
    pub line: usize,
    pub start_char: usize,
    pub end_char: usize,
}

impl SourceCodeLocation {
    pub fn new(text: String, line: usize, start_char: usize, end_char: usize) -> Self {
        return SourceCodeLocation {
            text,
            line,
            start_char,
            end_char,
        };
    }
}

