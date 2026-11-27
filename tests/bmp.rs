#[cfg(test)]
use imagesize::{size, ImageError, ImageSize};

#[test]
#[cfg(feature = "bmp")]
fn bmp_test() {
    let dim = size("tests/images/bmp/test.bmp").unwrap();
    assert_eq!(
        dim,
        ImageSize {
            width: 512,
            height: 512
        }
    );
}

#[cfg(feature = "bmp")]
fn bmp_header(dib: &[u8]) -> Vec<u8> {
    let mut data = b"BM".to_vec();
    data.extend_from_slice(&[0; 8]); // file size + reserved
    data.extend_from_slice(&(14 + dib.len() as u32).to_le_bytes()); // pixel data offset
    data.extend_from_slice(dib);
    data
}

#[test]
#[cfg(feature = "bmp")]
fn bmp_top_down_test() {
    // BITMAPINFOHEADER with a negative height (top-down rows)
    let mut dib = 40u32.to_le_bytes().to_vec();
    dib.extend_from_slice(&17i32.to_le_bytes());
    dib.extend_from_slice(&(-9i32).to_le_bytes());
    dib.extend_from_slice(&[0; 28]);

    let dim = imagesize::blob_size(&bmp_header(&dib)).unwrap();
    assert_eq!(
        dim,
        ImageSize {
            width: 17,
            height: 9
        }
    );
}

#[test]
#[cfg(feature = "bmp")]
fn bmp_core_header_test() {
    // OS/2 BITMAPCOREHEADER: 16-bit width and height
    let mut dib = 12u32.to_le_bytes().to_vec();
    dib.extend_from_slice(&300u16.to_le_bytes());
    dib.extend_from_slice(&200u16.to_le_bytes());
    dib.extend_from_slice(&1u16.to_le_bytes()); // planes
    dib.extend_from_slice(&24u16.to_le_bytes()); // bits per pixel

    let dim = imagesize::blob_size(&bmp_header(&dib)).unwrap();
    assert_eq!(
        dim,
        ImageSize {
            width: 300,
            height: 200
        }
    );
}

#[test]
#[cfg(feature = "bmp")]
fn bmp_invalid_dimensions_test() {
    for (width, height) in [(17i32, 0i32), (0, 9), (-17, 9)] {
        let mut dib = 40u32.to_le_bytes().to_vec();
        dib.extend_from_slice(&width.to_le_bytes());
        dib.extend_from_slice(&height.to_le_bytes());
        dib.extend_from_slice(&[0; 28]);

        assert!(matches!(
            imagesize::blob_size(&bmp_header(&dib)),
            Err(ImageError::CorruptedImage)
        ));
    }
}
