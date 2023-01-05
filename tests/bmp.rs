#[cfg(test)]
use imagesize::size;

#[test]
fn bmp_test() {
    let dim = size("tests/images/bmp/test.bmp").unwrap();
    assert_eq!(dim.width, 512);
    assert_eq!(dim.height, 512);
}
