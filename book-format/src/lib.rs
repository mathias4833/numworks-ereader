#![cfg_attr(not(feature = "std"), no_std)]

mod book;
mod utils;

#[cfg(feature = "std")]
mod builder;

#[derive(Debug)]
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

#[cfg(feature = "std")]
pub use builder::BookBuilder;
