#[cfg(test)]
use imagesize::{size, ImageSize};

#[test]
#[cfg(feature = "psd")]
fn psd_test() {
    let dim = size("tests/images/psd/test.psd").unwrap();
    assert_eq!(
        dim,
        ImageSize {
            width: 500,
            height: 500
        }
    );
}
