#[cfg(test)]
use imagesize::{size, ImageSize};

#[test]
#[cfg(feature = "webp")]
fn webp_test() {
    let dim = size("tests/images/webp/test.webp").unwrap();
    assert_eq!(
        dim,
        ImageSize {
            width: 716,
            height: 716
        }
    );
}

#[test]
#[cfg(feature = "webp")]
fn riffx_webp_test() {
    let dim = size("tests/images/webp/riffx.webp").unwrap();
    assert_eq!(
        dim,
        ImageSize {
            width: 128,
            height: 128
        }
    );
}

#[test]
#[cfg(feature = "webp")]
fn webp_extended() {
    let dim = size("tests/images/webp/extended.16x32.webp").unwrap();
    assert_eq!(
        dim,
        ImageSize {
            width: 16,
            height: 32
        }
    );
}

#[test]
#[cfg(feature = "webp")]
fn webp_lossless() {
    let dim = size("tests/images/webp/lossless.16x32.webp").unwrap();
    assert_eq!(
        dim,
        ImageSize {
            width: 16,
            height: 32
        }
    );
}

#[test]
#[cfg(feature = "webp")]
fn webp_lossy() {
    let dim = size("tests/images/webp/lossy.16x32.webp").unwrap();
    assert_eq!(
        dim,
        ImageSize {
            width: 16,
            height: 32
        }
    );
}
