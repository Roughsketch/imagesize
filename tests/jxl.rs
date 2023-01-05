#[cfg(test)]
use imagesize::size;

// Small image (<= 5 bits and multiple of 8)

#[test]
fn jxl_valid_small() {
    let dim = size("tests/images/jxl/valid_small.jxl").unwrap();
    assert_eq!(dim.width, 32);
    assert_eq!(dim.height, 32);
}

// 9 bits width, {9, 13, 18, 30} bits height

#[test]
fn jxl_valid_9w_9h() {
    let dim = size("tests/images/jxl/valid_9w_9h.jxl").unwrap();
    assert_eq!(dim.width, 256);
    assert_eq!(dim.height, 256);
}

#[test]
fn jxl_valid_9w_13h() {
    let dim = size("tests/images/jxl/valid_9w_13h.jxl").unwrap();
    assert_eq!(dim.width, 1);
    assert_eq!(dim.height, 4096);
}

#[test]
fn jxl_valid_9w_18h() {
    let dim = size("tests/images/jxl/valid_9w_18h.jxl").unwrap();
    assert_eq!(dim.width, 1);
    assert_eq!(dim.height, 65536);
}

#[test]
fn jxl_valid_9w_30h() {
    let dim = size("tests/images/jxl/valid_9w_30h.jxl").unwrap();
    assert_eq!(dim.width, 1);
    assert_eq!(dim.height, 16777216);
}

// {13, 18, 30} bits width, 9 bits height

#[test]
fn jxl_valid_13w_9h() {
    let dim = size("tests/images/jxl/valid_13w_9h.jxl").unwrap();
    assert_eq!(dim.width, 4096);
    assert_eq!(dim.height, 1);
}

#[test]
fn jxl_valid_18w_9h() {
    let dim = size("tests/images/jxl/valid_18w_9h.jxl").unwrap();
    assert_eq!(dim.width, 65536);
    assert_eq!(dim.height, 1);
}

#[test]
fn jxl_valid_30w_9h() {
    let dim = size("tests/images/jxl/valid_30w_9h.jxl").unwrap();
    assert_eq!(dim.width, 16777216);
    assert_eq!(dim.height, 1);
}

// Common ratios

#[test]
fn jxl_valid_ratio1() {
    let dim = size("tests/images/jxl/valid_ratio1.jxl").unwrap();
    assert_eq!(dim.width, 100);
    assert_eq!(dim.height, 100);
}

#[test]
fn jxl_valid_ratio2() {
    let dim = size("tests/images/jxl/valid_ratio2.jxl").unwrap();
    assert_eq!(dim.width, 120);
    assert_eq!(dim.height, 100);
}

#[test]
fn jxl_valid_ratio3() {
    let dim = size("tests/images/jxl/valid_ratio3.jxl").unwrap();
    assert_eq!(dim.width, 400);
    assert_eq!(dim.height, 300);
}

#[test]
fn jxl_valid_ratio4() {
    let dim = size("tests/images/jxl/valid_ratio4.jxl").unwrap();
    assert_eq!(dim.width, 300);
    assert_eq!(dim.height, 200);
}

#[test]
fn jxl_valid_ratio5() {
    let dim = size("tests/images/jxl/valid_ratio5.jxl").unwrap();
    assert_eq!(dim.width, 160);
    assert_eq!(dim.height, 90);
}

#[test]
fn jxl_valid_ratio6() {
    let dim = size("tests/images/jxl/valid_ratio6.jxl").unwrap();
    assert_eq!(dim.width, 500);
    assert_eq!(dim.height, 400);
}

#[test]
fn jxl_valid_ratio7() {
    let dim = size("tests/images/jxl/valid_ratio7.jxl").unwrap();
    assert_eq!(dim.width, 200);
    assert_eq!(dim.height, 100);
}

// Container format

#[test]
fn jxl_valid_box_jxlc() {
    let dim = size("tests/images/jxl/valid_box_jxlc.jxl").unwrap();
    assert_eq!(dim.width, 256);
    assert_eq!(dim.height, 256);
}

#[test]
fn jxl_valid_box_jxlp() {
    let dim = size("tests/images/jxl/valid_box_jxlp.jxl").unwrap();
    assert_eq!(dim.width, 256);
    assert_eq!(dim.height, 256);
}

// Orientation

#[test]
fn jxl_valid_orientation0() {
    let dim = size("tests/images/jxl/valid_orientation0.jxl").unwrap();
    assert_eq!(dim.width, 1386);
    assert_eq!(dim.height, 924);
}

#[test]
fn jxl_valid_orientation4() {
    let dim = size("tests/images/jxl/valid_orientation4.jxl").unwrap();
    assert_eq!(dim.width, 924);
    assert_eq!(dim.height, 1386);
}

// Bad inputs

#[test]
fn jxl_err_box() {
    assert!(size("tests/images/jxl/err_box.jxl").is_err());
}

#[test]
fn jxl_err_header() {
    assert!(size("tests/images/jxl/err_header.jxl").is_err());
}

#[test]
fn jxl_err_signature() {
    assert!(size("tests/images/jxl/err_signature.jxl").is_err());
}
