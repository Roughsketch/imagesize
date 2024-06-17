#[cfg(test)]
use imagesize::{size, ImageSize};

#[test]
fn ico_test() {
    let dim = size("tests/images/ico/test.ico").unwrap();
    assert_eq!(
        dim,
        ImageSize {
            width: 16,
            height: 16
        }
    );
}

#[test]
fn max_size_test() {
    let dim = size("tests/images/ico/max_width.ico").unwrap();
    assert_eq!(
        dim,
        ImageSize {
            width: 256,
            height: 255
        }
    );
}

#[test]
fn multiple_test() {
    // Contains 48x48, 32x32, and 16x16 versions of the same image
    let dim = size("tests/images/ico/multiple.ico").unwrap();
    assert_eq!(
        dim,
        ImageSize {
            width: 48,
            height: 48
        }
    );
}
