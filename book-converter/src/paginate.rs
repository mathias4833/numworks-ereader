const CHARS_PER_LINE: usize = 44;
const LINES_PER_PAGE: usize = 11;

pub fn paginate(text: &str) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current_line = String::new();

    for word in text.split_whitespace() {
        // If the line is empty, we don't need to add a space before the word
        let new_len = current_line.len() + usize::from(!current_line.is_empty()) + word.len();
        if new_len > CHARS_PER_LINE {
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

    lines
        .chunks(LINES_PER_PAGE)
        .map(|lines| lines.join("\n"))
        .collect()
}
