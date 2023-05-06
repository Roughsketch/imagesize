use std::io::{self, BufRead, Seek, SeekFrom};

use crate::{ImageResult, ImageSize};

pub fn size<R: BufRead + Seek>(reader: &mut R) -> ImageResult<ImageSize> {
    reader.seek(SeekFrom::Start(0))?;

    // Read the first line and check if it's a valid HDR format identifier
    let mut format_identifier = String::new();
    reader.read_line(&mut format_identifier)?;
    if !format_identifier.starts_with("#?RADIANCE") && !format_identifier.starts_with("#?RGBE") {
        return Err(
            io::Error::new(io::ErrorKind::InvalidData, "Invalid HDR format identifier").into(),
        );
    }

    loop {
        let mut line = String::new();
        reader.read_line(&mut line)?;

        // Extract width and height information
        if line.trim().is_empty() || !line.starts_with("-Y") {
            continue;
        }

        if line.starts_with("-Y") {
            let dimensions: Vec<&str> = line.split_whitespace().collect();
            if dimensions.len() != 4 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Invalid HDR dimensions line",
                )
                .into());
            }

            let height = dimensions[1].parse::<usize>().ok();
            let width = dimensions[3].parse::<usize>().ok();

            match width.is_some() && height.is_some() {
                true => {
                    return Ok(ImageSize {
                        width: width.unwrap(),
                        height: height.unwrap(),
                    });
                }
                false => (),
            }

            break;
        }
    }

    Err(io::Error::new(io::ErrorKind::InvalidData, "HDR dimensions not found").into())
}

pub fn matches(header: &[u8]) -> bool {
    let radiance_header = b"#?RADIANCE\n";
    let rgbe_header = b"#?RGBE\n";

    header.starts_with(radiance_header) || header.starts_with(rgbe_header)
}
