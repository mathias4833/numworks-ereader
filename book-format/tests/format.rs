#![cfg(feature = "std")]

use book_format::{
    Book, BookBuilder, COMPACT_COVER_BYTE_LEN, COVER_BYTE_LEN, Library, LibraryBuilder, ParseError,
};

#[test]
fn library_round_trip_preserves_books_and_pages() {
    let data = LibraryBuilder::new(vec![
        BookBuilder::new(
            "Été".into(),
            "Zoë".into(),
            vec!["First\npage".into(), "".into(), "Fin été".into()],
        )
        .with_covers([0x12; COVER_BYTE_LEN], [0x34; COMPACT_COVER_BYTE_LEN]),
        BookBuilder::new("Second".into(), "".into(), vec!["Last".into()]),
    ])
    .encode()
    .unwrap();
    let library = Library::parse(&data).unwrap();

    assert_eq!(library.book_count(), 2);
    let first = library.book(0).unwrap().unwrap();
    assert_eq!(first.title(), "Été");
    assert_eq!(first.author(), "Zoë");
    assert_eq!(first.cover(), [0x12; COVER_BYTE_LEN]);
    assert_eq!(first.compact_cover(), [0x34; COMPACT_COVER_BYTE_LEN]);
    assert_eq!(first.page_count(), 3);
    for (index, expected) in ["First\npage", "", "Fin été"].into_iter().enumerate() {
        assert_eq!(first.page(index).unwrap().text(), expected);
    }
    assert!(first.page(3).is_none());

    let second = library.book(1).unwrap().unwrap();
    assert_eq!(second.title(), "Second");
    assert_eq!(second.author(), "");
    assert_eq!(second.page_count(), 1);
    assert_eq!(second.page(0).unwrap().text(), "Last");
    assert!(library.book(2).unwrap().is_none());
}

#[test]
fn empty_library_has_no_books() {
    let data = LibraryBuilder::new(vec![]).encode().unwrap();
    // Magic, version, book count, and the final zero offset
    assert_eq!(data, b"NWLB\x02\0\0\0\0\0\0\0\0\0");
    let library = Library::parse(&data).unwrap();
    assert_eq!(library.book_count(), 0);
    assert!(library.book(0).unwrap().is_none());
}

#[test]
fn invalid_headers_are_rejected() {
    assert!(matches!(
        Book::parse(b"NOPE"),
        Err(ParseError::InvalidMagic)
    ));
    assert!(matches!(
        Library::parse(b"NOPE"),
        Err(ParseError::InvalidMagic)
    ));
    assert!(matches!(
        Book::parse(b"NWBK\xff\xff"),
        Err(ParseError::UnsupportedVersion)
    ));
    assert!(matches!(
        Library::parse(b"NWLB\xff\xff"),
        Err(ParseError::UnsupportedVersion)
    ));
}

#[test]
fn truncated_library_never_exposes_a_complete_page() {
    let data = LibraryBuilder::new(vec![BookBuilder::new(
        "Title".into(),
        "Author".into(),
        vec!["Page".into()],
    )])
    .encode()
    .unwrap();

    for end in 0..data.len() {
        if let Ok(library) = Library::parse(&data[..end]) {
            if let Ok(Some(book)) = library.book(0) {
                assert!(book.page(0).is_none(), "accepted truncation at {end}");
            }
        }
    }
}

#[test]
fn invalid_page_ranges_and_text_are_not_readable() {
    let data = BookBuilder::new("".into(), "".into(), vec!["abc".into()])
        .encode()
        .unwrap();

    // The fixed header and covers precede the two page offsets
    let offsets = 18 + COVER_BYTE_LEN + COMPACT_COVER_BYTE_LEN;
    for (start, end) in [(2u32, 1u32), (0, 4), (0, u32::MAX)] {
        let mut corrupted = data.clone();
        corrupted[offsets..offsets + 4].copy_from_slice(&start.to_le_bytes());
        corrupted[offsets + 4..offsets + 8].copy_from_slice(&end.to_le_bytes());
        assert!(Book::parse(&corrupted).unwrap().page(0).is_none());
    }

    let mut corrupted = data;
    *corrupted.last_mut().unwrap() = 0xff;
    assert!(Book::parse(&corrupted).unwrap().page(0).is_none());
}
