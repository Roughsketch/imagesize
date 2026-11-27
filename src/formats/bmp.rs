use crate::util::*;
use crate::{ImageError, ImageResult, ImageSize};

use std::io::{BufRead, Seek, SeekFrom};

pub fn size<R: BufRead + Seek>(reader: &mut R) -> ImageResult<ImageSize> {
    reader.seek(SeekFrom::Start(0x0E))?;
    let header_size = read_u32(reader, &Endian::Little)?;

    // The OS/2 BITMAPCOREHEADER stores the dimensions as two u16 values
    if header_size == 12 {
        return Ok(ImageSize {
            width: read_u16(reader, &Endian::Little)? as usize,
            height: read_u16(reader, &Endian::Little)? as usize,
        });
    }

    let width = read_i32(reader, &Endian::Little)?;
    let height = read_i32(reader, &Endian::Little)?;
    if width <= 0 || height == 0 {
        return Err(ImageError::CorruptedImage);
    }

    // A negative height marks a top-down bitmap
    Ok(ImageSize {
        width: width as usize,
        height: height.unsigned_abs() as usize,
    })
}

pub fn matches(header: &[u8]) -> bool {
    header.starts_with(b"\x42\x4D")
}
