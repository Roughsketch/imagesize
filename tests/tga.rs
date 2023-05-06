#[cfg(test)]
use imagesize::{size, ImageSize};

#[test]
fn exr_test() {
    let dim = size("tests/images/tga/test.tga").unwrap();
    assert_eq!(dim, ImageSize { width: 100, height: 67 });
}
