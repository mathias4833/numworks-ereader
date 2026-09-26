use crate::epub::EpubBook;
use book_format::BookBuilder;
use book_format::{
    COMPACT_COVER_BYTE_LEN, COMPACT_COVER_HEIGHT, COMPACT_COVER_WIDTH, COVER_BYTE_LEN,
    COVER_HEIGHT, COVER_WIDTH,
};
use deunicode::deunicode;
use image::imageops::FilterType;

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

    pub fn convert(&self, book: &EpubBook) -> BookBuilder {
        let pages = book
            .chapters
            .iter()
            .flat_map(|chapter| self.paginate(chapter))
            .collect();
        let builder = BookBuilder::new(deunicode(&book.title), deunicode(&book.author), pages);
        match book
            .cover
            .as_deref()
            .and_then(|bytes| image::load_from_memory(bytes).ok())
        {
            Some(image) => builder.with_covers(
                convert_cover::<COVER_BYTE_LEN>(&image, COVER_WIDTH, COVER_HEIGHT),
                convert_cover::<COMPACT_COVER_BYTE_LEN>(
                    &image,
                    COMPACT_COVER_WIDTH,
                    COMPACT_COVER_HEIGHT,
                ),
            ),
            None => builder,
        }
    }

    fn paginate(&self, text: &str) -> Vec<String> {
        let mut lines = Vec::new();

        for line in text.lines() {
            if line.trim().is_empty() {
                if !lines.is_empty() && !lines.last().is_some_and(String::is_empty) {
                    lines.push(String::new());
                }
            } else {
                self.wrap_paragraph(&deunicode(line), &mut lines);
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

fn convert_cover<const N: usize>(
    image: &image::DynamicImage,
    width: usize,
    height: usize,
) -> [u8; N] {
    let image = image
        .resize_to_fill(width as u32, height as u32, FilterType::Lanczos3)
        .to_rgb8();
    let mut cover = [0; N];

    for (x, y, pixel) in image.enumerate_pixels() {
        let [red, green, blue] = pixel.0;
        let rgb565 = ((red as u16 >> 3) << 11) | ((green as u16 >> 2) << 5) | (blue as u16 >> 3);
        let offset = ((y as usize * width) + x as usize) * 2;
        cover[offset..offset + 2].copy_from_slice(&rgb565.to_le_bytes());
    }

    cover
}
