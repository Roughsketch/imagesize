#[cfg(test)]
use imagesize::size;

#[test]
fn apng_test() {
    let dim = size("test/png/test.apng").unwrap();
    assert_eq!(dim.width, 100);
    assert_eq!(dim.height, 100);
}

#[test]
fn png_test() {
    let dim = size("test/png/test.png").unwrap();
    assert_eq!(dim.width, 690);
    assert_eq!(dim.height, 298);
}