#[cfg(test)]
use imagesize::{size, ImageSize};

#[test]
fn gif_test() {
    let dim = size("tests/images/gif/test.gif").unwrap();
    assert_eq!(
        dim,
        ImageSize {
            width: 100,
            height: 100
        }
    );
}
