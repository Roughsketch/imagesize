#[cfg(test)]
use imagesize::{
    image_type, size, AtcCompression, DdsCompression, ImageType, PkmCompression, PvrtcCompression,
};

#[test]
#[cfg(feature = "dds")]
fn test_all_dds_compressions() {
    let test_cases = vec![
        (
            "tests/images/dds/compressions/bc1.dds",
            DdsCompression::Bc1,
            690,
            298,
        ),
        (
            "tests/images/dds/compressions/bc2.dds",
            DdsCompression::Bc2,
            690,
            298,
        ),
        (
            "tests/images/dds/compressions/bc3.dds",
            DdsCompression::Bc3,
            690,
            298,
        ),
        (
            "tests/images/dds/compressions/bc4.dds",
            DdsCompression::Bc4,
            690,
            298,
        ),
        (
            "tests/images/dds/compressions/bc5.dds",
            DdsCompression::Bc5,
            690,
            298,
        ),
        (
            "tests/images/dds/compressions/bc6h.dds",
            DdsCompression::Bc6h,
            690,
            298,
        ),
        (
            "tests/images/dds/compressions/bc7.dds",
            DdsCompression::Bc7,
            690,
            298,
        ),
        (
            "tests/images/dds/compressions/rgba32.dds",
            DdsCompression::Rgba32,
            690,
            298,
        ),
    ];

    for (file_path, expected_compression, expected_width, expected_height) in test_cases {
        if std::path::Path::new(file_path).exists() {
            // Test size detection
            let size_result = size(file_path);
            assert!(size_result.is_ok(), "Failed to get size for {file_path}");

            let img_size = size_result.unwrap();
            assert_eq!(
                img_size.width, expected_width,
                "Wrong width detected for {file_path}: expected {expected_width}, got {}",
                img_size.width
            );
            assert_eq!(
                img_size.height, expected_height,
                "Wrong height detected for {file_path}: expected {expected_height}, got {}",
                img_size.height
            );

            // Test format and compression detection
            let file_data = std::fs::read(file_path).unwrap();
            match image_type(&file_data) {
                Ok(ImageType::Dds(compression)) => {
                    assert_eq!(
                        compression, expected_compression,
                        "Wrong compression detected for {file_path}"
                    );
                }
                Ok(other) => panic!("Expected DDS format, got {other:?} for {file_path}"),
                Err(e) => panic!("Failed to detect format for {file_path}: {e:?}"),
            }
        } else {
            println!("Skipping test for {file_path} (file not found)");
        }
    }
}

#[test]
#[cfg(feature = "etc2")]
fn test_all_pkm_etc2_compressions() {
    let test_cases = vec![(
        "tests/images/pkm/compressions/etc1.pkm",
        PkmCompression::Etc1,
        690,
        298,
    )];

    for (file_path, expected_compression, expected_width, expected_height) in test_cases {
        if std::path::Path::new(file_path).exists() {
            // Test size detection
            let size_result = size(file_path);
            assert!(size_result.is_ok(), "Failed to get size for {file_path}");

            let img_size = size_result.unwrap();
            assert_eq!(
                img_size.width, expected_width,
                "Wrong width detected for {file_path}: expected {expected_width}, got {}",
                img_size.width
            );
            assert_eq!(
                img_size.height, expected_height,
                "Wrong height detected for {file_path}: expected {expected_height}, got {}",
                img_size.height
            );

            // Test format and compression detection
            let file_data = std::fs::read(file_path).unwrap();
            match image_type(&file_data) {
                Ok(ImageType::Etc2(compression)) => {
                    assert_eq!(
                        compression, expected_compression,
                        "Wrong compression detected for {file_path}"
                    );
                }
                Ok(other) => panic!("Expected ETC2 format, got {other:?} for {file_path}"),
                Err(e) => panic!("Failed to detect format for {file_path}: {e:?}"),
            }
        } else {
            println!("Skipping test for {file_path} (file not found)");
        }
    }
}

#[test]
#[cfg(feature = "eac")]
fn test_all_pkm_eac_compressions() {
    // Note: EAC testing is currently disabled because we don't have proper PKM format EAC files.
    // EAC formats are available in PowerVR format and tested via PVRTC tests.
    let test_cases: Vec<(&str, PkmCompression)> = vec![];

    for (file_path, expected_compression) in test_cases {
        if std::path::Path::new(file_path).exists() {
            let size_result = size(file_path);
            assert!(size_result.is_ok(), "Failed to get size for {file_path}");

            let file_data = std::fs::read(file_path).unwrap();
            match image_type(&file_data) {
                Ok(ImageType::Eac(compression)) => {
                    assert_eq!(
                        compression, expected_compression,
                        "Wrong compression detected for {file_path}"
                    );
                }
                Ok(other) => panic!("Expected EAC format, got {other:?} for {file_path}"),
                Err(e) => panic!("Failed to detect format for {file_path}: {e:?}"),
            }
        } else {
            println!("Skipping test for {file_path} (file not found)");
        }
    }
}

