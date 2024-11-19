#[cfg(test)]
use imagesize::{size, ImageSize};

#[test]
#[cfg(feature = "ilbm")]
fn ilbm_test() {
    assert_eq!(
        size("tests/images/ilbm/test.iff").unwrap(),
        ImageSize {
            width: 640,
            height: 512
        }
    );
}
