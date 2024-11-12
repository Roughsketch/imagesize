#[cfg(test)]
use imagesize::{size, ImageSize};

#[test]
#[cfg(feature = "exr")]
fn exr_test() {
    let dim = size("tests/images/exr/test.exr").unwrap();
    assert_eq!(
        dim,
        ImageSize {
            width: 100,
            height: 100
        }
    );
}
