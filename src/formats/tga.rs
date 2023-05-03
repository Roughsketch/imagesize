use std::io::{BufRead, Seek, SeekFrom};

use crate::{ImageResult, ImageSize};

pub fn size<R: BufRead + Seek>(reader: &mut R) -> ImageResult<ImageSize> {
    reader.seek(SeekFrom::Start(0))?;

    let mut header = [0; 18];
    reader.read_exact(&mut header)?;

    let width = u16::from_le_bytes([header[12], header[13]]) as usize;
    let height = u16::from_le_bytes([header[14], header[15]]) as usize;

    Ok(ImageSize { width, height })
}

pub fn matches(header: &[u8]) -> bool {
    // Check the image type (byte 2) to be one of the uncompressed or RLE compressed types
    let image_type = header[2];
    if image_type != 1
        && image_type != 2
        && image_type != 3
        && image_type != 9
        && image_type != 10
        && image_type != 11
    {
        return false;
    }

    // Check that the colormap type (byte 1) is either 0 (no colormap) or 1 (colormap present)
    let colormap_type = header[1];
    if colormap_type != 0 && colormap_type != 1 {
        return false;
    }

    true
}
