use std::io::{BufRead, Seek, SeekFrom};

use crate::{
    util::{read_u16, Endian},
    ImageResult, ImageSize,
};

pub fn size<R: BufRead + Seek>(reader: &mut R) -> ImageResult<ImageSize> {
    // EAC files also use PKM format like ETC2
    // PKM header structure:
    // Magic: "PKM " (4 bytes)
    // Version: "10" or "20" (2 bytes)
    // Data type: 2 bytes (big-endian) - EAC types are 0x160A (EAC_R11), 0x160B (EAC_RG11), 0x1609 (EAC_R11_SIGNED), 0x1608 (EAC_RG11_SIGNED)
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
    // PKM format with EAC-specific data types
    if header.len() >= 8
        && header.starts_with(b"PKM ")
        && (header[4..6] == [b'1', b'0'] || header[4..6] == [b'2', b'0'])
    {
        // Check data type for EAC formats
        let data_type = u16::from_be_bytes([header[6], header[7]]);
        return matches!(data_type, 0x1608..=0x160B);
    }
    false
}
