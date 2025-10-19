use std::io::{BufRead, Seek, SeekFrom};

use crate::{
    util::{read_u16, Endian},
    ImageResult, ImageSize,
};

/// Compression formats for PKM containers (ETC/EAC family)
///
/// PKM (PowerVR Texture Compression) format is used to store various ETC and EAC
/// compressed textures. This enum identifies the specific compression variant.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PkmCompression {
    /// ETC1 RGB (original format)
    Etc1,
    /// ETC2 RGB (enhanced)
    Etc2,
    /// ETC2 with 1-bit punch-through alpha
    Etc2A1,
    /// ETC2 with 8-bit alpha channel
    Etc2A8,
    /// EAC single channel (R11)
    EacR,
    /// EAC two channel (RG11)
    EacRg,
    /// EAC single channel signed (R11_SIGNED)
    EacRSigned,
    /// EAC two channel signed (RG11_SIGNED)
    EacRgSigned,
    /// Other/Unknown PKM format
    Unknown,
}

pub fn size<R: BufRead + Seek>(reader: &mut R) -> ImageResult<ImageSize> {
    // ETC/EAC files are typically in PKM format
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

pub fn matches_eac(header: &[u8]) -> bool {
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

pub fn detect_compression<R: BufRead + Seek>(reader: &mut R) -> ImageResult<PkmCompression> {
    // Read the data type from PKM header to determine compression format
    reader.seek(SeekFrom::Start(6))?; // Skip magic and version
    let data_type = read_u16(reader, &Endian::Big)?;

    let compression = match data_type {
        // ETC1 formats
        0x0000 => PkmCompression::Etc1, // ETC1_RGB_NO_MIPMAPS

        // ETC2 formats
        0x0001 => PkmCompression::Etc2, // ETC2PACKAGE_RGB_NO_MIPMAPS
        0x0002 => PkmCompression::Etc2A1, // ETC2PACKAGE_RGBA1_NO_MIPMAPS (alternative)
        0x0003 => PkmCompression::Etc2A8, // ETC2PACKAGE_RGBA_NO_MIPMAPS_OLD
        0x0004 => PkmCompression::Etc2A1, // ETC2PACKAGE_RGBA1_NO_MIPMAPS (generated format)
        0x0005 => PkmCompression::Etc2A8, // ETC2PACKAGE_RGBA_NO_MIPMAPS (generated format)

        // Standard EAC formats
        0x1608 => PkmCompression::EacRgSigned, // EAC_RG11_SIGNED_FORMAT
        0x1609 => PkmCompression::EacRSigned,  // EAC_R11_SIGNED_FORMAT
        0x160A => PkmCompression::EacR,        // EAC_R11_UNSIGNED_FORMAT
        0x160B => PkmCompression::EacRg,       // EAC_RG11_UNSIGNED_FORMAT

        _ => PkmCompression::Unknown,
    };

    Ok(compression)
}
