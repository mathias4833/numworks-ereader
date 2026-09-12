mod converter;

use crate::converter::Converter;
use book_format::LibraryBuilder;
use std::path::Path;
use std::{env, fs};

const CHARS_PER_LINE: usize = 44;
const LINES_PER_PAGE: usize = 11;

fn main() {
    let inputs: Vec<String> = env::args().skip(1).collect();
    if inputs.is_empty() {
        eprintln!("Usage: library-converter <book1.txt> <book2.txt> ...");
        std::process::exit(1);
    }

    let converter = Converter::new(CHARS_PER_LINE, LINES_PER_PAGE);

    let books = inputs
        .iter()
        .map(|input| {
            let text = fs::read_to_string(input).expect("failed to read input");

            let title = Path::new(input)
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .into_owned();

            converter.convert(title, &text)
        })
        .collect();

    let library = LibraryBuilder::new(books);

    let data = library.encode().expect("failed to encode library");

    fs::write("assets/library.nwlib", data).expect("failed to write output");
}
