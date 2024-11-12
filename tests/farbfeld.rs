#[cfg(test)]
use imagesize::{size, ImageSize};

#[test]
#[cfg(feature = "farbfeld")]
fn farbfeld_test() {
    let dim = size("tests/images/farbfeld/test.ff").unwrap();
    assert_eq!(
        dim,
        ImageSize {
            width: 32,
            height: 32
        }
    );
}
