#![no_std]

mod book;
mod utils;

pub enum ParseError {
    TooShort,
    InvalidMagic,
    UnsupportedVersion,
    InvalidTitle,
    InvalidPageOffsets,
    InvalidPageData,
}

pub const MAGIC: &[u8; 4] = b"NWBK";
pub const VERSION: u16 = 1;

pub use book::{Book, Page};
