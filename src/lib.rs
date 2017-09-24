use std::fs::File;
use std::path::Path;
use std::io::Read;
use std::io::{BufRead, BufReader};

#[cfg(test)] mod test;

/// An Error type used in failure cases.
#[derive(Debug)]
pub enum ImageError {
    /// Used when the given data is not a supported format.
    NotSupported(String),
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
    Unknown,
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

/// Get the image type from a header byte
///
/// # Arguments
/// * `header` - The first byte of the file to check
///
/// # Remarks
/// 
/// This is an unsafe way to get the image type. It will match on the first
/// byte of the file to guess the format, but with so little information it
/// is still possible for it to be wrong. Use `image_type_safe` to include
/// some extra checks.
fn image_type(header: u8) -> ImageType {
    match header {
        0xFF => ImageType::Jpeg,
        0x89 => ImageType::Png,
        b'R' => ImageType::Webp,
        b'G' => ImageType::Gif,
        b'B' => ImageType::Bmp,
        _ => ImageType::Unknown,
    }
}

/// Calls the correct image size method based on the unsafe image type
///
/// # Arguments
/// * `reader` - A reader for the data
/// * `header` - The header of the file
///
/// # Remarks
/// 
/// This is an unsafe way to get the image size. It will match on the first
/// byte of the file to guess the format, but with so little information it
/// is still possible for it to be wrong. Use `dispatch_header_safe` to include
/// some extra checks.
fn dispatch_header<R: BufRead>(reader: &mut R, header: &[u8]) -> ImageResult<ImageSize> {
    match image_type(header[0]) {
        ImageType::Bmp => bmp_size(reader, header.len()),
        ImageType::Gif => gif_size(reader, header.len()),
        ImageType::Jpeg => jpeg_size(reader, header.len()),
        ImageType::Png => png_size(reader, header.len()),
        ImageType::Webp => {
            let mut buf = [0; 15];
            reader.read_exact(&mut buf)?;

            if buf[14] == b' ' {
                webp_vp8_size(reader, header.len() + buf.len())
            } else {
                webp_vp8x_size(reader, header.len() + buf.len())
            }
        }
        ImageType::Unknown => Err(ImageError::NotSupported("Could not decode image.".to_owned())),
    }
}

/// Get the image type from a header
///
/// # Arguments
/// * `header` - The header of the file. Must be 12 bytes to be safe.
///
/// # Remarks
/// 
/// This will check the header to determine what image type the data is.
fn image_type_safe(header: &[u8]) -> ImageType {
    if &header[0..3] == b"\xFF\xD8\xFF" {
        ImageType::Jpeg
    } else if &header[0..4] == b"\x89PNG" {
        ImageType::Png
    } else if &header[0..4] == b"GIF8" {
        ImageType::Gif
    } else if &header[0..4] == b"RIFF" && &header[8..12] == b"WEBP" {
        ImageType::Webp
    } else if &header[0..2] == b"\x42\x4D" {
        ImageType::Bmp
    } else {
        ImageType::Unknown
    }
}

/// Calls the correct image size method based on the image type
///
/// # Arguments
/// * `reader` - A reader for the data
/// * `header` - The header of the file
///
/// # Remarks
/// 
/// Will use the safe method for getting the image type, and then call
/// the appropriate method to get the image size.
fn dispatch_header_safe<R: BufRead>(reader: &mut R, header: &[u8]) -> ImageResult<ImageSize> {
    match image_type_safe(&header) {
        ImageType::Bmp => bmp_size(reader, header.len()),
        ImageType::Gif => gif_size_from_header(header),
        ImageType::Jpeg => jpeg_size(reader, header.len()),
        ImageType::Png => png_size(reader, header.len()),
        ImageType::Webp => {
            if header[15] == b' ' {
                webp_vp8_size(reader, header.len())
            } else {
                webp_vp8x_size(reader, header.len())
            }
        }
        ImageType::Unknown => Err(ImageError::NotSupported("Could not decode image.".to_owned())),
    }
}

/// Get the image size from a local file.
///
/// # Arguments
/// * `path` - A local path to the file to parse.
///
/// # Remarks
/// 
/// This method will try to read as little of the file as possible in order to
/// get the proper size information.
///
/// # Error
///
/// This method will return an `ImageError` under the following conditions:
///
/// * The first byte of the header isn't recognized as a supported image
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
pub fn size<P>(path: P) -> ImageResult<ImageSize> where P: AsRef<Path> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    let mut header = [0; 1];
    reader.read_exact(&mut header)?;

    dispatch_header(&mut reader, &header)
}

