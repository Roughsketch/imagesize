#[cfg(test)]
use imagesize::size;

#[test]
fn gif_test() {
    let dim = size("tests/images/gif/test.gif").unwrap();
    assert_eq!(dim.width, 100);
    assert_eq!(dim.height, 100);
}
