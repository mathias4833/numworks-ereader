mod converter;

use crate::converter::Converter;
use std::{env, fs};

const CHARS_PER_LINE: usize = 44;
const LINES_PER_PAGE: usize = 11;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: book-converter <input.txt> <output.nwbook>");
        std::process::exit(1);
    }

    let input = &args[1];
    let output = &args[2];

    let text = fs::read_to_string(input).expect("failed to read input file");
    let title = std::path::Path::new(input)
        .file_stem()
        .unwrap()
        .to_string_lossy()
        .into_owned();

    let data = Converter::new(CHARS_PER_LINE, LINES_PER_PAGE)
        .convert(title, &text)
        .expect("failed to convert");

    fs::write(output, data).unwrap();
}
