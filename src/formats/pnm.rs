use crate::util::*;
use crate::{ImageResult, ImageSize};

use std::io::{self, BufRead, Seek, SeekFrom};

pub fn size<R: BufRead + Seek>(reader: &mut R) -> ImageResult<ImageSize> {
    reader.seek(SeekFrom::Start(2))?;

    loop {
        // Lines can be arbitrarily long, but 1k is a good enough cap I think.
        // Anything higher and I blame whoever made the file.
        let line = read_line_capped(reader, 1024)?;
        let trimmed_line = line.trim();

        if trimmed_line.is_empty() || trimmed_line.starts_with('#') {
            continue;
        }

        let dimensions: Vec<&str> = trimmed_line.split_whitespace().collect();
        if dimensions.len() != 2 && dimensions.len() != 3 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid PNM dimensions line",
            )
            .into());
        }

        let width_parsed = dimensions[0].parse::<usize>().ok();
        let height_parsed = dimensions[1].parse::<usize>().ok();

        if let (Some(width), Some(height)) = (width_parsed, height_parsed) {
            return Ok(ImageSize { width, height });
        }

        break;
    }

    Err(io::Error::new(io::ErrorKind::InvalidData, "PNM dimensions not found").into())
}

pub fn matches(header: &[u8]) -> bool {
    if header[0] != b'P' {
        return false;
    }

    // We only support P1 to P6. Currently ignoring P7, PF, PFM
    if header[1] < b'1' && header[1] > b'6' {
        return false;
    }

    true
}
