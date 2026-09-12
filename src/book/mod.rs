use book_format::{Book, ParseError};

pub fn load() -> Result<Book<'static>, ParseError> {
    Book::parse(crate::eadk::external_data::get())
}
