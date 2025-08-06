#[cfg(test)]
use imagesize::{size, ImageSize};

#[test]
#[cfg(feature = "ktx2")]
fn ktx2_test() {
    let dim = size("tests/images/ktx2/test.ktx2").unwrap();
    assert_eq!(
        dim,
        ImageSize {
            width: 256,
            height: 256
        }
    );
}
