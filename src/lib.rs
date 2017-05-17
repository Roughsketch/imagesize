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
    /// Used when an IoError occurs when trying to read the given data.
    IoError(std::io::Error),
}

impl From<std::io::Error> for ImageError {
    fn from(err: std::io::Error) -> ImageError {
        ImageError::IoError(err)
    }
}

pub type ImageResult<T> = Result<T, ImageError>;

/// Holds the size information of an image.
pub struct Dimensions {
    /// Width of an image in pixels.
    pub width: usize,
    /// Height of an image in pixels.
    pub height: usize,
}

/// Get the image dimensions from a local file.
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
/// * The data isn't long enough to find the dimensions for the given format 
///
/// # Examples
///
/// ```
/// use imagesize::get_dimensions;
///
/// match get_dimensions("test/test.webp") {
///     Ok(dim) => {
///         assert_eq!(dim.width, 716);
///         assert_eq!(dim.height, 716);
///     }
///     Err(why) => println!("Error getting dimensions: {:?}", why)
/// }
pub fn get_dimensions<P>(path: P) -> ImageResult<Dimensions> where P: AsRef<Path> {
    let file = try!(File::open(path));
    let mut reader = BufReader::new(file);

    let mut header = [0; 1];
    try!(reader.read_exact(&mut header));

    match header[0] {
        0xFF => get_jpeg_dimensions(&mut reader, 1),
        0x89 => get_png_dimensions(&mut reader, 1),
        b'R' => get_webp_vp8x_dimensions(&mut reader, 1),
        b'G' => get_gif_dimensions(&mut reader, 1),
        b'B' => get_bmp_dimensions(&mut reader, 1),
        _ => Err(ImageError::NotSupported("Could not decode image.".to_owned()))
    }
}

/// Get the image dimensions from a local file with extra
/// checks to ensure it's a valid image.
///
/// # Arguments
/// * `path` - A local path to the file to parse.
///
/// # Remarks
/// 
/// This method is the safe version of `get_dimensions`. It is similar in that 
/// it will try to read as little of the file as possible in order to get the
/// proper size information, but it also adds extra checks to ensure it is
/// reading a valid image file.
///
/// # Error
///
/// This method will return an `ImageError` under the following conditions:
///
/// * The header isn't recognized as a supported image
/// * The data isn't long enough to find the dimensions for the given format 
///
/// # Examples
///
/// ```
/// use imagesize::get_dimensions_safe;
///
/// match get_dimensions_safe("test/test.webp") {
///     Ok(dim) => {
///         assert_eq!(dim.width, 716);
///         assert_eq!(dim.height, 716);
///     }
///     Err(why) => println!("Error getting dimensions: {:?}", why)
/// }
pub fn get_dimensions_safe<P>(path: P) -> ImageResult<Dimensions> where P: AsRef<Path> {
    let file = try!(File::open(path));
    let mut reader = BufReader::new(file);

    let mut header = [0; 16];
    try!(reader.read_exact(&mut header));

    if header[0] == 0xFF && header[1] == 0xD8 && header[2] == 0xFF {
        get_jpeg_dimensions(&mut reader, 16)
    } else if header[0] == 0x89 && header[1] == b'P' && header[2] == b'N' && header[3] == b'G' {
        get_png_dimensions(&mut reader, 16)
    } else if header[0] == b'G' && header[1] == b'I' && header[2] == b'F' && header[3] == b'8' {
        get_gif_dimensions_from_header(&header)
    } else if header[0] == b'R' && header[1] == b'I' && header[2] == b'F' && header[3] == b'F' &&
              header[8] == b'W' && header[9] == b'E' && header[10] == b'B' && header[11] == b'P' {
        if header[15] == b' ' {
            get_webp_vp8_dimensions(&mut reader, 16)
        } else {
            get_webp_vp8x_dimensions(&mut reader, 16)
        }
    } else if header[0] == 0x42 && header[1] == 0x4D {
        get_bmp_dimensions(&mut reader, 16)
    } else {
        Err(ImageError::NotSupported("Could not decode image.".to_owned()))
    }
}

/// Get the image dimensions from a block of data.
///
/// # Arguments
/// * `data` - A Vec containing the data to parse for image dimensions.
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
/// * The data isn't long enough to find the dimensions for the given format 
///
/// # Examples
///
/// ```
/// use imagesize::get_dimensions_from_blob;
///
/// //  First 32 bytes of a PNG Header with size 123x321
/// let data = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 
///                 0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52, 
///                 0x00, 0x00, 0x00, 0x7B, 0x00, 0x00, 0x01, 0x41,
///                 0x08, 0x06, 0x00, 0x00, 0x00, 0x9A, 0x38, 0xC4];
///
/// match get_dimensions_from_blob(&data) {
///     Ok(dim) => {
///         assert_eq!(dim.width, 123);
///         assert_eq!(dim.height, 321);
///     }
///     Err(why) => println!("Error getting dimensions: {:?}", why)
/// }
/// ```
pub fn get_dimensions_from_blob(data: &[u8]) -> ImageResult<Dimensions> {
    let mut reader = BufReader::new(&data[..]);

    let mut header = [0; 1];
    try!(reader.read_exact(&mut header));

    match header[0] {
        0xFF => get_jpeg_dimensions(&mut reader, 1),
        0x89 => get_png_dimensions(&mut reader, 1),
        b'R' => get_webp_vp8x_dimensions(&mut reader, 1),
        b'G' => get_gif_dimensions(&mut reader, 1),
        b'B' => get_bmp_dimensions(&mut reader, 1),
        _ => Err(ImageError::NotSupported("Could not decode image.".to_owned()))
    }
}

