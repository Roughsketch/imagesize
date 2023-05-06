#[cfg(test)]
use imagesize::{size, ImageSize};

#[test]
fn qoi_test() {
    let dim = size("tests/images/qoi/test.qoi").unwrap();
    assert_eq!(dim, ImageSize { width: 800, height: 600 });
}
