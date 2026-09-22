mod converter;
mod epub;

use book_format::{EncodeError, LibraryBuilder};
use converter::Converter;
use epub::EpubBook;
use std::path::PathBuf;

const CHARS_PER_LINE: usize = 39;
const LINES_PER_PAGE: usize = 13;

pub struct LibraryConverter {
    converter: Converter,
}

impl Default for LibraryConverter {
    fn default() -> Self {
        Self::new()
    }
}

impl LibraryConverter {
    pub const fn new() -> Self {
        Self {
            converter: Converter::new(CHARS_PER_LINE, LINES_PER_PAGE),
        }
    }

    pub fn convert(&self, inputs: &[PathBuf]) -> Result<Vec<u8>, EncodeError> {
        let books = inputs
            .iter()
            .map(|path| {
                let epub = EpubBook::open(path);
                self.converter.convert(&epub)
            })
            .collect();

        LibraryBuilder::new(books).encode()
    }
}
