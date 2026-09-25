use crate::utils::Writer;
use crate::{BOOK_MAGIC, COVER_BYTE_LEN, EncodeError, LIBRARY_MAGIC, VERSION};
use std::string::String;
use std::vec::Vec;

const DEFAULT_COVER: &[u8; COVER_BYTE_LEN] = include_bytes!("../../assets/default-cover.rgb565");

pub struct BookBuilder {
    title: String,
    author: String,
    pages: Vec<String>,
    cover: Option<[u8; COVER_BYTE_LEN]>,
}

impl BookBuilder {
    pub fn new(title: String, author: String, pages: Vec<String>) -> Self {
        Self {
            title,
            author,
            pages,
            cover: None,
        }
    }

    pub fn with_cover(mut self, cover: [u8; COVER_BYTE_LEN]) -> Self {
        self.cover = Some(cover);
        self
    }

    pub fn encode(&self) -> Result<Vec<u8>, EncodeError> {
        let mut writer = Writer::new();
        self.write_to(&mut writer)?;
        Ok(writer.finish())
    }

    fn write_to(&self, writer: &mut Writer) -> Result<(), EncodeError> {
        writer.bytes(BOOK_MAGIC);
        writer.u16(VERSION);
        writer.u32(self.pages.len())?;
        writer.u32(self.title.len())?;
        writer.u32(self.author.len())?;

        writer.bytes(self.title.as_bytes());
        writer.bytes(self.author.as_bytes());
        writer.bytes(self.cover.as_ref().unwrap_or(DEFAULT_COVER));

        writer.indexed(&self.pages, |writer, page| {
            writer.bytes(page.as_bytes());
            Ok(())
        })?;

        Ok(())
    }
}

pub struct LibraryBuilder {
    books: Vec<BookBuilder>,
}

impl LibraryBuilder {
    pub fn new(books: Vec<BookBuilder>) -> Self {
        Self { books }
    }

    pub fn encode(&self) -> Result<Vec<u8>, EncodeError> {
        let mut writer = Writer::new();

        writer.bytes(LIBRARY_MAGIC);
        writer.u16(VERSION);
        writer.u32(self.books.len())?;

        writer.indexed(&self.books, |writer, book| book.write_to(writer))?;

        Ok(writer.finish())
    }
}
