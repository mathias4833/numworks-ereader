use crate::utils::{Offsets, Reader};
use crate::{BOOK_MAGIC, ParseError, VERSION};

pub struct Book<'a> {
    title: &'a str,
    page_offsets: Offsets<'a>,
    pages: &'a str,
}

impl<'a> Book<'a> {
    pub fn parse(data: &'a [u8]) -> Result<Self, ParseError> {
        let mut reader = Reader::new(data);

        let magic = reader.take(BOOK_MAGIC.len()).ok_or(ParseError::TooShort)?;
        if magic != BOOK_MAGIC {
            return Err(ParseError::InvalidMagic);
        }

        let version = reader.u16().ok_or(ParseError::TooShort)?;
        if version != VERSION {
            return Err(ParseError::UnsupportedVersion);
        }

        let page_count = reader.u32().ok_or(ParseError::TooShort)? as usize;
        let title_len = reader.u32().ok_or(ParseError::TooShort)? as usize;

        let title = str::from_utf8(reader.take(title_len).ok_or(ParseError::InvalidTitle)?)
            .map_err(|_| ParseError::InvalidTitle)?;

        let page_offsets =
            Offsets::read(&mut reader, page_count).ok_or(ParseError::InvalidPageOffsets)?;
        let pages = str::from_utf8(reader.remaining()).map_err(|_| ParseError::InvalidPageData)?;

        Ok(Self {
            title,
            page_offsets,
            pages,
        })
    }

    pub const fn title(&self) -> &'a str {
        self.title
    }

    pub const fn page_count(&self) -> usize {
        self.page_offsets.len()
    }

    pub fn page(&self, index: usize) -> Option<Page<'a>> {
        let range = self.page_offsets.range(index)?;
        let text = self.pages.get(range)?;

        Some(Page { text })
    }
}

pub struct Page<'a> {
    text: &'a str,
}

impl<'a> Page<'a> {
    pub fn text(&self) -> &'a str {
        self.text
    }

    pub fn lines(&self) -> impl Iterator<Item = &'a str> {
        self.text.lines()
    }
}
