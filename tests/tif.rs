#[cfg(test)]
use imagesize::size;

#[test]
fn tiff_test() {
    let dim = size("test/tif/test.tif").unwrap();
    assert_eq!(dim.width, 1419);
    assert_eq!(dim.height, 1001);
}
