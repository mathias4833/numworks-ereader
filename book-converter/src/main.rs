mod converter;
mod epub;

use crate::converter::Converter;
use crate::epub::EpubBook;
use book_format::LibraryBuilder;
use std::path::Path;
use std::{env, fs};

const CHARS_PER_LINE: usize = 44;
const LINES_PER_PAGE: usize = 11;

fn main() {
    let inputs: Vec<String> = env::args().skip(1).collect();
    if inputs.is_empty() {
        eprintln!("Usage: book-converter <book1.epub> <book2.epub> ...");
        std::process::exit(1);
    }

    let converter = Converter::new(CHARS_PER_LINE, LINES_PER_PAGE);

    let books = inputs
        .iter()
        .map(|input| converter.convert(&EpubBook::open(Path::new(input))))
        .collect();

    let library = LibraryBuilder::new(books);

    let data = library.encode().expect("failed to encode library");

    fs::write("assets/library.nwlib", data).expect("failed to write output");
}
