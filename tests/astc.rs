#[cfg(test)]
use imagesize::{blob_size, image_type, size, ImageType};
use std::fs;

#[test]
#[cfg(feature = "astc")]
fn astc_format_detection() {
    let data = fs::read("tests/images/astc/64x64_4x4.astc").expect("Failed to read ASTC test file");

    match image_type(&data) {
        Ok(ImageType::Astc) => (),
        _ => panic!("ASTC format not detected correctly"),
    }
}

#[test]
#[cfg(feature = "astc")]
fn astc_size_reading_64x64() {
    let data = fs::read("tests/images/astc/64x64_4x4.astc").expect("Failed to read ASTC test file");

    let img_size = blob_size(&data).unwrap();
    assert_eq!(img_size.width, 64);
    assert_eq!(img_size.height, 64);
}

#[test]
#[cfg(feature = "astc")]
fn astc_size_reading_128x256() {
    let data =
        fs::read("tests/images/astc/128x256_6x6.astc").expect("Failed to read ASTC test file");

    let img_size = blob_size(&data).unwrap();
    assert_eq!(img_size.width, 128);
    assert_eq!(img_size.height, 256);
}

#[test]
#[cfg(feature = "astc")]
fn astc_size_function() {
    let img_size = size("tests/images/astc/64x64_4x4.astc").expect("Failed to get ASTC image size");
    assert_eq!(img_size.width, 64);
    assert_eq!(img_size.height, 64);
}
