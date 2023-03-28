#[cfg(test)]
use imagesize::size;

#[test]
fn tiff_test() {
    let dim = size("tests/images/tif/test.tif").unwrap();
    assert_eq!(dim.width, 1419);
    assert_eq!(dim.height, 1001);
}

#[test]
fn tiff_test_16bit_size() {
    let dim = size("tests/images/tif/test_16.tif").unwrap();
    assert_eq!(dim.width, 256);
    assert_eq!(dim.height, 256);
}
