#![no_std]

pub struct Book<'a> {
    title: &'a str,
    pages: &'a [Page<'a>],
}

impl<'a> Book<'a> {
    pub const fn new(title: &'a str, pages: &'a [Page<'a>]) -> Self {
        Self { title, pages }
    }

    pub const fn title(&self) -> &'a str {
        self.title
    }

    pub const fn page_count(&self) -> usize {
        self.pages.len()
    }

    pub fn page(&self, index: usize) -> Option<&Page<'a>> {
        self.pages.get(index)
    }
}

pub struct Page<'a> {
    lines: &'a [&'a str],
}

impl<'a> Page<'a> {
    pub const fn new(lines: &'a [&'a str]) -> Self {
        Self { lines }
    }

    pub const fn lines(&self) -> &'a [&'a str] {
        self.lines
    }
}