/// Get the image size from a local file with extra
/// checks to ensure it's a valid image.
///
/// # Arguments
/// * `path` - A local path to the file to parse.
///
/// # Remarks
/// 
/// This method is the safe version of `size`. It is similar in that 
/// it will try to read as little of the file as possible in order to get the
/// proper size information, but it also adds extra checks to ensure it is
/// reading a valid image file.
///
/// # Error
///
/// This method will return an `ImageError` under the following conditions:
///
/// * The header isn't recognized as a supported image
/// * The data isn't long enough to find the size for the given format 
///
/// # Examples
///
/// ```
/// use imagesize::size_safe;
///
/// match size_safe("test/test.webp") {
///     Ok(dim) => {
///         assert_eq!(dim.width, 716);
///         assert_eq!(dim.height, 716);
///     }
///     Err(why) => println!("Error getting size: {:?}", why)
/// }
pub fn size_safe<P>(path: P) -> ImageResult<ImageSize> where P: AsRef<Path> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    let mut header = [0; 16];
    reader.read_exact(&mut header)?;

    dispatch_header_safe(&mut reader, &header)
}

/// Get the image size from a block of data.
///
/// # Arguments
/// * `data` - A Vec containing the data to parse for image size.
///
/// # Remarks
/// 
/// This method is useful when you need only the size of an image and have
/// a way to only read part of the data. For example, using the Range header
/// in a http request to receive the first part of an image file.
///
/// # Error
///
/// This method will return an `ImageError` under the following conditions:
///
/// * The first byte of the header isn't recognized as a supported image
/// * The data isn't long enough to find the size for the given format 
///
/// # Examples
///
/// ```
/// use imagesize::blob_size;
///
/// //  First 32 bytes of a PNG Header with size 123x321
/// let data = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 
///                 0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52, 
///                 0x00, 0x00, 0x00, 0x7B, 0x00, 0x00, 0x01, 0x41,
///                 0x08, 0x06, 0x00, 0x00, 0x00, 0x9A, 0x38, 0xC4];
///
/// match blob_size(&data) {
///     Ok(dim) => {
///         assert_eq!(dim.width, 123);
///         assert_eq!(dim.height, 321);
///     }
///     Err(why) => println!("Error getting size: {:?}", why)
/// }
/// ```
pub fn blob_size(data: &[u8]) -> ImageResult<ImageSize> {
    let mut reader = BufReader::new(&data[..]);

    let mut header = [0; 1];
    reader.read_exact(&mut header)?;

    dispatch_header(&mut reader, &header)
}

/// Get the image size from a block of data with extra checks to ensure
/// it's a valid image.
///
/// # Arguments
/// * `data` - A Vec containing the data to parse for image size.
///
/// # Remarks
/// 
/// This method is the same as `blob_size`, except it has added
/// checks to make sure that the given data is in fact an image. 
///
/// # Error
///
/// This method will return an `ImageError` under the following conditions:
///
/// * The header isn't recognized as a supported image
/// * The data isn't long enough to find the size for the given format 
///
/// # Examples
///
/// ```
/// use imagesize::blob_size_safe;
///
/// // First few bytes of arbitrary data.
/// // blob_size would assume this is a PNG with size 123x16777537
/// let data = vec![0x89, 0x89, 0x89, 0x89, 0x0D, 0x0A, 0x1A, 0x0A, 
///                 0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52, 
///                 0x00, 0x00, 0x00, 0x7B, 0x01, 0x00, 0x01, 0x41,
///                 0x08, 0x06, 0x00, 0x00, 0x00, 0x9A, 0x38, 0xC4];
///
/// assert_eq!(blob_size_safe(&data).is_err(), true);
/// ```
pub fn blob_size_safe(data: &[u8]) -> ImageResult<ImageSize> {
    let mut reader = BufReader::new(&data[..]);

    let mut header = [0; 16];
    reader.read_exact(&mut header)?;

    dispatch_header_safe(&mut reader, &header)
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

fn gif_size_from_header(header: &[u8]) -> ImageResult<ImageSize> {
    Ok(ImageSize {
        width:  ((header[6] as usize) | ((header[7] as usize) << 8)),
        height: ((header[8] as usize) | ((header[9] as usize) << 8))
    })
}

fn gif_size<R: BufRead>(reader: &mut R, offset: usize) -> ImageResult<ImageSize> {
    let mut buffer = [0; 4];
    reader.consume(6 - offset);
    reader.read_exact(&mut buffer)?;

    Ok(ImageSize {
        width:  ((buffer[0] as usize) | ((buffer[1] as usize) << 8)),
        height: ((buffer[2] as usize) | ((buffer[3] as usize) << 8))
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