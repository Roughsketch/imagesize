#[cfg(test)]
use imagesize::{size, ImageSize};

#[test]
fn vtf_test() {
    let dim = size("tests/images/vtf/test.vtf").unwrap();
    assert_eq!(
        dim,
        ImageSize {
            width: 512,
            height: 256
        }
    );
}
