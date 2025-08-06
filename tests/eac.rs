#[cfg(test)]
use imagesize::{blob_size, image_type, size, ImageType};
use std::fs;

#[test]
#[cfg(feature = "eac")]
fn eac_format_detection() {
    let data = fs::read("tests/images/eac/64x64_r11.pkm").expect("Failed to read EAC test file");

    match image_type(&data) {
        Ok(ImageType::Eac) => (),
        _ => panic!("EAC format not detected correctly"),
    }
}

#[test]
#[cfg(feature = "eac")]
fn eac_size_reading_64x64() {
    let data = fs::read("tests/images/eac/64x64_r11.pkm").expect("Failed to read EAC test file");

    let img_size = blob_size(&data).unwrap();
    assert_eq!(img_size.width, 64);
    assert_eq!(img_size.height, 64);
}

#[test]
#[cfg(feature = "eac")]
fn eac_size_reading_256x128() {
    let data = fs::read("tests/images/eac/256x128_rg11.pkm").expect("Failed to read EAC test file");

    let img_size = blob_size(&data).unwrap();
    assert_eq!(img_size.width, 256);
    assert_eq!(img_size.height, 128);
}

#[test]
#[cfg(feature = "eac")]
fn eac_size_function() {
    let img_size = size("tests/images/eac/64x64_r11.pkm").expect("Failed to get EAC image size");
    assert_eq!(img_size.width, 64);
    assert_eq!(img_size.height, 64);
}
