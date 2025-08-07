#[cfg(test)]
use imagesize::{blob_size, image_type, size, ImageType};
use std::fs;

#[test]
#[cfg(feature = "pvrtc")]
fn pvrtc_format_detection() {
    let data = fs::read("tests/images/pvrtc/64x64.pvr").expect("Failed to read PVRTC test file");

    match image_type(&data) {
        Ok(ImageType::Pvrtc(_)) => (),
        _ => panic!("PVRTC format not detected correctly"),
    }
}

#[test]
#[cfg(feature = "pvrtc")]
fn pvrtc_size_reading_64x64() {
    let data = fs::read("tests/images/pvrtc/64x64.pvr").expect("Failed to read PVRTC test file");

    let img_size = blob_size(&data).unwrap();
    assert_eq!(img_size.width, 64);
    assert_eq!(img_size.height, 64);
}

#[test]
#[cfg(feature = "pvrtc")]
fn pvrtc_size_reading_128x256() {
    let data = fs::read("tests/images/pvrtc/128x256.pvr").expect("Failed to read PVRTC test file");

    let img_size = blob_size(&data).unwrap();
    assert_eq!(img_size.width, 128);
    assert_eq!(img_size.height, 256);
}

#[test]
#[cfg(feature = "pvrtc")]
fn pvrtc_size_function() {
    let img_size = size("tests/images/pvrtc/64x64.pvr").expect("Failed to get PVRTC image size");
    assert_eq!(img_size.width, 64);
    assert_eq!(img_size.height, 64);
}
