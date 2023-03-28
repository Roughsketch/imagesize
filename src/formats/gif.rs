use crate::{ImageResult, ImageSize};

pub fn size(header: &[u8]) -> ImageResult<ImageSize> {
    Ok(ImageSize {
        width: ((header[6] as usize) | ((header[7] as usize) << 8)),
        height: ((header[8] as usize) | ((header[9] as usize) << 8)),
    })
}

pub fn matches(header: &[u8]) -> bool {
    header.starts_with(b"GIF8")
}
