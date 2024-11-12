#[cfg(test)]
use imagesize::{size, ImageSize};

#[test]
#[cfg(feature = "bmp")]
fn bmp_test() {
    let dim = size("tests/images/bmp/test.bmp").unwrap();
    assert_eq!(
        dim,
        ImageSize {
            width: 512,
            height: 512
        }
    );
}
