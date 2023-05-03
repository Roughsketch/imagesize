#[cfg(test)]
use imagesize::size;

#[test]
fn dds_test() {
    let dim = size("tests/images/dds/test.dds").unwrap();
    assert_eq!(dim.width, 100);
    assert_eq!(dim.height, 67);
}
