#[cfg(test)]
use imagesize::{image_type, size, Compression, ImageSize, ImageType};

#[test]
#[cfg(feature = "heif")]
fn avif_test() {
    let dim = size("tests/images/avif/test.avif").unwrap();
    assert_eq!(
        dim,
        ImageSize {
            width: 1204,
            height: 800
        }
    );
}

#[test]
#[cfg(feature = "heif")]
fn avif_multi_picks_largest() {
    let dim = size("tests/images/avif/test.avifs").unwrap();
    assert_eq!(
        dim,
        ImageSize {
            width: 159,
            height: 159
        }
    );
}

#[test]
#[cfg(feature = "heif")]
fn avif_type() {
    use std::{fs::File, io::Read};

    let mut f = File::open("tests/images/avif/test.avif").unwrap();
    let mut buf = vec![];
    f.read_to_end(&mut buf).unwrap();

    let ty = image_type(&buf).unwrap();
    assert_eq!(ty, ImageType::Heif(Compression::Av1));
}

#[test]
#[cfg(feature = "heif")]
fn avif_seq_type() {
    use std::{fs::File, io::Read};

    let mut f = File::open("tests/images/avif/test.avifs").unwrap();
    let mut buf = vec![];
    f.read_to_end(&mut buf).unwrap();

    let ty = image_type(&buf).unwrap();
    assert_eq!(ty, ImageType::Heif(Compression::Av1));
}
