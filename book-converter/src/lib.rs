mod converter;
mod epub;

use anyhow::{Context, Result, anyhow};
use book_format::LibraryBuilder;
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

    pub fn convert(&self, inputs: &[PathBuf]) -> Result<Vec<u8>> {
        let books = inputs
            .iter()
            .map(|path| {
                let epub = EpubBook::open(path)
                    .with_context(|| format!("failed to import {}", path.display()))?;

                Ok(self.converter.convert(&epub))
            })
            .collect::<Result<Vec<_>>>()?;

        LibraryBuilder::new(books)
            .encode()
            .map_err(|_| anyhow!("library is too large"))
    }
}
