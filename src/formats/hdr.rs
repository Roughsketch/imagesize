use std::io::{self, BufRead, Seek, SeekFrom};

use crate::{
    util::read_line_capped,
    ImageResult, ImageSize
};

pub fn size<R: BufRead + Seek>(reader: &mut R) -> ImageResult<ImageSize> {
    reader.seek(SeekFrom::Start(0))?;

    // Read the first line and check if it's a valid HDR format identifier
    // Only read max of 11 characters which is max for longest valid header
    let format_identifier = read_line_capped(reader, 11)?;

    if !format_identifier.starts_with("#?RADIANCE") && !format_identifier.starts_with("#?RGBE") {
        return Err(
            io::Error::new(io::ErrorKind::InvalidData, "Invalid HDR format identifier").into(),
        );
    }

    loop {
        // Assuming no line will ever go above 256. Just a random guess at the moment.
        // If a line goes over the capped length we will return InvalidData which I think
        // is better than potentially reading a malicious file and exploding memory usage.
        let line = read_line_capped(reader, 256)?;

        if line.trim().is_empty() {
            continue;
        }

        // HDR image dimensions can be stored in 8 different ways based on orientation
        // Using EXIF orientation as a reference: 
        // https://web.archive.org/web/20220924095433/https://sirv.sirv.com/website/exif-orientation-values.jpg
        //
        // -Y N +X M => Standard orientation (EXIF 1)
        // -Y N -X M => Flipped horizontally (EXIF 2)
        // +Y N -X M => Flipped vertically and horizontally (EXIF 3)
        // +Y N +X M => Flipped vertically (EXIF 4)
        // +X M -Y N => Rotate 90 CCW and flip vertically (EXIF 5)
        // -X M -Y N => Rotate 90 CCW (EXIF 6)
        // -X M +Y N => Rotate 90 CW and flip vertically (EXIF 7)
        // +X M +Y N => Rotate 90 CW (EXIF 8)

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
