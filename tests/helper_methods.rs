#[cfg(test)]
use imagesize::{
    image_type, CompressionFamily, DdsCompression, ImageType, PkmCompression, PvrtcCompression,
};

#[test]
#[cfg(feature = "dds")]
fn test_compression_family_dds() {
    let bc1_type = ImageType::Dds(DdsCompression::Bc1);
    assert_eq!(
        bc1_type.compression_family(),
        Some(CompressionFamily::BlockCompression)
    );

    let bc7_type = ImageType::Dds(DdsCompression::Bc7);
    assert_eq!(
        bc7_type.compression_family(),
        Some(CompressionFamily::BlockCompression)
    );

    let rgba_type = ImageType::Dds(DdsCompression::Rgba32);
    assert_eq!(
        rgba_type.compression_family(),
        Some(CompressionFamily::Uncompressed)
    );

    let unknown_type = ImageType::Dds(DdsCompression::Unknown);
    assert_eq!(unknown_type.compression_family(), None);
}

#[test]
#[cfg(feature = "pvrtc")]
fn test_compression_family_pvrtc() {
    let pvrtc_type = ImageType::Pvrtc(PvrtcCompression::Pvrtc2BppRgb);
    assert_eq!(
        pvrtc_type.compression_family(),
        Some(CompressionFamily::Pvrtc)
    );

    let etc2_type = ImageType::Pvrtc(PvrtcCompression::Etc2Rgb);
    assert_eq!(etc2_type.compression_family(), Some(CompressionFamily::Etc));

    let eac_type = ImageType::Pvrtc(PvrtcCompression::EacR11);
    assert_eq!(eac_type.compression_family(), Some(CompressionFamily::Eac));
}

#[test]
#[cfg(feature = "etc2")]
fn test_compression_family_etc2() {
    let etc1_type = ImageType::Etc2(PkmCompression::Etc1);
    assert_eq!(etc1_type.compression_family(), Some(CompressionFamily::Etc));

    let etc2_type = ImageType::Etc2(PkmCompression::Etc2);
    assert_eq!(etc2_type.compression_family(), Some(CompressionFamily::Etc));
}

#[test]
fn test_compression_family_simple_formats() {
    #[cfg(feature = "png")]
    {
        let png_type = ImageType::Png;
        assert_eq!(png_type.compression_family(), None);
    }

    #[cfg(feature = "jpeg")]
    {
        let jpeg_type = ImageType::Jpeg;
        assert_eq!(jpeg_type.compression_family(), None);
    }
}

#[test]
#[cfg(feature = "dds")]
fn test_is_block_compressed() {
    let bc1_type = ImageType::Dds(DdsCompression::Bc1);
    assert!(bc1_type.is_block_compressed());

    let rgba_type = ImageType::Dds(DdsCompression::Rgba32);
    assert!(!rgba_type.is_block_compressed());

    #[cfg(feature = "png")]
    {
        let png_type = ImageType::Png;
        assert!(!png_type.is_block_compressed());
    }
}

#[test]
#[cfg(feature = "dds")]
fn test_container_format() {
    let dds_type = ImageType::Dds(DdsCompression::Bc1);
    assert_eq!(dds_type.container_format(), Some("DDS"));
}

#[test]
#[cfg(feature = "pvrtc")]
fn test_container_format_pvrtc() {
    let pvr_type = ImageType::Pvrtc(PvrtcCompression::Pvrtc2BppRgb);
    assert_eq!(pvr_type.container_format(), Some("PowerVR"));
}

#[test]
#[cfg(feature = "etc2")]
fn test_container_format_pkm() {
    let etc2_type = ImageType::Etc2(PkmCompression::Etc1);
    assert_eq!(etc2_type.container_format(), Some("PKM"));
}

#[test]
fn test_container_format_simple() {
    #[cfg(feature = "png")]
    {
        let png_type = ImageType::Png;
        assert_eq!(png_type.container_format(), None);
    }
}

