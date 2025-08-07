use std::io::{BufRead, Seek, SeekFrom};

use crate::{
    util::{read_u32, Endian},
    ImageResult, ImageSize,
};

/// Compression formats for DDS containers
///
/// DirectDraw Surface (DDS) files can contain various compressed and uncompressed formats.
/// This enum identifies the specific compression algorithm used within the DDS container.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DdsCompression {
    /// Block Compression 1 (DXT1) - RGB, 1-bit alpha
    Bc1,
    /// Block Compression 2 (DXT3) - RGBA with explicit alpha
    Bc2,
    /// Block Compression 3 (DXT5) - RGBA with interpolated alpha
    Bc3,
    /// Block Compression 4 (ATI1) - Single channel
    Bc4,
    /// Block Compression 5 (ATI2) - Two channel (RG)
    Bc5,
    /// Block Compression 6H - HDR format
    Bc6h,
    /// Block Compression 7 - High quality RGB/RGBA
    Bc7,
    /// Uncompressed RGBA32
    Rgba32,
    /// Uncompressed RGB24
    Rgb24,
    /// Other/Unknown DDS format
    Unknown,
}

pub fn size<R: BufRead + Seek>(reader: &mut R) -> ImageResult<ImageSize> {
    reader.seek(SeekFrom::Start(12))?;
    let height = read_u32(reader, &Endian::Little)? as usize;
    let width = read_u32(reader, &Endian::Little)? as usize;
    Ok(ImageSize { width, height })
}

pub fn matches(header: &[u8]) -> bool {
    header.starts_with(b"DDS ")
}

pub fn detect_compression<R: BufRead + Seek>(reader: &mut R) -> ImageResult<DdsCompression> {
    // DDS header structure:
    // Signature: "DDS " (4 bytes)
    // Header size: 124 (4 bytes)
    // Flags: various flags (4 bytes)
    // Height, Width: (4 bytes each)
    // PitchOrLinearSize: (4 bytes)
    // Depth: (4 bytes)
    // MipMapCount: (4 bytes)
    // Reserved1: (44 bytes)
    // Pixel Format: (32 bytes)
    //   - Size: 32 (4 bytes)
    //   - Flags: (4 bytes)
    //   - FourCC: (4 bytes) - this tells us the compression format
    //   - RGBBitCount: (4 bytes)
    //   - RBitMask, GBitMask, BBitMask, ABitMask: (16 bytes)

    reader.seek(SeekFrom::Start(84))?; // Skip to pixel format FourCC
    let mut fourcc = [0u8; 4];
    reader.read_exact(&mut fourcc)?;

    let compression = match &fourcc {
        b"DXT1" => DdsCompression::Bc1,
        b"DXT3" => DdsCompression::Bc2,
        b"DXT5" => DdsCompression::Bc3,
        b"ATI1" | b"BC4U" | b"BC4S" => DdsCompression::Bc4,
        b"ATI2" | b"BC5U" | b"BC5S" => DdsCompression::Bc5,
        b"BC6H" => DdsCompression::Bc6h,
        b"BC7U" | b"BC7L" => DdsCompression::Bc7,
        b"DX10" => {
            // DX10 extended header starts right after the main DDS header (128 bytes from start)
            // Skip the rest of the pixel format, caps, and reserved2 fields first
            reader.seek(SeekFrom::Start(128))?; // Jump to DX10 extended header
            let dxgi_format = read_u32(reader, &Endian::Little)?;
            match dxgi_format {
                95 => DdsCompression::Bc6h, // DXGI_FORMAT_BC6H_UF16
                96 => DdsCompression::Bc6h, // DXGI_FORMAT_BC6H_SF16  
                98 => DdsCompression::Bc7,  // DXGI_FORMAT_BC7_UNORM
                99 => DdsCompression::Bc7,  // DXGI_FORMAT_BC7_UNORM_SRGB
                _ => DdsCompression::Unknown,
            }
        }
        [0, 0, 0, 0] => {
            // No FourCC, check if it's uncompressed
            // We need to check the RGB bit count and masks
            let rgb_bit_count = read_u32(reader, &Endian::Little)?;
            match rgb_bit_count {
                32 => DdsCompression::Rgba32,
                24 => DdsCompression::Rgb24,
                _ => DdsCompression::Unknown,
            }
        }
        _ => DdsCompression::Unknown,
    };

    Ok(compression)
}