/// Get the image dimensions from a block of data with extra checks to ensure
/// it's a valid image.
///
/// # Arguments
/// * `data` - A Vec containing the data to parse for image dimensions.
///
/// # Remarks
/// 
/// This method is the same as `get_dimensions_from_blob`, except it has added
/// checks to make sure that the given data is in fact an image. 
///
/// # Error
///
/// This method will return an `ImageError` under the following conditions:
///
/// * The header isn't recognized as a supported image
/// * The data isn't long enough to find the dimensions for the given format 
///
/// # Examples
///
/// ```
/// use imagesize::get_dimensions_from_blob_safe;
///
/// // First few bytes of arbitrary data.
/// // get_dimensions_from_blob would assume this is a PNG with size 123x16777537
/// let data = vec![0x89, 0x89, 0x89, 0x89, 0x0D, 0x0A, 0x1A, 0x0A, 
///                 0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52, 
///                 0x00, 0x00, 0x00, 0x7B, 0x01, 0x00, 0x01, 0x41,
///                 0x08, 0x06, 0x00, 0x00, 0x00, 0x9A, 0x38, 0xC4];
///
/// assert_eq!(get_dimensions_from_blob_safe(&data).is_err(), true);
/// ```
pub fn get_dimensions_from_blob_safe(data: &[u8]) -> ImageResult<Dimensions> {
    let mut reader = BufReader::new(&data[..]);

    let mut header = [0; 16];
    try!(reader.read_exact(&mut header));

    if header[0] == 0xFF && header[1] == 0xD8 && header[2] == 0xFF && header[3] == 0xE0 {
        get_jpeg_dimensions(&mut reader, 16)
    } else if header[0] == 0x89 && header[1] == b'P' && header[2] == b'N' && header[3] == b'G' {
        get_png_dimensions(&mut reader, 16)
    } else if header[0] == b'G' && header[1] == b'I' && header[2] == b'F' && header[3] == b'8' {
        get_gif_dimensions_from_header(&header)
    } else if header[0] == b'R' && header[1] == b'I' && header[2] == b'F' && header[3] == b'F' &&
              header[8] == b'W' && header[9] == b'E' && header[10] == b'B' && header[11] == b'P' {
        if header[15] == b' ' {
            get_webp_vp8_dimensions(&mut reader, 16)
        } else {
            get_webp_vp8x_dimensions(&mut reader, 16)
        }
    } else if header[0] == 0x42 && header[1] == 0x4D {
        get_bmp_dimensions(&mut reader, 16)
    } else {
        Err(ImageError::NotSupported("Could not decode image.".to_owned()))
    }
}

fn get_bmp_dimensions<R: BufRead>(reader: &mut R, offset: usize) -> ImageResult<Dimensions> {
    let mut buffer = [0; 8];
    reader.consume(0x12 - offset);
    try!(reader.read_exact(&mut buffer));

    Ok(Dimensions {
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

fn get_gif_dimensions_from_header(header: &[u8]) -> ImageResult<Dimensions> {
    Ok(Dimensions {
        width:  ((header[6] as usize) | ((header[7] as usize) << 8)),
        height: ((header[8] as usize) | ((header[9] as usize) << 8))
    })
}

fn get_gif_dimensions<R: BufRead>(reader: &mut R, offset: usize) -> ImageResult<Dimensions> {
    let mut buffer = [0; 4];
    reader.consume(6 - offset);
    try!(reader.read_exact(&mut buffer));

    Ok(Dimensions {
        width:  ((buffer[0] as usize) | ((buffer[1] as usize) << 8)),
        height: ((buffer[2] as usize) | ((buffer[3] as usize) << 8))
    })
}

fn get_jpeg_dimensions<R: BufRead>(reader: &mut R, _offset: usize) -> ImageResult<Dimensions> {
    let mut search = Vec::new();
    let mut buffer = [0; 4];
    let mut page = [0; 1];

    loop {
        //  Read until it hits the next potential marker
        let _ = try!(reader.read_until(0xFF, &mut search));

        try!(reader.read_exact(&mut page));
        if page[0] == 0xC0 {
            //  Correct marker, go forward 3 bytes so we're at height offset
            reader.consume(3);
            break;
        }

        reader.consume(1);
    }

    try!(reader.read_exact(&mut buffer));

    Ok(Dimensions {
        width:  ((buffer[3] as usize) | ((buffer[2] as usize) << 8)),
        height: ((buffer[1] as usize) | ((buffer[0] as usize) << 8))
    })
}

fn get_png_dimensions<R: BufRead>(reader: &mut R, offset: usize) -> ImageResult<Dimensions> {
    let mut buffer = [0; 8];
    reader.consume(16 - offset);
    try!(reader.read_exact(&mut buffer));

    Ok(Dimensions {
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

fn get_webp_vp8x_dimensions<R: BufRead>(reader: &mut R, offset: usize) -> ImageResult<Dimensions> {
    let mut buffer = [0; 6];
    reader.consume(0x18 - offset);
    try!(reader.read_exact(&mut buffer));

    Ok(Dimensions {
        width:  ((buffer[0] as usize) |
                ((buffer[1] as usize) << 8) |
                ((buffer[2] as usize) << 16)) + 1,

        height: ((buffer[3] as usize) |
                ((buffer[4] as usize) << 8) |
                ((buffer[5] as usize) << 16)) + 1
    })
}

fn get_webp_vp8_dimensions<R: BufRead>(reader: &mut R, offset: usize) -> ImageResult<Dimensions> {
    let mut buffer = [0; 6];
    reader.consume(0x1A - offset);
    try!(reader.read_exact(&mut buffer));

    Ok(Dimensions {
        width:  ((buffer[0] as usize) |
                ((buffer[1] as usize) << 8)),

        height: ((buffer[2] as usize) |
                ((buffer[3] as usize) << 8))
    })
}