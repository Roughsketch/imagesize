use std::io::{BufRead, Seek, SeekFrom};

use crate::{
    util::{read_u16, Endian},
    ImageResult, ImageSize,
};

pub fn size<R: BufRead + Seek>(reader: &mut R) -> ImageResult<ImageSize> {
    // ETC2 files are typically in PKM format
    // PKM header structure:
    // Magic: "PKM " (4 bytes)
    // Version: "10" or "20" (2 bytes)
    // Data type: 2 bytes (big-endian)
    // Extended width: 2 bytes (big-endian)
    // Extended height: 2 bytes (big-endian)
    // Original width: 2 bytes (big-endian)
    // Original height: 2 bytes (big-endian)

    reader.seek(SeekFrom::Start(8))?; // Skip magic + version + data type
    let _extended_width = read_u16(reader, &Endian::Big)?;
    let _extended_height = read_u16(reader, &Endian::Big)?;
    let width = read_u16(reader, &Endian::Big)? as usize;
    let height = read_u16(reader, &Endian::Big)? as usize;

    Ok(ImageSize { width, height })
}

pub fn matches(header: &[u8]) -> bool {
    // PKM format magic number followed by version
    if header.len() >= 6 {
        return header.starts_with(b"PKM ")
            && (header[4..6] == [b'1', b'0'] || header[4..6] == [b'2', b'0']);
    }
    false
}
