use std::error::Error;
use std::fmt;
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Cursor, Read, Seek, SeekFrom};

#[cfg(test)] 
mod test;

/// An Error type used in failure cases.
#[derive(Debug)]
pub enum ImageError {
    /// Used when the given data is not a supported format.
    NotSupported,
    /// Used when the image has an invalid format.
    CorruptedImage,
    /// Used when an IoError occurs when trying to read the given data.
    IoError(std::io::Error),
}

impl Error for ImageError {}

impl fmt::Display for ImageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::ImageError::*;
        match self {
            NotSupported => f.write_str("Could not decode image"),
            CorruptedImage => f.write_str("Hit end of file before finding size"),
            IoError(error) => error.fmt(f),
        }
    }
}

impl From<std::io::Error> for ImageError {
    fn from(err: std::io::Error) -> ImageError {
        ImageError::IoError(err)
    }
}

pub type ImageResult<T> = Result<T, ImageError>;

/// Types of image formats that this crate can identify.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ImageType {
    Bmp,
    Gif,
    Heif,
    Jpeg,
    Png,
    Psd,
    Tiff,
    Webp,
}

/// Holds the size information of an image.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ImageSize {
    /// Width of an image in pixels.
    pub width: usize,
    /// Height of an image in pixels.
    pub height: usize,
}

/// Used for TIFF decoding
enum Endian {
    Little,
    Big,
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
        } else if header.len() >= 3 && &header[0..3] == b"\xFF\xD8\xFF" {
            return Ok(ImageType::Jpeg);
        } else if header.len() >= 4 && &header[0..4] == b"\x89PNG" {
            return Ok(ImageType::Png);
        } else if header.len() >= 4 && &header[0..4] == b"GIF8" {
            return Ok(ImageType::Gif);
        } else if header.len() >= 4 && (&header[0..4] == b"II\x2A\x00" || &header[0..4] == b"MM\x00\x2A") {
            return Ok(ImageType::Tiff);
        } else if header.len() >= 4 && &header[0..4] == b"8BPS" {
            return Ok(ImageType::Psd);
        } else if header.len() >= 8 &&
            &header[4..8] == b"ftyp" {
            return Ok(ImageType::Heif);
        } else if header.len() >= 12 && 
            &header[0..4] == b"RIFF" &&
            &header[8..12] == b"WEBP"{
            return Ok(ImageType::Webp);
        } else {
            return Err(ImageError::NotSupported);
        }
    }

    Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "Not enough data").into())
}

