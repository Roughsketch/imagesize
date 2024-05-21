#[cfg(test)]
use imagesize::{image_type, size, ImageSize, ImageType};

#[test]
fn heic_test() {
    let dim = size("tests/images/heic/test.heic").unwrap();
    assert_eq!(
        dim,
        ImageSize {
            width: 1280,
            height: 720
        }
    );
}

#[test]
fn heic_multi_picks_largest() {
    let dim = size("tests/images/heic/IMG_0007.heic").unwrap();
    assert_eq!(
        dim,
        ImageSize {
            width: 2448,
            height: 3264
        }
    );
}

#[test]
fn heic_type() {
    use std::{fs::File, io::Read};

    let mut f = File::open("tests/images/heic/test.heic").unwrap();
    let mut buf = vec![];
    f.read_to_end(&mut buf).unwrap();

    let ty = image_type(&buf).unwrap();
    assert_eq!(ty, ImageType::Heic);
}
