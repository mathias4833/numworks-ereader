use heapless::Vec;

const MAX_BOOKS: usize = 32;

#[derive(Clone, Copy, Default)]
struct BookProgress {
    page_index: usize,
}

pub struct ReadingState {
    current_book: usize,
    progress: Vec<BookProgress, MAX_BOOKS>,
}

impl ReadingState {
    pub fn new(book_count: usize) -> Option<Self> {
        if book_count == 0 || book_count > MAX_BOOKS {
            return None;
        }
        let progress = (0..book_count).map(|_| BookProgress::default()).collect();

        Some(Self {
            current_book: 0,
            progress,
        })
    }

    pub const fn current_book(&self) -> usize {
        self.current_book
    }

    pub fn current_page(&self) -> usize {
        self.progress[self.current_book].page_index
    }

    pub fn select_book(&mut self, index: usize) -> bool {
        if index >= self.progress.len() {
            return false;
        }

        self.current_book = index;
        true
    }

    pub fn previous_page(&mut self) -> bool {
        let progress = &mut self.progress[self.current_book];
        if progress.page_index == 0 {
            return false;
        }

        progress.page_index -= 1;
        true
    }

    pub fn next_page(&mut self, page_count: usize) -> bool {
        let progress = &mut self.progress[self.current_book];
        if progress.page_index + 1 >= page_count {
            return false;
        }

        progress.page_index += 1;
        true
    }

    pub fn progress_percent(&self, page_count: usize) -> usize {
        if page_count == 0 {
            return 0;
        }

        (self.current_page() + 1) * 100 / page_count
    }
}
