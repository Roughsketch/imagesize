#[cfg(test)]
use imagesize::size;

#[test]
fn heif_test() {
    let dim = size("test/heic/test.heic").unwrap();
    assert_eq!(dim.width, 1280);
    assert_eq!(dim.height, 720);
}

#[test]
fn heif_multi_picks_largest() {
    let dim = size("test/heic/IMG_0007.heic").unwrap();
    assert_eq!(dim.width, 2448);
    assert_eq!(dim.height, 3264);
}