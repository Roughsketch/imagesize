#[cfg(test)]
use imagesize::size;

#[test]
fn exr_test() {
    let dim = size("tests/images/exr/test.exr").unwrap();
    assert_eq!(dim.width, 100);
    assert_eq!(dim.height, 100);
}
