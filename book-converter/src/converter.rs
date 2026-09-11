use book_format::{BookBuilder, EncodeError};
use deunicode::deunicode;

pub struct Converter {
    chars_per_line: usize,
    lines_per_page: usize,
}

impl Converter {
    pub const fn new(chars_per_line: usize, lines_per_page: usize) -> Self {
        Self {
            chars_per_line,
            lines_per_page,
        }
    }

    pub fn convert(&self, title: String, text: &str) -> Result<Vec<u8>, EncodeError> {
        let text = deunicode(text).replace("\r\n", "\n");
        let pages = self.paginate(&text);

        BookBuilder::new(title, pages).encode()
    }

    fn paginate(&self, text: &str) -> Vec<String> {
        let mut lines = Vec::new();

        for paragraph in text.split("\n\n").filter(|p| !p.trim().is_empty()) {
            self.wrap_paragraph(paragraph, &mut lines);

            if !lines.is_empty() {
                lines.push(String::new());
            }
        }

        if lines.last().is_some_and(String::is_empty) {
            lines.pop();
        }

        lines
            .chunks(self.lines_per_page)
            .map(|page| page.join("\n"))
            .collect()
    }

    fn wrap_paragraph(&self, paragraph: &str, lines: &mut Vec<String>) {
        let mut current_line = String::new();

        for word in paragraph.split_whitespace() {
            // If the line is empty, we don't need to add a space before the word
            let new_len = current_line.len() + usize::from(!current_line.is_empty()) + word.len();
            if new_len > self.chars_per_line {
                lines.push(current_line);
                current_line = String::new();
            }

            if !current_line.is_empty() {
                current_line.push(' ');
            }
            current_line.push_str(word);
        }

        if !current_line.is_empty() {
            lines.push(current_line);
        }
    }
}
