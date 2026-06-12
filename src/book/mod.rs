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

const ALICE_PAGE_1_LINES: [&str; 11] = [
    "Alice was beginning to get very tired of",
    "sitting by her sister on the bank, and of",
    "having nothing to do: once or twice she had",
    "peeped into the book her sister was reading,",
    "but it had no pictures or conversations in it,",
    "\"and what is the use of a book,\" thought",
    "Alice \"without pictures or conversations?\" So",
    "she was considering in her own mind (as well",
    "as she could, for the hot day made her feel",
    "very sleepy and stupid), whether the pleasure",
    "of making a daisy-chain would be worth the",
];

const ALICE_PAGE_2_LINES: [&str; 11] = [
    "trouble of getting up and picking the",
    "daisies, when suddenly a White Rabbit with",
    "pink eyes ran close by her.",
    "There was nothing so very remarkable in",
    "that; nor did Alice think it so very much",
    "out of the way to hear the Rabbit say to",
    "itself, \"Oh dear! Oh dear! I shall be too",
    "late!\" (when she thought it over afterwards,",
    "it occurred to her that she ought to have",
    "wondered at this, but at the time it all",
    "seemed quite natural);",
];

const ALICE_PAGE_3_LINES: [&str; 11] = [
    "but when the Rabbit actually took a watch",
    "out of its waistcoat-pocket, and looked at",
    "it, and then hurried on, Alice started to",
    "her feet, for it flashed across her mind",
    "that she had never before seen a rabbit with",
    "either a waistcoat-pocket, or a watch to",
    "take out of it, and burning with curiosity,",
    "she ran across the field after it, and",
    "fortunately was just in time to see it pop",
    "down a large rabbit-hole under the hedge.",
    "In another moment down went Alice after it,",
];

const ALICE_PAGES: [Page<'static>; 3] = [
    Page::new(&ALICE_PAGE_1_LINES),
    Page::new(&ALICE_PAGE_2_LINES),
    Page::new(&ALICE_PAGE_3_LINES),
];

pub static ALICE_IN_WONDERLAND: Book<'static> =
    Book::new("Alice in Wonderland", &ALICE_PAGES);
