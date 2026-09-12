use book_format::Book;

pub struct ReadingState {
    book: Book<'static>,
    book_index: usize,
    page_index: usize,
}

impl ReadingState {
    pub const fn new(book_index: usize, book: Book<'static>) -> Self {
        Self {
            book,
            book_index,
            page_index: 0,
        }
    }

    pub const fn book(&self) -> &Book<'static> {
        &self.book
    }

    pub const fn book_index(&self) -> usize {
        self.book_index
    }

    pub const fn page_index(&self) -> usize {
        self.page_index
    }

    pub fn open_book(&mut self, book_index: usize, book: Book<'static>) {
        self.book = book;
        self.book_index = book_index;
        self.page_index = 0;
    }

    pub fn previous_page(&mut self) -> bool {
        if self.page_index == 0 {
            return false;
        }

        self.page_index -= 1;
        true
    }

    pub fn next_page(&mut self) -> bool {
        if self.page_index + 1 >= self.book.page_count() {
            return false;
        }

        self.page_index += 1;
        true
    }

    pub fn progress_percent(&self) -> usize {
        let page_count = self.book.page_count();
        if page_count == 0 {
            return 0;
        }

        (self.page_index + 1) * 100 / page_count
    }
}
