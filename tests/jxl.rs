#[cfg(test)]
use imagesize::{size, ImageSize};

// Small image (<= 5 bits and multiple of 8)

#[test]
#[cfg(feature = "jxl")]
fn jxl_valid_small() {
    let dim = size("tests/images/jxl/valid_small.jxl").unwrap();
    assert_eq!(
        dim,
        ImageSize {
            width: 32,
            height: 32
        }
    );
}

// 9 bits width, {9, 13, 18, 30} bits height

#[test]
#[cfg(feature = "jxl")]
fn jxl_valid_9w_9h() {
    let dim = size("tests/images/jxl/valid_9w_9h.jxl").unwrap();
    assert_eq!(
        dim,
        ImageSize {
            width: 256,
            height: 256
        }
    );
}

#[test]
#[cfg(feature = "jxl")]
fn jxl_valid_9w_13h() {
    let dim = size("tests/images/jxl/valid_9w_13h.jxl").unwrap();
    assert_eq!(
        dim,
        ImageSize {
            width: 1,
            height: 4096
        }
    );
}

#[test]
#[cfg(feature = "jxl")]
fn jxl_valid_9w_18h() {
    let dim = size("tests/images/jxl/valid_9w_18h.jxl").unwrap();
    assert_eq!(
        dim,
        ImageSize {
            width: 1,
            height: 65536
        }
    );
}

#[test]
#[cfg(feature = "jxl")]
fn jxl_valid_9w_30h() {
    let dim = size("tests/images/jxl/valid_9w_30h.jxl").unwrap();
    assert_eq!(
        dim,
        ImageSize {
            width: 1,
            height: 16777216
        }
    );
}

// {13, 18, 30} bits width, 9 bits height

#[test]
#[cfg(feature = "jxl")]
fn jxl_valid_13w_9h() {
    let dim = size("tests/images/jxl/valid_13w_9h.jxl").unwrap();
    assert_eq!(
        dim,
        ImageSize {
            width: 4096,
            height: 1
        }
    );
}

#[test]
#[cfg(feature = "jxl")]
fn jxl_valid_18w_9h() {
    let dim = size("tests/images/jxl/valid_18w_9h.jxl").unwrap();
    assert_eq!(
        dim,
        ImageSize {
            width: 65536,
            height: 1
        }
    );
}

#[test]
#[cfg(feature = "jxl")]
fn jxl_valid_30w_9h() {
    let dim = size("tests/images/jxl/valid_30w_9h.jxl").unwrap();
    assert_eq!(
        dim,
        ImageSize {
            width: 16777216,
            height: 1
        }
    );
}

// Common ratios

#[test]
#[cfg(feature = "jxl")]
fn jxl_valid_ratio1() {
    let dim = size("tests/images/jxl/valid_ratio1.jxl").unwrap();
    assert_eq!(
        dim,
        ImageSize {
            width: 100,
            height: 100
        }
    );
}

#[test]
#[cfg(feature = "jxl")]
fn jxl_valid_ratio2() {
    let dim = size("tests/images/jxl/valid_ratio2.jxl").unwrap();
    assert_eq!(
        dim,
        ImageSize {
            width: 120,
            height: 100
        }
    );
}

#[test]
#[cfg(feature = "jxl")]
fn jxl_valid_ratio3() {
    let dim = size("tests/images/jxl/valid_ratio3.jxl").unwrap();
    assert_eq!(
        dim,
        ImageSize {
            width: 400,
            height: 300
        }
    );
}

#[test]
#[cfg(feature = "jxl")]
fn jxl_valid_ratio4() {
    let dim = size("tests/images/jxl/valid_ratio4.jxl").unwrap();
    assert_eq!(
        dim,
        ImageSize {
            width: 300,
            height: 200
        }
    );
}

#[test]
#[cfg(feature = "jxl")]
fn jxl_valid_ratio5() {
    let dim = size("tests/images/jxl/valid_ratio5.jxl").unwrap();
    assert_eq!(
        dim,
        ImageSize {
            width: 160,
            height: 90
        }
    );
}

#[test]
#[cfg(feature = "jxl")]
fn jxl_valid_ratio6() {
    let dim = size("tests/images/jxl/valid_ratio6.jxl").unwrap();
    assert_eq!(
        dim,
        ImageSize {
            width: 500,
            height: 400
        }
    );
}

#[test]
#[cfg(feature = "jxl")]
fn jxl_valid_ratio7() {
    let dim = size("tests/images/jxl/valid_ratio7.jxl").unwrap();
    assert_eq!(
        dim,
        ImageSize {
            width: 200,
            height: 100
        }
    );
}

// Container format

#[test]
#[cfg(feature = "jxl")]
fn jxl_valid_box_jxlc() {
    let dim = size("tests/images/jxl/valid_box_jxlc.jxl").unwrap();
    assert_eq!(
        dim,
        ImageSize {
            width: 256,
            height: 256
        }
    );
}

#[test]
#[cfg(feature = "jxl")]
fn jxl_valid_box_jxlp() {
    let dim = size("tests/images/jxl/valid_box_jxlp.jxl").unwrap();
    assert_eq!(
        dim,
        ImageSize {
            width: 256,
            height: 256
        }
    );
}

// Orientation

#[test]
#[cfg(feature = "jxl")]
fn jxl_valid_orientation0() {
    let dim = size("tests/images/jxl/valid_orientation0.jxl").unwrap();
    assert_eq!(
        dim,
        ImageSize {
            width: 1386,
            height: 924
        }
    );
}

#[test]
#[cfg(feature = "jxl")]
fn jxl_valid_orientation4() {
    let dim = size("tests/images/jxl/valid_orientation4.jxl").unwrap();
    assert_eq!(
        dim,
        ImageSize {
            width: 924,
            height: 1386
        }
    );
}

// Bad inputs

#[test]
#[cfg(feature = "jxl")]
fn jxl_err_box() {
    assert!(size("tests/images/jxl/err_box.jxl").is_err());
}

#[test]
#[cfg(feature = "jxl")]
fn jxl_err_header() {
    assert!(size("tests/images/jxl/err_header.jxl").is_err());
}

#[test]
#[cfg(feature = "jxl")]
fn jxl_err_signature() {
    assert!(size("tests/images/jxl/err_signature.jxl").is_err());
}
