#![cfg_attr(not(feature = "std"), no_std)]

mod book;
mod utils;

#[cfg(feature = "std")]
mod builder;
mod library;

const BOOK_MAGIC: &[u8; 4] = b"NWBK";
const LIBRARY_MAGIC: &[u8; 4] = b"NWLB";
const VERSION: u16 = 1;

pub const COVER_WIDTH: usize = 60;
pub const COVER_HEIGHT: usize = 90;
pub const COVER_BYTE_LEN: usize = COVER_WIDTH * COVER_HEIGHT * 2;

#[derive(Debug)]
pub enum ParseError {
    TooShort,
    InvalidMagic,
    UnsupportedVersion,
    InvalidTitle,
    InvalidPageOffsets,
    InvalidPageData,
    InvalidBookOffsets,
}

#[cfg(feature = "std")]
#[derive(Debug)]
pub enum EncodeError {
    TooLarge,
}

pub use book::{Book, Page};
pub use library::Library;

#[cfg(feature = "std")]
pub use builder::{BookBuilder, LibraryBuilder};
