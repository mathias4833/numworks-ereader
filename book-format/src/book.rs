use crate::utils::{OffsetTable, Reader};
use crate::{BOOK_MAGIC, COVER_BYTE_LEN, ParseError, VERSION};

pub struct Book<'a> {
    title: &'a str,
    cover: &'a [u8],
    page_offsets: OffsetTable<'a>,
    pages: &'a [u8],
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

        let cover = reader.take(COVER_BYTE_LEN).ok_or(ParseError::TooShort)?;

        let page_offsets =
            OffsetTable::read(&mut reader, page_count).ok_or(ParseError::InvalidPageOffsets)?;

        Ok(Self {
            title,
            cover,
            page_offsets,
            pages: reader.remaining(),
        })
    }

    pub const fn title(&self) -> &'a str {
        self.title
    }

    pub const fn cover(&self) -> &'a [u8] {
        self.cover
    }

    pub const fn page_count(&self) -> usize {
        self.page_offsets.len()
    }

    pub fn page(&self, index: usize) -> Option<Page<'a>> {
        let range = self.page_offsets.range(index)?;
        let text = str::from_utf8(self.pages.get(range)?).ok()?;

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
