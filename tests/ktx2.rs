#[cfg(test)]
use imagesize::{size, ImageSize};

#[test]
fn ktx2_test() {
    let dim = size("tests/images/ktx2/test.ktx2").unwrap();
    assert_eq!(dim, ImageSize { width: 1, height: 256 });
}
