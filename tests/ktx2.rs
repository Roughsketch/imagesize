#[cfg(test)]
use imagesize::size;

#[test]
fn exr_test() {
    let dim = size("tests/images/ktx2/test.ktx2").unwrap();
    assert_eq!(dim.width, 1);
    assert_eq!(dim.height, 256);
}