/// Calls the correct image size method based on the image type
///
/// # Arguments
/// * `reader` - A reader for the data
/// * `header` - The header of the file
fn dispatch_header<R: BufRead + Seek>(reader: &mut R, header: &[u8]) -> ImageResult<ImageSize> {
    match image_type(&header)? {
        ImageType::Bmp => bmp_size(reader, header.len()),
        ImageType::Gif => gif_size(header),
        ImageType::Heif => heif_size(reader, header),
        ImageType::Jpeg => jpeg_size(reader, header.len()),
        ImageType::Png => png_size(reader, header.len()),
        ImageType::Psd => psd_size(reader, header.len()),
        ImageType::Tiff => tiff_size(reader, header),
        ImageType::Webp => webp_size(reader, header),
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
/// * The header isn't recognized as a supported image format
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

    let mut header = [0; 12];
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
/// * The header isn't recognized as a supported image format
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
    let mut reader = Cursor::new(&data[..]);

    let mut header = [0; 12];
    reader.read_exact(&mut header)?;

    dispatch_header(&mut reader, &header)
}

fn bmp_size<R: BufRead + Seek>(reader: &mut R, offset: usize) -> ImageResult<ImageSize> {
    reader.consume(0x12 - offset);

    Ok(ImageSize {
        width: read_u32(reader, &Endian::Little)? as usize,
        height: read_u32(reader, &Endian::Little)? as usize,
    })
}

fn gif_size(header: &[u8]) -> ImageResult<ImageSize> {
    Ok(ImageSize {
        width:  ((header[6] as usize) | ((header[7] as usize) << 8)),
        height: ((header[8] as usize) | ((header[9] as usize) << 8))
    })
}

fn heif_size<R: BufRead + Seek>(reader: &mut R, header: &[u8]) -> ImageResult<ImageSize> {
    //  Read the ftyp header size
    let ftyp_size = read_u32(&mut Cursor::new(&header[0..]), &Endian::Big)?;

    //  Jump to the first actual box offset
    reader.seek(SeekFrom::Start(ftyp_size.into()))?;

    //  Skip to meta tag which contains all the metadata
    skip_to_tag(reader, b"meta")?;
    read_u32(reader, &Endian::Big)?;    //  Meta has a junk value after it
    skip_to_tag(reader, b"iprp")?;      //  Find iprp tag
    skip_to_tag(reader, b"ipco")?;      //  Find ipco tag
    skip_to_tag(reader, b"ispe")?;      //  Find ispe tag which has spatial dimensions

    read_u32(reader, &Endian::Big)?;    //  Discard junk value
    let width = read_u32(reader, &Endian::Big)? as usize;
    let height = read_u32(reader, &Endian::Big)? as usize;
    return Ok(ImageSize { width, height });
}

fn skip_to_tag<R: BufRead + Seek>(reader: &mut R, tag: &[u8]) -> ImageResult<()> {
    let mut tag_buf = [0; 4];

    loop {
        let size = read_u32(reader, &Endian::Big)?;
        reader.read_exact(&mut tag_buf)?;

        if tag_buf == tag {
            return Ok(());
        }

        if size <= 8 {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Invalid heif box size: {}", size)).into());
        }

        reader.consume((size - 8) as usize);
    }
}

fn jpeg_size<R: BufRead + Seek>(reader: &mut R, _offset: usize) -> ImageResult<ImageSize> {
    let mut search = Vec::new();
    let mut page = [0; 1];
    let mut depth = 0i32;

    loop {
        //  Read until it hits the next potential marker
        let read_bytes = reader.read_until(0xFF, &mut search)?;

        loop {
            reader.read_exact(&mut page)?;

            if page[0] != 0xFF {
                break;
            }
        }
        
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
                return Err(ImageError::CorruptedImage);
            }
        }
    }

    Ok(ImageSize {
        height: read_u16(reader, &Endian::Big)? as usize,
        width: read_u16(reader, &Endian::Big)? as usize,
    })
}

fn png_size<R: BufRead + Seek>(reader: &mut R, offset: usize) -> ImageResult<ImageSize> {
    reader.consume(16 - offset);

    Ok(ImageSize {
        width: read_u32(reader, &Endian::Big)? as usize,
        height: read_u32(reader, &Endian::Big)? as usize,
    })
}

fn psd_size<R: BufRead + Seek>(reader: &mut R, offset: usize) -> ImageResult<ImageSize> {
    reader.consume(14 - offset);

    Ok(ImageSize {
        height: read_u32(reader, &Endian::Big)? as usize,
        width: read_u32(reader, &Endian::Big)? as usize,
    })
}

