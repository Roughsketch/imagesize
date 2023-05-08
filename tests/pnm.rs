#[cfg(test)]
use imagesize::{size, ImageSize};

// Test files from: 
// P1: https://people.sc.fsu.edu/~jburkardt/data/pbma/pbma.html
// P4: https://people.sc.fsu.edu/~jburkardt/data/pbmb/pbmb.html
//
// Need more test files for P2, 3, 5, 6
#[test]
fn pnm_same_line_test() {
    let dim = size("tests/images/pnm/P1/fool.ascii.pbm").unwrap();
    assert_eq!(dim, ImageSize { width: 514, height: 324 });
}

#[test]
fn pnm_p1_test() {
    let dim = size("tests/images/pnm/P1/feep.ascii.pbm").unwrap();
    assert_eq!(dim, ImageSize { width: 24, height: 7 });
}

#[test]
fn pnm_p2_test() {
    let dim = size("tests/images/pnm/P2/feep.ascii.pgm").unwrap();
    assert_eq!(dim, ImageSize { width: 24, height: 7 });
}

#[test]
fn pnm_p3_test() {
    let dim = size("tests/images/pnm/P3/feep.ascii.ppm").unwrap();
    assert_eq!(dim, ImageSize { width: 4, height: 4 });
}

#[test]
fn pnm_p4_test() {
    let dim = size("tests/images/pnm/P4/feep.pbm").unwrap();
    assert_eq!(dim, ImageSize { width: 24, height: 7 });
}

#[test]
fn pnm_p5_test() {
    let dim = size("tests/images/pnm/P5/feep.pgm").unwrap();
    assert_eq!(dim, ImageSize { width: 24, height: 7 });
}

// #[test]
// fn pnm_p6_test() {
//     let dim = size("tests/images/pnm/P6/pbmlib.ppm").unwrap();
//     assert_eq!(dim, ImageSize { width: 20, height: 10 });
// }