#[test]
#[cfg(feature = "atc")]
fn test_all_atc_compressions() {
    let test_cases = vec![
        (
            "tests/images/atc/compressions/atc_rgb.pkm",
            AtcCompression::Rgb,
            64,
            64,
        ),
        (
            "tests/images/atc/compressions/atc_rgba_explicit.pkm",
            AtcCompression::RgbaExplicit,
            64,
            64,
        ),
        (
            "tests/images/atc/compressions/atc_rgba_interp.pkm",
            AtcCompression::RgbaInterpolated,
            64,
            64,
        ),
    ];

    for (file_path, expected_compression, expected_width, expected_height) in test_cases {
        if std::path::Path::new(file_path).exists() {
            // Test size detection
            let size_result = size(file_path);
            assert!(size_result.is_ok(), "Failed to get size for {file_path}");

            let img_size = size_result.unwrap();
            assert_eq!(
                img_size.width, expected_width,
                "Wrong width detected for {file_path}: expected {expected_width}, got {}",
                img_size.width
            );
            assert_eq!(
                img_size.height, expected_height,
                "Wrong height detected for {file_path}: expected {expected_height}, got {}",
                img_size.height
            );

            // Test format and compression detection
            let file_data = std::fs::read(file_path).unwrap();
            match image_type(&file_data) {
                Ok(ImageType::Atc(compression)) => {
                    assert_eq!(
                        compression, expected_compression,
                        "Wrong compression detected for {file_path}"
                    );
                }
                Ok(other) => panic!("Expected ATC format, got {other:?} for {file_path}"),
                Err(e) => panic!("Failed to detect format for {file_path}: {e:?}"),
            }
        } else {
            println!("Skipping test for {file_path} (file not found)");
        }
    }
}

#[test]
#[cfg(feature = "pvrtc")]
fn test_all_pvrtc_compressions() {
    // Test PowerVR format files with various compression types:
    // - PVRTC compression variants (native PowerVR)
    // - ETC2/EAC compression stored in PowerVR containers
    let test_cases = vec![
        // PVRTC compression formats (512x256)
        (
            "tests/images/pvrtc/compressions/pvrtc_2bpp_rgb.pvr",
            PvrtcCompression::Pvrtc2BppRgb,
            512,
            256,
        ),
        (
            "tests/images/pvrtc/compressions/pvrtc_2bpp_rgba.pvr",
            PvrtcCompression::Pvrtc2BppRgba,
            512,
            256,
        ),
        (
            "tests/images/pvrtc/compressions/pvrtc_4bpp_rgb.pvr",
            PvrtcCompression::Pvrtc4BppRgb,
            512,
            256,
        ),
        (
            "tests/images/pvrtc/compressions/pvrtc_4bpp_rgba.pvr",
            PvrtcCompression::Pvrtc4BppRgba,
            512,
            256,
        ),
        // ETC2 compression formats in PowerVR containers (690x298)
        (
            "tests/images/pvrtc/compressions/etc2_rgb.pvr",
            PvrtcCompression::Etc2Rgb,
            690,
            298,
        ),
        (
            "tests/images/pvrtc/compressions/etc2_a1.pvr",
            PvrtcCompression::Etc2RgbA1,
            690,
            298,
        ),
        (
            "tests/images/pvrtc/compressions/etc2_a8.pvr",
            PvrtcCompression::Etc2Rgba,
            690,
            298,
        ),
        // EAC compression formats in PowerVR containers (690x298)
        (
            "tests/images/pvrtc/compressions/eac_r.pvr",
            PvrtcCompression::EacR11,
            690,
            298,
        ),
        (
            "tests/images/pvrtc/compressions/eac_rg.pvr",
            PvrtcCompression::EacRg11,
            690,
            298,
        ),
    ];

    for (file_path, expected_compression, expected_width, expected_height) in test_cases {
        if std::path::Path::new(file_path).exists() {
            // Test size detection
            let size_result = size(file_path);
            assert!(size_result.is_ok(), "Failed to get size for {file_path}");

            let img_size = size_result.unwrap();
            assert_eq!(
                img_size.width, expected_width,
                "Wrong width detected for {file_path}: expected {expected_width}, got {}",
                img_size.width
            );
            assert_eq!(
                img_size.height, expected_height,
                "Wrong height detected for {file_path}: expected {expected_height}, got {}",
                img_size.height
            );

            // Test format and compression detection
            let file_data = std::fs::read(file_path).unwrap();
            match image_type(&file_data) {
                Ok(ImageType::Pvrtc(compression)) => {
                    assert_eq!(
                        compression, expected_compression,
                        "Wrong compression detected for {file_path}"
                    );
                }
                Ok(other) => panic!("Expected PVRTC format, got {other:?} for {file_path}"),
                Err(e) => panic!("Failed to detect format for {file_path}: {e:?}"),
            }
        } else {
            println!("Skipping test for {file_path} (file not found)");
        }
    }
}
