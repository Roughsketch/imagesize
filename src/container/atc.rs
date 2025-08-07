use std::io::{BufRead, Seek, SeekFrom};

use crate::{
    util::{read_u16, read_u32, Endian},
    ImageResult, ImageSize,
};

/// Compression formats for ATC containers
///
/// Adaptive Texture Compression (ATC) is used primarily on Adreno GPUs.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AtcCompression {
    /// ATC RGB (no alpha)
    Rgb,
    /// ATC RGBA with explicit alpha
    RgbaExplicit,
    /// ATC RGBA with interpolated alpha
    RgbaInterpolated,
    /// Other/Unknown ATC format
    Unknown,
}

pub fn size<R: BufRead + Seek>(reader: &mut R) -> ImageResult<ImageSize> {
    // ATC files typically use DDS container format
    // But also can be in PKM format or custom ATC format

    // Try DDS format first
    let mut header = [0u8; 4];
    reader.read_exact(&mut header)?;

    if header == *b"DDS " {
        // DDS format - seek to dimensions
        reader.seek(SeekFrom::Start(12))?;
        let height = read_u32(reader, &Endian::Little)? as usize;
        let width = read_u32(reader, &Endian::Little)? as usize;
        return Ok(ImageSize { width, height });
    }

    // Try PKM format (used by some ATC implementations)
    reader.seek(SeekFrom::Start(0))?;
    let mut pkm_header = [0u8; 8];
    reader.read_exact(&mut pkm_header)?;

    if pkm_header.starts_with(b"PKM ")
        && (pkm_header[4..6] == [b'1', b'0'] || pkm_header[4..6] == [b'2', b'0'])
    {
        // Check for ATC-specific data types
        let data_type = u16::from_be_bytes([pkm_header[6], pkm_header[7]]);
        if matches!(data_type, 0x8C92 | 0x8C93 | 0x87EE) {
            // ATC_RGB, ATC_RGBA_EXPLICIT_ALPHA, and ATC_RGBA_INTERPOLATED_ALPHA
            reader.seek(SeekFrom::Start(8))?; // Skip magic + version + data type
            let _extended_width = read_u16(reader, &Endian::Big)?;
            let _extended_height = read_u16(reader, &Endian::Big)?;
            let width = read_u16(reader, &Endian::Big)? as usize;
            let height = read_u16(reader, &Endian::Big)? as usize;
            return Ok(ImageSize { width, height });
        }
    }

    // Fallback: assume basic ATC dimensions at a standard location
    reader.seek(SeekFrom::Start(4))?;
    let height = read_u32(reader, &Endian::Little)? as usize;
    let width = read_u32(reader, &Endian::Little)? as usize;
    Ok(ImageSize { width, height })
}

pub fn matches(header: &[u8]) -> bool {
    // Only check for PKM format with ATC data types
    // DDS files with ATC compression should be handled by the DDS format detector
    if header.len() >= 8
        && header.starts_with(b"PKM ")
        && (header[4..6] == [b'1', b'0'] || header[4..6] == [b'2', b'0'])
    {
        let data_type = u16::from_be_bytes([header[6], header[7]]);
        return matches!(data_type, 0x8C92 | 0x8C93 | 0x87EE);
    }

    false
}

pub fn detect_compression<R: BufRead + Seek>(reader: &mut R) -> ImageResult<AtcCompression> {
    // Check if it's a PKM format first
    let mut header = [0u8; 8];
    reader.seek(SeekFrom::Start(0))?;
    reader.read_exact(&mut header)?;

    if header.starts_with(b"PKM ") && (header[4..6] == [b'1', b'0'] || header[4..6] == [b'2', b'0'])
    {
        let data_type = u16::from_be_bytes([header[6], header[7]]);
        let compression = match data_type {
            0x8C92 => AtcCompression::Rgb,              // ATC_RGB
            0x8C93 => AtcCompression::RgbaExplicit,     // ATC_RGBA_EXPLICIT_ALPHA
            0x87EE => AtcCompression::RgbaInterpolated, // ATC_RGBA_INTERPOLATED_ALPHA
            _ => AtcCompression::Unknown,
        };
        return Ok(compression);
    }

    // Check if it's DDS format
    if header[0..4] == *b"DDS " {
        // For DDS, we'd need to check the pixel format section for ATC FourCC
        // This is a more complex check that would examine the DDS pixel format
        return Ok(AtcCompression::Unknown); // Default for DDS-contained ATC
    }

    Ok(AtcCompression::Unknown)
}
