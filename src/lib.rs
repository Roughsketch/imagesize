use std::fs::File;
use std::path::Path;
use std::io::Read;
use std::io::{BufRead, BufReader};

#[cfg(test)] 
mod test;

/// An Error type used in failure cases.
#[derive(Debug)]
pub enum ImageError {
    /// Used when the given data is not a supported format.
    NotSupported(String),
    /// Used when the image has an invalid format.
    CorruptedImage(String),
    /// Used when an IoError occurs when trying to read the given data.
    IoError(std::io::Error),
}

impl From<std::io::Error> for ImageError {
    fn from(err: std::io::Error) -> ImageError {
        ImageError::IoError(err)
    }
}

pub type ImageResult<T> = Result<T, ImageError>;

pub enum ImageType {
    Bmp,
    Gif,
    Jpeg,
    Png,
    Webp,
}

/// Holds the size information of an image.
#[derive(Debug)]
pub struct ImageSize {
    /// Width of an image in pixels.
    pub width: usize,
    /// Height of an image in pixels.
    pub height: usize,
}

/// Get the image type from a header
///
/// # Arguments
/// * `header` - The header of the file.
///
/// # Remarks
///
/// This will check the header to determine what image type the data is.
pub fn image_type(header: &[u8]) -> ImageResult<ImageType> {
    if header.len() >= 2 {
        if &header[0..2] == b"\x42\x4D" {
            return Ok(ImageType::Bmp);
        } else if &header[0..2] == b"\xFF\xD8" {
            if header.len() >= 3 {
                return if &header[2..3] == b"\xFF" {
                    Ok(ImageType::Jpeg)
                } else {
                    not_supported()
                };
            }
        } else if &header[0..2] == b"\x89P" {
            if header.len() >= 4 {
                return if &header[2..4] == b"NG" {
                    Ok(ImageType::Png)
                } else {
                    not_supported()
                };
            }
        } else if &header[0..2] == b"GI" {
            if header.len() >= 4 {
                return if &header[2..4] == b"F8" {
                    Ok(ImageType::Gif)
                } else {
                    not_supported()
                };
            }
        } else if &header[0..2] == b"RI" {
            if header.len() >= 12 {
                return if &header[2..4] == b"FF" && &header[8..12] == b"WEBP" {
                    Ok(ImageType::Webp)
                } else {
                    not_supported()
                };
            }
        } else {
            return not_supported();
        }
    }

    fn not_supported() -> ImageResult<ImageType> {
        Err(ImageError::NotSupported("Could not decode image.".into()))
    }

    Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "Not enough data").into())
}

/// Calls the correct image size method based on the image type
///
/// # Arguments
/// * `reader` - A reader for the data
/// * `header` - The header of the file
fn dispatch_header<R: BufRead>(reader: &mut R, header: &[u8]) -> ImageResult<ImageSize> {
    match image_type(&header)? {
        ImageType::Bmp => bmp_size(reader, header.len()),
        ImageType::Gif => gif_size(header),
        ImageType::Jpeg => jpeg_size(reader, header.len()),
        ImageType::Png => png_size(reader, header.len()),
        ImageType::Webp => {
            if header[15] == b' ' {
                webp_vp8_size(reader, header.len())
            } else {
                webp_vp8x_size(reader, header.len())
            }
        }
    }
}

/// Get the image size from a local file
///
/// # Arguments
/// * `path` - A local path to the file to parse.
///
/// # Remarks
///
/// Will try to read as little of the file as possible in order to get the
/// proper size information.
///
/// # Error
///
/// This method will return an [`ImageError`] under the following conditions:
///
/// * The header isn't recognized as a supported image
/// * The data isn't long enough to find the size for the given format
///
/// # Examples
///
/// ```
/// use imagesize::size;
///
/// match size("test/test.webp") {
///     Ok(dim) => {
///         assert_eq!(dim.width, 716);
///         assert_eq!(dim.height, 716);
///     }
///     Err(why) => println!("Error getting size: {:?}", why)
/// }
/// ```
/// 
/// [`ImageError`]: enum.ImageError.html
pub fn size<P>(path: P) -> ImageResult<ImageSize> where P: AsRef<Path> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    let mut header = [0; 16];
    reader.read_exact(&mut header)?;

    dispatch_header(&mut reader, &header)
}

