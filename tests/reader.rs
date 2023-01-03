#[cfg(test)]
use imagesize::reader_size;
use std::io::Cursor;

#[test]
fn reader_test() {
    // PNG Header with size 123x321
    let reader = Cursor::new([
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A,
        0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52,
        0x00, 0x00, 0x00, 0x7B, 0x00, 0x00, 0x01, 0x41,
        0x08, 0x06, 0x00, 0x00, 0x00, 0x9A, 0x38, 0xC4
    ]);
    let dim = reader_size(reader).unwrap();
    assert_eq!(dim.width, 123);
    assert_eq!(dim.height, 321);
}

#[test]
fn reader_test_fail() {
    // only header part of webp
    let webp_reader = Cursor::new([
        0x52, 0x49, 0x46, 0x46, 0xD8, 0xA1, 0x00, 0x00,
        0x57, 0x45, 0x42, 0x50, 0x56, 0x50, 0x38, 0x58
    ]);
    assert!(reader_size(webp_reader).is_err());
}