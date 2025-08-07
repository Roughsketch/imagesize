use std::io::{BufRead, Seek, SeekFrom};

use crate::{
    util::{read_u32, read_u64, Endian},
    ImageResult, ImageSize,
};

/// Compression formats for PVRTC containers
///
/// PowerVR containers can contain different compression formats beyond just PVRTC.
/// This enum identifies the specific compression algorithm used within the PowerVR container.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PvrtcCompression {
    /// PVRTC 2 bits per pixel RGB
    Pvrtc2BppRgb,
    /// PVRTC 2 bits per pixel RGBA
    Pvrtc2BppRgba,
    /// PVRTC 4 bits per pixel RGB
    Pvrtc4BppRgb,
    /// PVRTC 4 bits per pixel RGBA
    Pvrtc4BppRgba,
    /// ETC2 RGB compression
    Etc2Rgb,
    /// ETC2 RGBA compression  
    Etc2Rgba,
    /// ETC2 RGB with 1-bit alpha
    Etc2RgbA1,
    /// EAC R11 (single channel)
    EacR11,
    /// EAC RG11 (dual channel)
    EacRg11,
    /// Other/Unknown format
    Unknown,
}

pub fn size<R: BufRead + Seek>(reader: &mut R) -> ImageResult<ImageSize> {
    // PVRTC (PowerVR Texture Compression) format supports:
    // - PVRTCI-2bpp: 2 bits per pixel compression
    // - PVRTCI-4bpp: 4 bits per pixel compression
    // Both RGB and RGBA variants are supported
    // PVRTC header structure:
    // Header size: 4 bytes (little-endian)
    // Height: 4 bytes (little-endian)
    // Width: 4 bytes (little-endian)
    // Mipmap count: 4 bytes (little-endian)
    // Flags: 4 bytes (little-endian)
    // Data length: 4 bytes (little-endian)
    // Bits per pixel: 4 bytes (little-endian) - indicates 2bpp or 4bpp
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

pub fn detect_compression<R: BufRead + Seek>(reader: &mut R) -> ImageResult<PvrtcCompression> {
    // Read the pixel format to determine compression
    // PVR v3 format layout:
    // 0-3: Magic "PVR\x03"
    // 4-7: Flags
    // 8-15: Pixel format (8 bytes)
    reader.seek(SeekFrom::Start(8))?; // Skip to pixel format field
    let pixel_format = read_u64(reader, &Endian::Little)?;

    let compression = match pixel_format {
        0 => PvrtcCompression::Pvrtc2BppRgb,  // PVRTCI_2BPP_RGB
        1 => PvrtcCompression::Pvrtc2BppRgba, // PVRTCI_2BPP_RGBA
        2 => PvrtcCompression::Pvrtc4BppRgb,  // PVRTCI_4BPP_RGB
        3 => PvrtcCompression::Pvrtc4BppRgba, // PVRTCI_4BPP_RGBA
        22 => PvrtcCompression::Etc2Rgb,      // ETC2_RGB
        23 => PvrtcCompression::Etc2Rgba,     // ETC2_RGBA
        24 => PvrtcCompression::Etc2RgbA1,    // ETC2_RGB_A1
        25 => PvrtcCompression::EacR11,       // EAC_R11
        26 => PvrtcCompression::EacRg11,      // EAC_RG11
        _ => PvrtcCompression::Unknown,
    };

    Ok(compression)
}
