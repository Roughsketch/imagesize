#[cfg(test)]
use imagesize::{size, ImageSize};

#[test]
#[cfg(feature = "tiff")]
fn tiff_test() {
    let dim = size("tests/images/tif/test.tif").unwrap();
    assert_eq!(
        dim,
        ImageSize {
            width: 1419,
            height: 1001
        }
    );
}

#[test]
#[cfg(feature = "tiff")]
fn tiff_test_16bit_size() {
    let dim = size("tests/images/tif/test_16.tif").unwrap();
    assert_eq!(
        dim,
        ImageSize {
            width: 256,
            height: 256
        }
    );
}

#[test]
#[cfg(feature = "tiff")]
fn tiff_test_bigtiff() {
    let dim = size("tests/images/tif/test_bif.tif").unwrap();
    assert_eq!(
        dim,
        ImageSize {
            width: 1,
            height: 1,
        }
    );
}
