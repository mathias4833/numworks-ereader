use crate::utils::{Reader, read_u32};
use crate::{MAGIC, ParseError, VERSION};

pub struct Book<'a> {
    title: &'a str,
    page_offsets: &'a [u8],
    pages: &'a str,
}

impl<'a> Book<'a> {
    pub fn parse(data: &'a [u8]) -> Result<Self, ParseError> {
        let mut reader = Reader::new(data);

        let magic = reader.take(MAGIC.len()).ok_or(ParseError::TooShort)?;
        if magic != MAGIC {
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

        let offsets_len = page_count
            .checked_add(1) // +1 for the last page offset
            .and_then(|n| n.checked_mul(size_of::<u32>()))
            .ok_or(ParseError::InvalidPageOffsets)?;

        let page_offsets = reader
            .take(offsets_len)
            .ok_or(ParseError::InvalidPageOffsets)?;

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
        self.page_offsets.len() / size_of::<u32>() - 1
    }

    pub fn page(&self, index: usize) -> Option<Page<'a>> {
        if index >= self.page_count() {
            return None;
        }

        let start = read_u32(self.page_offsets, index * size_of::<u32>())? as usize;
        let end = read_u32(self.page_offsets, (index + 1) * size_of::<u32>())? as usize;

        let text = self.pages.get(start..end)?;
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
