#![cfg_attr(not(feature = "std"), no_std)]

mod book;
mod utils;

#[cfg(feature = "std")]
mod builder;

pub const MAGIC: &[u8; 4] = b"NWBK";
pub const VERSION: u16 = 1;

pub use book::{Book, Page, ParseError};

#[cfg(feature = "std")]
pub use builder::{BookBuilder, EncodeError};
