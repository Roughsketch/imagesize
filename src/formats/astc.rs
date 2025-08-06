use std::io::{BufRead, Seek, SeekFrom};

use crate::{ImageResult, ImageSize};

pub fn size<R: BufRead + Seek>(reader: &mut R) -> ImageResult<ImageSize> {
    // ASTC header is 16 bytes total
    // Magic number: 0x13, 0x12, 0x10, 0x5C (4 bytes)
    // Block dimensions: blockdim_x, blockdim_y, blockdim_z (3 bytes) - skip these
    // Image dimensions: xsize (3 bytes), ysize (3 bytes), zsize (3 bytes)

    reader.seek(SeekFrom::Start(7))?; // Skip magic + block dimensions

    // Read 3-byte little-endian values for image dimensions
    let mut xsize_bytes = [0u8; 4];
    reader.read_exact(&mut xsize_bytes[0..3])?;
    let width = u32::from_le_bytes(xsize_bytes) as usize;

    let mut ysize_bytes = [0u8; 4];
    reader.read_exact(&mut ysize_bytes[0..3])?;
    let height = u32::from_le_bytes(ysize_bytes) as usize;

    Ok(ImageSize { width, height })
}

pub fn matches(header: &[u8]) -> bool {
    // ASTC magic number is 0x13 0xAB 0xA0 0x5C
    header.len() >= 4 && header[0..4] == [0x13, 0xAB, 0xA0, 0x5C]
}
