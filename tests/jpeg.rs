#[cfg(test)]
use imagesize::{size, ImageSize};

#[test]
fn jpeg_test() {
    let dim = size("tests/images/jpg/test.jpg").unwrap();
    assert_eq!(
        dim,
        ImageSize {
            width: 690,
            height: 298
        }
    );
}

#[test]
fn jpeg_extra_info_test() {
    let dim = size("tests/images/jpg/extra.jpg").unwrap();
    assert_eq!(
        dim,
        ImageSize {
            width: 1500,
            height: 844
        }
    );
}

#[test]
fn issue_9_test() {
    let dim = size("tests/images/jpg/issue-9.jpg").unwrap();
    assert_eq!(
        dim,
        ImageSize {
            width: 1360,
            height: 1904
        }
    );
}

#[test]
fn jpg_unexpected_eof() {
    let dim = size("tests/images/jpg/unexpected_eof.jpg").unwrap();
    assert_eq!(
        dim,
        ImageSize {
            width: 3047,
            height: 2008
        }
    );
}

#[test]
fn jpg_unexpected_eof_2() {
    let dim = size("tests/images/jpg/unexpected_eof_2.jpg").unwrap();
    assert_eq!(
        dim,
        ImageSize {
            width: 4980,
            height: 3321
        }
    );
}

#[test]
fn jpg_unexpected_eof_3() {
    let dim = size("tests/images/jpg/unexpected_eof_3.jpg").unwrap();
    assert_eq!(
        dim,
        ImageSize {
            width: 2995,
            height: 1998
        }
    );
}

#[test]
fn jpg_wrong_size() {
    let dim = size("tests/images/jpg/wrong_size.jpg").unwrap();
    assert_eq!(
        dim,
        ImageSize {
            width: 1080,
            height: 1080
        }
    );
}
