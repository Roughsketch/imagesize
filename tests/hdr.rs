#[cfg(test)]
use imagesize::size;

#[test]
fn hdr_test() {
    let dim = size("tests/images/hdr/test.hdr").unwrap();
    assert_eq!(dim.width, 100);
    assert_eq!(dim.height, 67);
}