/// Get the image size from a block of raw data.
///
/// # Arguments
/// * `data` - A Vec containing the data to parse for image size.
///
/// # Error
///
/// This method will return an [`ImageError`] under the following conditions:
///
/// * The header isn't recognized as a supported image
/// * The data isn't long enough to find the size for the given format
///
/// # Examples
///
/// ```
/// use imagesize::blob_size;
///
/// // First few bytes of arbitrary data.
/// let data = vec![0x89, 0x89, 0x89, 0x89, 0x0D, 0x0A, 0x1A, 0x0A,
///                 0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52,
///                 0x00, 0x00, 0x00, 0x7B, 0x01, 0x00, 0x01, 0x41,
///                 0x08, 0x06, 0x00, 0x00, 0x00, 0x9A, 0x38, 0xC4];
///
/// assert_eq!(blob_size(&data).is_err(), true);
/// ```
/// 
/// [`ImageError`]: enum.ImageError.html
pub fn blob_size(data: &[u8]) -> ImageResult<ImageSize> {
    let mut reader = BufReader::new(&data[..]);

    let mut header = [0; 16];
    reader.read_exact(&mut header)?;

    dispatch_header(&mut reader, &header)
}

fn bmp_size<R: BufRead>(reader: &mut R, offset: usize) -> ImageResult<ImageSize> {
    let mut buffer = [0; 8];
    reader.consume(0x12 - offset);
    reader.read_exact(&mut buffer)?;

    Ok(ImageSize {
        width:  ((buffer[0] as usize) |
                ((buffer[1] as usize) << 8) |
                ((buffer[2] as usize) << 16) |
                ((buffer[3] as usize) << 24)),

        height: ((buffer[4] as usize) |
                ((buffer[5] as usize) << 8) |
                ((buffer[6] as usize) << 16) |
                ((buffer[7] as usize) << 24))
    })
}

fn gif_size(header: &[u8]) -> ImageResult<ImageSize> {
    Ok(ImageSize {
        width:  ((header[6] as usize) | ((header[7] as usize) << 8)),
        height: ((header[8] as usize) | ((header[9] as usize) << 8))
    })
}

fn jpeg_size<R: BufRead>(reader: &mut R, _offset: usize) -> ImageResult<ImageSize> {
    let mut search = Vec::new();
    let mut buffer = [0; 4];
    let mut page = [0; 1];
    let mut depth = 0i32;

    loop {
        //  Read until it hits the next potential marker
        reader.read_until(0xFF, &mut search)?;

        reader.read_exact(&mut page)?;
        if page[0] == 0xC0 || page[0] == 0xC2 {
            //  Only get outside image size
            if depth == 0 {
                //  Correct marker, go forward 3 bytes so we're at height offset
                reader.consume(3);
                break;
            }
        } else if page[0] == 0xD8 {
            depth += 1;
        } else if page[0] == 0xD9 {
            depth -= 1;
            if depth < 0 {
                return Err(ImageError::CorruptedImage("Hit end of file before finding size.".into()));
            }
        }

        reader.consume(1);
    }

    reader.read_exact(&mut buffer)?;

    Ok(ImageSize {
        width:  ((buffer[3] as usize) | ((buffer[2] as usize) << 8)),
        height: ((buffer[1] as usize) | ((buffer[0] as usize) << 8))
    })
}

fn png_size<R: BufRead>(reader: &mut R, offset: usize) -> ImageResult<ImageSize> {
    let mut buffer = [0; 8];
    reader.consume(16 - offset);
    reader.read_exact(&mut buffer)?;

    Ok(ImageSize {
        width:  ((buffer[3] as usize) |
                ((buffer[2] as usize) << 8) |
                ((buffer[1] as usize) << 16) |
                ((buffer[0] as usize) << 24)),

        height: ((buffer[7] as usize) |
                ((buffer[6] as usize) << 8) |
                ((buffer[5] as usize) << 16) |
                ((buffer[4] as usize) << 24))
    })
}

fn webp_vp8x_size<R: BufRead>(reader: &mut R, offset: usize) -> ImageResult<ImageSize> {
    let mut buffer = [0; 6];
    reader.consume(0x18 - offset);
    reader.read_exact(&mut buffer)?;

    Ok(ImageSize {
        width:  ((buffer[0] as usize) |
                ((buffer[1] as usize) << 8) |
                ((buffer[2] as usize) << 16)) + 1,

        height: ((buffer[3] as usize) |
                ((buffer[4] as usize) << 8) |
                ((buffer[5] as usize) << 16)) + 1
    })
}

fn webp_vp8_size<R: BufRead>(reader: &mut R, offset: usize) -> ImageResult<ImageSize> {
    let mut buffer = [0; 6];
    reader.consume(0x1A - offset);
    reader.read_exact(&mut buffer)?;

    Ok(ImageSize {
        width:  ((buffer[0] as usize) |
                ((buffer[1] as usize) << 8)),

        height: ((buffer[2] as usize) |
                ((buffer[3] as usize) << 8))
    })
}