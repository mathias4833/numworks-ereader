use book_format::{Book, ParseError};

#[cfg(not(target_os = "none"))]
pub fn load() -> Result<Book<'static>, ParseError> {
    Book::parse(include_bytes!("../../assets/alice.nwbook"))
}
