#[cfg(test)]
use imagesize::{size, ImageSize};

#[test]
fn heif_test() {
    let dim = size("tests/images/heic/test.heic").unwrap();
    assert_eq!(
        dim,
        ImageSize {
            width: 1280,
            height: 720
        }
    );
}

#[test]
fn heif_multi_picks_largest() {
    let dim = size("tests/images/heic/IMG_0007.heic").unwrap();
    assert_eq!(
        dim,
        ImageSize {
            width: 2448,
            height: 3264
        }
    );
}
