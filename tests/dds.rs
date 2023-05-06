#[cfg(test)]
use imagesize::{size, ImageSize};

#[test]
fn dds_test() {
    let dim = size("tests/images/dds/test.dds").unwrap();
    assert_eq!(dim, ImageSize { width: 100, height: 67 });
}
