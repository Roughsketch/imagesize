#[cfg(test)]
use imagesize::{size, ImageSize};

#[test]
fn avif_test() {
    let dim = size("tests/images/avif/test.avif").unwrap();
    assert_eq!(dim, ImageSize { width: 1204, height: 800 });
}

#[test]
fn avif_multi_picks_largest() {
    let dim = size("tests/images/avif/test.avifs").unwrap();
    assert_eq!(dim, ImageSize { width: 159, height: 159 });
}
