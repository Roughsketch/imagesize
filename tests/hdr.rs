#[cfg(test)]
use imagesize::{size, ImageSize};

#[test]
fn hdr_test() {
    let dim = size("tests/images/hdr/test.hdr").unwrap();
    assert_eq!(dim, ImageSize { width: 100, height: 67 });
}
