use std::io::{BufRead, Seek, SeekFrom};

use crate::{
    util::{read_u32, Endian},
    ImageResult, ImageSize,
};

pub fn size<R: BufRead + Seek>(reader: &mut R) -> ImageResult<ImageSize> {
    // PVRTC header structure:
    // Header size: 4 bytes (little-endian)
    // Height: 4 bytes (little-endian)
    // Width: 4 bytes (little-endian)
    // Mipmap count: 4 bytes (little-endian)
    // Flags: 4 bytes (little-endian)
    // Data length: 4 bytes (little-endian)
    // Bits per pixel: 4 bytes (little-endian)
    // Red mask: 4 bytes (little-endian)
    // Green mask: 4 bytes (little-endian)
    // Blue mask: 4 bytes (little-endian)
    // Alpha mask: 4 bytes (little-endian)
    // PVR magic: 4 bytes (little-endian) - should be "PVR!"
    // Surface count: 4 bytes (little-endian)

    reader.seek(SeekFrom::Start(4))?; // Skip header size
    let height = read_u32(reader, &Endian::Little)? as usize;
    let width = read_u32(reader, &Endian::Little)? as usize;

    Ok(ImageSize { width, height })
}

pub fn matches(header: &[u8]) -> bool {
    // PVRTC files can have different magic numbers:
    // Legacy format starts with header length (usually 52 bytes = 0x34000000 in little endian)
    // Modern format has "PVR!" magic at different offsets
    if header.len() >= 4 {
        // Check for legacy format (header size = 52)
        let header_size = u32::from_le_bytes([header[0], header[1], header[2], header[3]]);
        if header_size == 52 {
            return true;
        }
    }

    // Check for "PVR!" magic at various positions
    if header.len() >= 48 {
        let pvr_magic = &header[44..48];
        if pvr_magic == b"PVR!" {
            return true;
        }
    }

    // Check for "PVR\x03" which is the modern PVRTC format
    if header.len() >= 4 && header.starts_with(b"PVR\x03") {
        return true;
    }

    false
}
