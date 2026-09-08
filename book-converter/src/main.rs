mod paginate;

use crate::paginate::paginate;
use book_format::BookBuilder;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: book-converter <input.txt> <output.nwbook>");
        std::process::exit(1);
    }

    let input = &args[1];
    let output = &args[2];

    let text = fs::read_to_string(input).expect("failed to read input file");
    let pages = paginate(&text);

    let title = std::path::Path::new(input)
        .file_stem()
        .unwrap()
        .to_string_lossy()
        .into_owned();

    let book = BookBuilder::new(title, pages);
    let data = book.encode().unwrap();

    std::fs::write(output, data).unwrap();
}
