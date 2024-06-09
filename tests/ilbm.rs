#[cfg(test)]
use imagesize::{size, ImageSize};

#[test]
fn ilbm_test() {
    assert_eq!(
        size("tests/images/ilbm/test.iff").unwrap(),
        ImageSize {
            width: 640,
            height: 512
        }
    );
}
