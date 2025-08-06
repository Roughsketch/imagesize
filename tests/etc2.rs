#[cfg(test)]
use imagesize::{blob_size, image_type, size, ImageType};
use std::fs;

#[test]
#[cfg(feature = "etc2")]
fn etc2_format_detection() {
    let data = fs::read("tests/images/etc2/64x64_rgb.pkm").expect("Failed to read ETC2 test file");

    match image_type(&data) {
        Ok(ImageType::Etc2) => (),
        _ => panic!("ETC2 format not detected correctly"),
    }
}

#[test]
#[cfg(feature = "etc2")]
fn etc2_size_reading_64x64() {
    let data = fs::read("tests/images/etc2/64x64_rgb.pkm").expect("Failed to read ETC2 test file");

    let img_size = blob_size(&data).unwrap();
    assert_eq!(img_size.width, 64);
    assert_eq!(img_size.height, 64);
}

#[test]
#[cfg(feature = "etc2")]
fn etc2_size_reading_128x64() {
    let data = fs::read("tests/images/etc2/128x64_rgb.pkm").expect("Failed to read ETC2 test file");

    let img_size = blob_size(&data).unwrap();
    assert_eq!(img_size.width, 128);
    assert_eq!(img_size.height, 64);
}

#[test]
#[cfg(feature = "etc2")]
fn etc2_size_function() {
    let img_size = size("tests/images/etc2/64x64_rgb.pkm").expect("Failed to get ETC2 image size");
    assert_eq!(img_size.width, 64);
    assert_eq!(img_size.height, 64);
}
