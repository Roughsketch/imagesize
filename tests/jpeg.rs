#[cfg(test)]
use imagesize::size;

#[test]
fn jpeg_test() {
    let dim = size("tests/images/jpg/test.jpg").unwrap();
    assert_eq!(dim.width, 690);
    assert_eq!(dim.height, 298);
}

#[test]
fn jpeg_extra_info_test() {
    let dim = size("tests/images/jpg/extra.jpg").unwrap();
    assert_eq!(dim.width, 1500);
    assert_eq!(dim.height, 844);
}

#[test]
fn issue_9_test() {
    let dim = size("tests/images/jpg/issue-9.jpg").unwrap();
    assert_eq!(dim.width, 1360);
    assert_eq!(dim.height, 1904);
}

#[test]
fn jpg_unexpected_eof() {
    let dim = size("tests/images/jpg/unexpected_eof.jpg").unwrap();
    assert_eq!(dim.width, 3047);
    assert_eq!(dim.height, 2008);
}

#[test]
fn jpg_unexpected_eof_2() {
    let dim = size("tests/images/jpg/unexpected_eof_2.jpg").unwrap();
    assert_eq!(dim.width, 4980);
    assert_eq!(dim.height, 3321);
}

#[test]
fn jpg_unexpected_eof_3() {
    let dim = size("tests/images/jpg/unexpected_eof_3.jpg").unwrap();
    assert_eq!(dim.width, 2995);
    assert_eq!(dim.height, 1998);
}

#[test]
fn jpg_wrong_size() {
    let dim = size("tests/images/jpg/wrong_size.jpg").unwrap();
    assert_eq!(dim.width, 1080);
    assert_eq!(dim.height, 1080);
}
