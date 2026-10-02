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

    pub fn progress_percent_for(&self, book_index: usize, page_count: usize) -> usize {
        if page_count == 0 || book_index >= self.progress.len() {
            return 0;
        }

        (self.progress[book_index].page_index + 1) * 100 / page_count
    }

    pub fn progress_percent(&self, page_count: usize) -> usize {
        self.progress_percent_for(self.current_book, page_count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn library_size_must_fit_progress_storage() {
        assert!(ReadingState::new(0).is_none());
        assert!(ReadingState::new(1).is_some());
        assert!(ReadingState::new(MAX_BOOKS).is_some());
        assert!(ReadingState::new(MAX_BOOKS + 1).is_none());
    }

    #[test]
    fn page_turns_stop_at_book_boundaries() {
        let mut state = ReadingState::new(1).unwrap();
        assert!(!state.previous_page());
        assert!(!state.next_page(0));
        assert!(!state.next_page(1));
        assert_eq!(state.current_page(), 0);
        assert_eq!(state.progress_percent(0), 0);
        assert_eq!(state.progress_percent(2), 50);

        assert!(state.next_page(2));
        assert!(!state.next_page(2));
        assert_eq!(state.current_page(), 1);
        assert_eq!(state.progress_percent(2), 100);
        assert!(state.previous_page());
        assert_eq!(state.current_page(), 0);
        assert!(!state.previous_page());
    }

    #[test]
    fn switching_books_preserves_independent_progress() {
        let mut state = ReadingState::new(2).unwrap();
        assert!(state.next_page(4));
        assert!(state.select_book(1));
        assert_eq!(state.current_page(), 0);
        assert!(state.next_page(3));
        assert!(state.next_page(3));
        assert_eq!(state.progress_percent_for(0, 4), 50);
        assert_eq!(state.progress_percent_for(1, 3), 100);

        assert!(!state.select_book(2));
        assert_eq!(state.current_book(), 1);
        assert_eq!(state.current_page(), 2);
        assert!(state.select_book(0));
        assert_eq!(state.current_page(), 1);
    }
}
