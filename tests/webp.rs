#[cfg(test)]
use imagesize::size;

#[test]
fn webp_test() {
    let dim = size("test/webp/test.webp").unwrap();
    assert_eq!(dim.width, 716);
    assert_eq!(dim.height, 716);
}

#[test]
fn riffx_webp_test() {
    let dim = size("test/webp/riffx.webp").unwrap();
    assert_eq!(dim.width, 128);
    assert_eq!(dim.height, 128);
}

#[test]
fn webp_extended() {
    let dim = size("test/webp/extended.16x32.webp").unwrap();
    assert_eq!(dim.width, 16);
    assert_eq!(dim.height, 32);
}

#[test]
fn webp_lossless() {
    let dim = size("test/webp/lossless.16x32.webp").unwrap();
    assert_eq!(dim.width, 16);
    assert_eq!(dim.height, 32);
}

#[test]
fn webp_lossy() {
    let dim = size("test/webp/lossy.16x32.webp").unwrap();
    assert_eq!(dim.width, 16);
    assert_eq!(dim.height, 32);
}