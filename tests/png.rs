#[cfg(test)]
use imagesize::{size, ImageSize};

#[test]
#[cfg(feature = "png")]
fn apng_test() {
    let dim = size("tests/images/png/test.apng").unwrap();
    assert_eq!(
        dim,
        ImageSize {
            width: 100,
            height: 100
        }
    );
}

#[test]
#[cfg(feature = "png")]
fn png_test() {
    let dim = size("tests/images/png/test.png").unwrap();
    assert_eq!(
        dim,
        ImageSize {
            width: 690,
            height: 298
        }
    );
}