#[test]
#[cfg(feature = "dds")]
fn test_is_multi_compression_container() {
    let dds_type = ImageType::Dds(DdsCompression::Bc1);
    assert!(dds_type.is_multi_compression_container());
}

#[test]
#[cfg(feature = "pvrtc")]
fn test_is_multi_compression_container_pvrtc() {
    let pvr_type = ImageType::Pvrtc(PvrtcCompression::Etc2Rgb);
    assert!(pvr_type.is_multi_compression_container());
}

#[test]
#[cfg(feature = "etc2")]
fn test_is_multi_compression_container_pkm() {
    let etc2_type = ImageType::Etc2(PkmCompression::Etc1);
    assert!(!etc2_type.is_multi_compression_container()); // PKM is single-compression
}

#[test]
fn test_is_multi_compression_container_simple() {
    #[cfg(feature = "png")]
    {
        let png_type = ImageType::Png;
        assert!(!png_type.is_multi_compression_container());
    }
}

#[test]
#[cfg(all(feature = "dds", feature = "pvrtc"))]
fn test_cross_container_compression_family() {
    // Test that same compression families are identified across different containers
    let dds_bc1 = ImageType::Dds(DdsCompression::Bc1);
    let dds_bc7 = ImageType::Dds(DdsCompression::Bc7);

    assert_eq!(dds_bc1.compression_family(), dds_bc7.compression_family());
    assert_eq!(
        dds_bc1.compression_family(),
        Some(CompressionFamily::BlockCompression)
    );

    // ETC2 in PowerVR container should be same family as PKM ETC2
    let pvr_etc2 = ImageType::Pvrtc(PvrtcCompression::Etc2Rgb);
    assert_eq!(pvr_etc2.compression_family(), Some(CompressionFamily::Etc));
}

// Integration test using real data
#[test]
#[cfg(feature = "dds")]
fn test_helper_methods_with_real_data() {
    // Test with a real DDS file if it exists
    let test_files = [
        "tests/images/dds/compressions/bc1.dds",
        "tests/images/dds/compressions/bc7.dds",
    ];

    for file_path in &test_files {
        if std::path::Path::new(file_path).exists() {
            let file_data = std::fs::read(file_path).unwrap();

            match image_type(&file_data) {
                Ok(img_type) => {
                    // Should identify as block compression
                    assert_eq!(
                        img_type.compression_family(),
                        Some(CompressionFamily::BlockCompression)
                    );
                    assert!(img_type.is_block_compressed());
                    assert_eq!(img_type.container_format(), Some("DDS"));
                    assert!(img_type.is_multi_compression_container());
                }
                Err(e) => panic!("Failed to detect image type for {file_path}: {e:?}"),
            }
        }
    }
}

#[test]
#[cfg(feature = "pvrtc")]
fn test_helper_methods_with_real_pvrtc_data() {
    // Test with real PowerVR files
    let test_cases = [
        (
            "tests/images/pvrtc/compressions/pvrtc_2bpp_rgb.pvr",
            CompressionFamily::Pvrtc,
        ),
        (
            "tests/images/pvrtc/compressions/etc2_rgb.pvr",
            CompressionFamily::Etc,
        ),
        (
            "tests/images/pvrtc/compressions/eac_r.pvr",
            CompressionFamily::Eac,
        ),
    ];

    for (file_path, expected_family) in &test_cases {
        if std::path::Path::new(file_path).exists() {
            let file_data = std::fs::read(file_path).unwrap();

            match image_type(&file_data) {
                Ok(img_type) => {
                    assert_eq!(img_type.compression_family(), Some(*expected_family));
                    assert_eq!(img_type.container_format(), Some("PowerVR"));
                    assert!(img_type.is_multi_compression_container());
                }
                Err(e) => panic!("Failed to detect image type for {file_path}: {e:?}"),
            }
        }
    }
}
