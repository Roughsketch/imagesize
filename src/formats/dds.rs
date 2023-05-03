use std::io::{self, BufRead, Seek, SeekFrom};

use crate::{ImageResult, ImageSize};

pub fn size<R: BufRead + Seek>(reader: &mut R) -> ImageResult<ImageSize> {
    reader.seek(SeekFrom::Start(0))?;

    let mut magic_number = [0; 4];
    reader.read_exact(&mut magic_number)?;
    if &magic_number != b"DDS " {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid DDS magic number").into());
    }

    let mut header = [0; 124];
    reader.read_exact(&mut header)?;

    let height = u32::from_le_bytes([header[8], header[9], header[10], header[11]]) as usize;
    let width = u32::from_le_bytes([header[12], header[13], header[14], header[15]]) as usize;

    Ok(ImageSize { width, height })
}

pub fn matches(header: &[u8]) -> bool {
    header.starts_with(b"DDS")
}
