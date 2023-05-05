#[cfg(test)]
use imagesize::size;

#[test]
fn qoi_test() {
    let dim = size("tests/images/qoi/test.qoi").unwrap();
    assert_eq!(dim.width, 800);
    assert_eq!(dim.height, 600);
}
