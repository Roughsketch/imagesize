#[cfg(test)]
use imagesize::size;

#[test]
fn psd_test() {
    let dim = size("test/psd/test.psd").unwrap();
    assert_eq!(dim.width, 500);
    assert_eq!(dim.height, 500);
}