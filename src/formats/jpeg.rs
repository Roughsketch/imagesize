use crate::util::*;
use crate::{ImageError, ImageResult, ImageSize};

use std::io::{BufRead, Seek, SeekFrom};

pub fn size<R: BufRead + Seek>(reader: &mut R) -> ImageResult<ImageSize> {
    // Go to the first marker after SOI (FF D8).
    reader.seek(SeekFrom::Start(2))?;

    loop {
        if read_u8(reader)? != 0xFF {
            return Err(ImageError::CorruptedImage);
        }

        // T.81 B.1.1.2 allows FF fill bytes before any marker.
        // https://www.w3.org/Graphics/JPEG/itu-t81.pdf
        let mut marker = read_u8(reader)?;
        while marker == 0xFF {
            marker = read_u8(reader)?;
        }

        match marker {
            // TEM stands alone and has no length field.
            0x01 => continue,
            // FF 00 is byte stuffing in scan data, not a header marker.
            // RST, another SOI, EOI, and SOS are unexpected before a frame.
            0x00 | 0xD0..=0xDA => return Err(ImageError::CorruptedImage),
            _ => {}
        }

        // Segment lengths include their own two bytes, but not the marker.
        let segment_length = read_u16(reader, &Endian::Big)?;
        if segment_length < 2 {
            return Err(ImageError::CorruptedImage);
        }

        // Check for valid SOFn markers. C4, C8, and CC aren't dimension markers.
        if (0xC0..=0xC3).contains(&marker)
            || (0xC5..=0xC7).contains(&marker)
            || (0xC9..=0xCB).contains(&marker)
            || (0xCD..=0xCF).contains(&marker)
        {
            // A frame declares at least one component: 8 + 3 * 1 bytes.
            if segment_length < 11 {
                return Err(ImageError::CorruptedImage);
            }

            read_u8(reader)?; // Sample precision.
            return Ok(ImageSize {
                height: read_u16(reader, &Endian::Big)? as usize,
                width: read_u16(reader, &Endian::Big)? as usize,
            });
        }

        // Payloads may contain marker-like bytes or embedded JPEGs; skip them intact.
        reader.seek(SeekFrom::Current(i64::from(segment_length - 2)))?;
    }
}

pub fn matches(header: &[u8]) -> bool {
    header.starts_with(b"\xFF\xD8\xFF")
}
