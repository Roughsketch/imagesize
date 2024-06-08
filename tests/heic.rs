#[cfg(test)]
use imagesize::{image_type, size, ImageSize, ImageType};

#[test]
fn heic_test() {
    let dim = size("tests/images/heic/heic.heic").unwrap();
    assert_eq!(
        dim,
        ImageSize {
            width: 2448,
            height: 3264
        }
    );
}

#[test]
fn heic_multi_picks_largest() {
    let dim = size("tests/images/heic/heic.heic").unwrap();
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

    let mut f = File::open("tests/images/heic/heic.heic").unwrap();
    let mut buf = vec![];
    f.read_to_end(&mut buf).unwrap();

    let ty = image_type(&buf).unwrap();
    assert_eq!(ty, ImageType::Heic);
}

#[test]
fn heic_msf1_type() {
    use std::{fs::File, io::Read};

    let mut f = File::open("tests/images/heic/heic_msf1.heic").unwrap();
    let mut buf = vec![];
    f.read_to_end(&mut buf).unwrap();

    let ty = image_type(&buf).unwrap();
    assert_eq!(ty, ImageType::Heic);
}