fn tiff_size<R: BufRead + Seek>(reader: &mut R, header: &[u8]) -> ImageResult<ImageSize> {
    //  Get the endianness which determines how we read the input
    let endianness = if &header[0..2] == b"II" {
        Endian::Little
    } else if &header[0..2] == b"MM" {
        Endian::Big
    } else {
        //  Shouldn't get here by normal means, but handle invalid header anyway
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid TIFF header").into())
    };

    //  Read the IFD offset from the header
    let ifd_offset = read_u32(&mut Cursor::new(&header[4..]), &endianness)?;

    //  IFD offset cannot be 0
    if ifd_offset == 0 {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid IFD offset").into())
    }

    //  Jump to the IFD offset
    reader.seek(SeekFrom::Start(ifd_offset.into()))?;

    //  Read how many IFD records there are
    let ifd_count = read_u16(reader, &endianness)?;
    let mut width = None;
    let mut height = None;

    for _ifd in 0..ifd_count {
        let tag = read_u16(reader, &endianness)?;

        //  Tag 0x100 is the image width, 0x101 is image height
        if tag == 0x100 {
            //  Skip the type/count since we just need the value
            reader.consume(6);
            width = Some(read_u32(reader, &endianness)?);
        }
        else if tag == 0x101 {
            //  Skip the type/count since we just need the value
            reader.consume(6);
            height = Some(read_u32(reader, &endianness)?);
        } else {
            //  Not a tag we care about. Just figure out how much data to skip.
            let kind = read_u16(reader, &endianness)?;
            let count = read_u32(reader, &endianness)?;

            let skip_count = match kind {
                1 | 2 => count, //  Byte | ASCII both skip count bytes
                3 => count * 2, //  Shorts are 2 bytes each
                4 => count * 4, //  Longs are 4 bytes each
                5 => count * 8, //  Rationals consist of two Longs, so 8 bytes each
                //  Anything else is invalid
                _ => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid IDF type").into()),
            };

            //  Skip the amount determined
            reader.consume(skip_count as usize);
        }

        //  If we've read both values we need, return the data
        if width.is_some() && height.is_some() {
            return Ok(ImageSize {
                width: width.unwrap() as usize,
                height: height.unwrap() as usize,
            });
        }
    }

    //  If no width/height pair was found return invalid data
    return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "No dimensions in IFD tags").into())
}

fn webp_size<R: BufRead + Seek>(reader: &mut R, header: &[u8]) -> ImageResult<ImageSize> {
    let mut buffer = [0; 4];
    reader.read_exact(&mut buffer)?;

    if buffer[3] == b' ' {
        webp_vp8_size(reader, header.len() + buffer.len())
    } else {
        webp_vp8x_size(reader, header.len() + buffer.len())
    }
}

fn webp_vp8x_size<R: BufRead + Seek>(reader: &mut R, offset: usize) -> ImageResult<ImageSize> {
    reader.consume(0x18 - offset);

    Ok(ImageSize {
        width: read_u24(reader, &Endian::Little)? as usize + 1,
        height: read_u24(reader, &Endian::Little)? as usize + 1,
    })
}

fn webp_vp8_size<R: BufRead + Seek>(reader: &mut R, offset: usize) -> ImageResult<ImageSize> {
    reader.consume(0x1A - offset);

    Ok(ImageSize {
        width: read_u16(reader, &Endian::Little)? as usize,
        height: read_u16(reader, &Endian::Little)? as usize,
    })
}

fn read_u32<R: BufRead + Seek>(reader: &mut R, endianness: &Endian) -> ImageResult<u32> {
    let mut buf = [0; 4];
    reader.read_exact(&mut buf)?;

    match endianness {
        Endian::Little => Ok(((buf[3] as u32) << 24) | ((buf[2] as u32) << 16) | ((buf[1] as u32) << 8) | (buf[0] as u32)),
        Endian::Big => Ok(((buf[0] as u32) << 24) | ((buf[1] as u32) << 16) | ((buf[2] as u32) << 8) | (buf[3] as u32)),
    }
}

fn read_u24<R: BufRead + Seek>(reader: &mut R, endianness: &Endian) -> ImageResult<u32> {
    let mut buf = [0; 3];
    reader.read_exact(&mut buf)?;

    match endianness {
        Endian::Little => Ok(((buf[2] as u32) << 16) | ((buf[1] as u32) << 8) | (buf[0] as u32)),
        Endian::Big => Ok(((buf[0] as u32) << 16) | ((buf[1] as u32) << 8) | ((buf[2] as u32))),
    }
}

fn read_u16<R: BufRead + Seek>(reader: &mut R, endianness: &Endian) -> ImageResult<u16> {
    let mut buf = [0; 2];
    reader.read_exact(&mut buf)?;

    match endianness {
        Endian::Little => Ok(((buf[1] as u16) << 8) | (buf[0] as u16)),
        Endian::Big => Ok(((buf[0] as u16) << 8) | (buf[1] as u16)),
    }
}