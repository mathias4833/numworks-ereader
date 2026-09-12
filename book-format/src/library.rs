use crate::utils::{Offsets, Reader};
use crate::{Book, LIBRARY_MAGIC, ParseError, VERSION};

pub struct Library<'a> {
    book_offsets: Offsets<'a>,
    books: &'a [u8],
}

impl<'a> Library<'a> {
    pub fn parse(data: &'a [u8]) -> Result<Self, ParseError> {
        let mut reader = Reader::new(data);

        let magic = reader
            .take(LIBRARY_MAGIC.len())
            .ok_or(ParseError::TooShort)?;
        if magic != LIBRARY_MAGIC {
            return Err(ParseError::InvalidMagic);
        }

        let version = reader.u16().ok_or(ParseError::TooShort)?;
        if version != VERSION {
            return Err(ParseError::UnsupportedVersion);
        }

        let book_count = reader.u32().ok_or(ParseError::TooShort)? as usize;
        let book_offsets =
            Offsets::read(&mut reader, book_count).ok_or(ParseError::InvalidBookOffsets)?;

        let books = reader.remaining();

        Ok(Self {
            book_offsets,
            books,
        })
    }

    pub const fn book_count(&self) -> usize {
        self.book_offsets.len()
    }

    pub fn book(&self, index: usize) -> Result<Option<Book<'a>>, ParseError> {
        let Some(range) = self.book_offsets.range(index) else {
            return Ok(None);
        };

        let Some(data) = self.books.get(range) else {
            return Ok(None);
        };

        Book::parse(data).map(Some)
    }
}
