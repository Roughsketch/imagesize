use std::io::{self, BufRead, Seek, SeekFrom};

use crate::{ImageResult, ImageSize};

pub fn size<R: BufRead + Seek>(reader: &mut R) -> ImageResult<ImageSize> {
    reader.seek(SeekFrom::Start(0))?;

    let mut identifier = [0; 12];
    reader.read_exact(&mut identifier)?;
    let ktx2_identifier = [
        0xAB, 0x4B, 0x54, 0x58, 0x20, 0x32, 0x30, 0xBB, 0x0D, 0x0A, 0x1A, 0x0A,
    ];

    if identifier != ktx2_identifier {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid KTX2 identifier").into());
    }

    let mut header = [0; 40];
    reader.read_exact(&mut header)?;

    let width = u32::from_le_bytes([header[4], header[5], header[6], header[7]]) as usize;
    let height = u32::from_le_bytes([header[8], header[9], header[10], header[11]]) as usize;

    Ok(ImageSize { width, height })
}

pub fn matches(header: &[u8]) -> bool {
    let ktx2_identifier = [
        0xAB, 0x4B, 0x54, 0x58, 0x20, 0x32, 0x30, 0xBB, 0x0D, 0x0A, 0x1A, 0x0A,
    ];
    header.starts_with(&ktx2_identifier)
}
