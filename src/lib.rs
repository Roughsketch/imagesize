use std::error::Error;
use std::fmt;
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Cursor, Read, Seek, SeekFrom};

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
    Jxl,
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
        } else if (header.len() >= 2 && &header[0..2] == b"\xFF\x0A") ||
            (header.len() >= 12 && &header[0..12] == b"\x00\x00\x00\x0CJXL \x0D\x0A\x87\x0A") {
            return Ok(ImageType::Jxl);
        } else {
            return Err(ImageError::NotSupported);
        }
    }

    Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "Not enough data").into())
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

/// Calls the correct image size method based on the image type
///
/// # Arguments
/// * `reader` - A reader for the data
/// * `header` - The header of the file
fn dispatch_header<R: BufRead + Seek>(reader: &mut R, header: &[u8]) -> ImageResult<ImageSize> {
    match image_type(&header)? {
        ImageType::Bmp => bmp_size(reader),
        ImageType::Gif => gif_size(header),
        ImageType::Heif => heif_size(reader),
        ImageType::Jpeg => jpeg_size(reader),
        ImageType::Jxl => jxl_size(reader),
        ImageType::Png => png_size(reader),
        ImageType::Psd => psd_size(reader),
        ImageType::Tiff => tiff_size(reader),
        ImageType::Webp => webp_size(reader),
    }
}

fn bmp_size<R: BufRead + Seek>(reader: &mut R) -> ImageResult<ImageSize> {
    reader.seek(SeekFrom::Start(0x12))?;

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

fn heif_size<R: BufRead + Seek>(reader: &mut R) -> ImageResult<ImageSize> {
    reader.seek(SeekFrom::Start(0))?;
    //  Read the ftyp header size
    let ftyp_size = read_u32(reader, &Endian::Big)?;

    //  Jump to the first actual box offset
    reader.seek(SeekFrom::Start(ftyp_size.into()))?;

    //  Skip to meta tag which contains all the metadata
    skip_to_tag(reader, b"meta")?;
    read_u32(reader, &Endian::Big)?;    //  Meta has a junk value after it
    skip_to_tag(reader, b"iprp")?;      //  Find iprp tag

    let mut ipco_size = skip_to_tag(reader, b"ipco")? as usize;      //  Find ipco tag

    //  Keep track of the max size of ipco tag
    let mut max_width = 0usize;
    let mut max_height = 0usize;
    let mut found_ispe = false;
    let mut rotation = 0u8;

    while let Ok((tag, size)) = next_tag(reader) {
        //  Size of tag length + tag cannot be under 8 (4 bytes each)
        if size < 8 {
            return Err(ImageError::CorruptedImage);
        }

        //  ispe tag has a junk value followed by width and height as u32
        if tag == "ispe" {
            found_ispe = true;
            read_u32(reader, &Endian::Big)?;    //  Discard junk value
            let width = read_u32(reader, &Endian::Big)? as usize;
            let height = read_u32(reader, &Endian::Big)? as usize;

            //  Assign new largest size by area
            if width * height > max_width * max_height {
                max_width = width;
                max_height = height;
            }
        } else if tag == "irot" {
            //  irot is 9 bytes total: size, tag, 1 byte for rotation (0-3)
            rotation = read_u8(reader)?;
        } else if size >= ipco_size {
            //  If we've gone past the ipco boundary, then break
            break;
        } else {
            //  If we're still inside ipco, consume all bytes for
            //  the current tag, minus the bytes already read in `next_tag`
            ipco_size -= size;
            reader.seek(SeekFrom::Current(size as i64 - 8))?;
        }
    }

    //  If no ispe found, then we have no actual dimension data to use
    if !found_ispe {
        return Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "Not enough data").into());
    }

    //  Rotation can only be 0-3. 1 and 3 are 90 and 270 degrees respectively (anti-clockwise)
    //  If we have 90 or 270 rotation, flip width and height
    if rotation == 1 || rotation == 3 {
        std::mem::swap(&mut max_width, &mut max_height);
    }

    Ok(ImageSize {
        width: max_width,
        height: max_height
    })
}

/// Returns the amount of bytes left to read from limit
fn next_tag<R: BufRead + Seek>(reader: &mut R) -> ImageResult<(String, usize)> {
    let mut tag_buf = [0; 4];
    let size = read_u32(reader, &Endian::Big)? as usize;
    reader.read_exact(&mut tag_buf)?;

    Ok((String::from_utf8_lossy(&tag_buf).into_owned(), size))
}

fn skip_to_tag<R: BufRead + Seek>(reader: &mut R, tag: &[u8]) -> ImageResult<u32> {
    let mut tag_buf = [0; 4];

    loop {
        let size = read_u32(reader, &Endian::Big)?;
        reader.read_exact(&mut tag_buf)?;

        if tag_buf == tag {
            return Ok(size);
        }

        if size <= 8 {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Invalid heif box size: {}", size)).into());
        }

        reader.seek(SeekFrom::Current(size as i64 - 8))?;
    }
}

fn jpeg_size<R: BufRead + Seek>(reader: &mut R) -> ImageResult<ImageSize> {
    let mut marker = [0; 2];
    let mut depth = 0i32;

    //  Go to the first tag after FF D8
    reader.seek(SeekFrom::Start(2))?;

    loop {
        //  Read current marker (FF XX)
        reader.read_exact(&mut marker)?;

        if marker[0] != 0xFF {
            //  Did not read a marker. Assume image is corrupt.
            return Err(ImageError::CorruptedImage);
        }

        let page = marker[1];

        //  Check for valid SOFn markers. C4, C8, and CC aren't dimension markers.
        if  (page >= 0xC0 && page <= 0xC3) || (page >= 0xC5 && page <= 0xC7) ||
            (page >= 0xC9 && page <= 0xCB) || (page >= 0xCD && page <= 0xCF) {
            //  Only get outside image size
            if depth == 0 {
                //  Correct marker, go forward 3 bytes so we're at height offset
                reader.seek(SeekFrom::Current(3))?;
                break;
            }
        } else if page == 0xD8 {
            depth += 1;
        } else if page == 0xD9 {
            depth -= 1;
            if depth < 0 {
                return Err(ImageError::CorruptedImage);
            }
        }

        //  Read the marker length and skip over it entirely
        let page_size = read_u16(reader, &Endian::Big)? as i64;
        reader.seek(SeekFrom::Current(page_size - 2))?;
    }

    Ok(ImageSize {
        height: read_u16(reader, &Endian::Big)? as usize,
        width: read_u16(reader, &Endian::Big)? as usize,
    })
}

fn jxl_size<R: BufRead + Seek>(reader: &mut R) -> ImageResult<ImageSize> {
    let mut file_header = [0; 16]; // The size is variable, but doesn't exceed 16 bytes
    let mut header_size = 0;

    reader.seek(SeekFrom::Start(0))?;
    reader.read_exact(&mut file_header[..2])?;

    if &file_header[..2] == b"\xFF\x0A" {
        // Raw data: Read header directly
        header_size = reader.read(&mut file_header[2..])? + 2;
    } else {
        // Container format: Read from a single jxlc box or multiple jxlp boxes
        reader.seek(SeekFrom::Start(12))?;

        loop {
            let (box_type, box_size) = next_tag(reader)?;
            let box_start = reader.stream_position()? - 8;

            // If box_size is 1, the real size is stored in the first 8 bytes of content.
            // If box_size is 0, the box ends at EOF.

            let box_size = match box_size {
                1 => {
                    let mut box_size = [0; 8];
                    reader.read_exact(&mut box_size)?;
                    u64::from_be_bytes(box_size)
                },
                _ => box_size as u64,
            };

            let box_end = box_start.checked_add(box_size).ok_or(ImageError::CorruptedImage)?;
            let box_header_size = reader.stream_position()? - box_start;

            if box_size < box_header_size && box_size != 0 {
                return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Invalid size for {} box: {}", box_type, box_size)).into());
            }

            // The jxlc box must contain the complete codestream

            if box_type == "jxlc" {
                let read_end = match box_size {
                    0 => 16,
                    _ => std::cmp::min(box_size - box_header_size, 16) as usize,
                };

                header_size = reader.read(&mut file_header[..read_end])?;
                break;
            }

            // Or it could be stored as part of multiple jxlp boxes

            if box_type == "jxlp" {
                let mut jxlp_index = [0; 4];
                reader.read_exact(&mut jxlp_index)?;

                if box_size != 0 && box_size - box_header_size < 4 {
                    return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Invalid size for jxlp box: {}", box_size)).into());
                }

                let read_end = match box_size {
                    0 => 16 - header_size,
                    _ => std::cmp::min(box_size - box_header_size - 4, 16 - header_size as u64) as usize + header_size,
                };

                if read_end < header_size {
                    return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Invalid read end: {}", read_end)).into());
                }

                header_size += reader.read(&mut file_header[header_size..read_end])?;

                // If jxlp_index has the high bit set to 1, this is the final jxlp box

                if header_size == 16 || (jxlp_index[0] & 0x80) != 0 {
                    break;
                }
            }

            if box_size == 0 {
                break;
            }

            reader.seek(SeekFrom::Start(box_end))?;
        }
    }

    if header_size < 2 {
        return Err(ImageError::CorruptedImage);
    }

    if &file_header[0..2] != b"\xFF\x0A" {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid JXL signature").into());
    }

    // Parse the header data

    let file_header = u128::from_le_bytes(file_header);
    let header_size = 8*header_size;

    let is_small = read_bits(file_header, 1, 16, header_size)? != 0;

    // Extract image height:
    //     For small images, the height is stored in the next 5 bits
    //     For non-small images, the next two bits are used to determine the number of bits to read

    let height_selector = read_bits(file_header, 2, 17, header_size)?;

    let (height_bits, height_offset, height_shift) = match (is_small, height_selector) {
        (true, _) => (5, 17, 3),
        (false, 0) => (9, 19, 0),
        (false, 1) => (13, 19, 0),
        (false, 2) => (18, 19, 0),
        (false, 3) => (30, 19, 0),
        (false, _) => (0, 0, 0),
    };

    let height = (read_bits(file_header, height_bits, height_offset, header_size)? + 1) << height_shift;

    // Extract image width:
    //     If ratio is 0, use the same logic as before
    //     Otherwise, the width is calculated using a predefined aspect ratio

    let ratio = read_bits(file_header, 3, height_bits + height_offset, header_size)?;
    let width_selector = read_bits(file_header, 2, height_bits + height_offset + 3, 128)?;

    let (width_bits, width_offset, width_shift) = match (is_small, width_selector) {
        (true, _) => (5, 25, 3),
        (false, 0) => (9, height_bits + height_offset + 5, 0),
        (false, 1) => (13, height_bits + height_offset + 5, 0),
        (false, 2) => (18, height_bits + height_offset + 5, 0),
        (false, 3) => (30, height_bits + height_offset + 5, 0),
        (false, _) => (0, 0, 0),
    };

    let width = match ratio {
        1 => height,             // 1:1
        2 => (height / 10) * 12, // 12:10
        3 => (height / 3) * 4,   // 4:3
        4 => (height / 2) * 3,   // 3:2
        5 => (height / 9) * 16,  // 16:9
        6 => (height / 4) * 5,   // 5:4
        7 => height * 2,         // 2:1
        _ => (read_bits(file_header, width_bits, width_offset, header_size)? + 1) << width_shift,
    };

    // Extract orientation:
    //     This value overrides the orientation in EXIF metadata

    let metadata_offset = match ratio {
        0 => width_bits + width_offset,
        _ => height_bits + height_offset + 3,
    };

    let all_default = read_bits(file_header, 1, metadata_offset, header_size)? != 0;

    let orientation = match all_default {
        true => 0,
        false => {
            let extra_fields = read_bits(file_header, 1, metadata_offset + 1, header_size)? != 0;

            match extra_fields {
                false => 0,
                true => read_bits(file_header, 3, metadata_offset + 2, header_size)?,
            }
        },
    };

    if orientation < 4 {
        Ok(ImageSize { width, height })
    } else {
        Ok(ImageSize { width: height, height: width })
    }
}

fn png_size<R: BufRead + Seek>(reader: &mut R) -> ImageResult<ImageSize> {
    reader.seek(SeekFrom::Start(0x10))?;

    Ok(ImageSize {
        width: read_u32(reader, &Endian::Big)? as usize,
        height: read_u32(reader, &Endian::Big)? as usize,
    })
}

fn psd_size<R: BufRead + Seek>(reader: &mut R) -> ImageResult<ImageSize> {
    reader.seek(SeekFrom::Start(0x0E))?;

    Ok(ImageSize {
        height: read_u32(reader, &Endian::Big)? as usize,
        width: read_u32(reader, &Endian::Big)? as usize,
    })
}

fn tiff_size<R: BufRead + Seek>(reader: &mut R) -> ImageResult<ImageSize> {
    reader.seek(SeekFrom::Start(0))?;

    let mut endian_marker = [0; 2];
    reader.read_exact(&mut endian_marker)?;

    //  Get the endianness which determines how we read the input
    let endianness = if &endian_marker[0..2] == b"II" {
        Endian::Little
    } else if &endian_marker[0..2] == b"MM" {
        Endian::Big
    } else {
        //  Shouldn't get here by normal means, but handle invalid header anyway
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid TIFF header").into())
    };

    //  Read the IFD offset from the header
    reader.seek(SeekFrom::Start(4))?;
    let ifd_offset = read_u32(reader, &endianness)?;

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
            reader.seek(SeekFrom::Current(6))?;
            width = Some(read_u32(reader, &endianness)?);
        }
        else if tag == 0x101 {
            //  Skip the type/count since we just need the value
            reader.seek(SeekFrom::Current(6))?;
            height = Some(read_u32(reader, &endianness)?);
        } else {
            //  Not a tag we care about. Just figure out how much data to skip.
            let kind = read_u16(reader, &endianness)?;
            let count = read_u32(reader, &endianness)? as i64;

            let skip_count = match kind {
                1 | 2 => count, //  Byte | ASCII both skip count bytes
                3 => count * 2, //  Shorts are 2 bytes each
                4 => count * 4, //  Longs are 4 bytes each
                5 => count * 8, //  Rationals consist of two Longs, so 8 bytes each
                //  Anything else is invalid
                _ => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid IDF type").into()),
            };

            //  Skip the amount determined
            reader.seek(SeekFrom::Current(skip_count))?;
        }

        //  If we've read both values we need, return the data
        if let (Some(width), Some(height)) = (width, height) {
            return Ok(ImageSize {
                width: width as usize,
                height: height as usize,
            });
        }
    }

    //  If no width/height pair was found return invalid data
    Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "No dimensions in IFD tags").into())
}

fn webp_size<R: BufRead + Seek>(reader: &mut R) -> ImageResult<ImageSize> {
    let mut buffer = [0; 4];
    reader.read_exact(&mut buffer)?;

    if buffer[3] == b' ' {
        webp_vp8_size(reader)
    } else if buffer[3] == b'L' {
        webp_vp8l_size(reader)
    } else if buffer[3] == b'X' {
        webp_vp8x_size(reader)
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid VP8 Tag").into())
    }
}

fn webp_vp8x_size<R: BufRead + Seek>(reader: &mut R) -> ImageResult<ImageSize> {
    reader.seek(SeekFrom::Start(0x18))?;

    Ok(ImageSize {
        width: read_u24(reader, &Endian::Little)? as usize + 1,
        height: read_u24(reader, &Endian::Little)? as usize + 1,
    })
}

fn webp_vp8l_size<R: BufRead + Seek>(reader: &mut R) -> ImageResult<ImageSize> {
    reader.seek(SeekFrom::Start(0x15))?;

    let dims = read_u32(reader, &Endian::Little)?;

    Ok(ImageSize {
        width: (dims & 0x3FFF) as usize + 1,
        height: ((dims >> 14) & 0x3FFF) as usize + 1,
    })
}

fn webp_vp8_size<R: BufRead + Seek>(reader: &mut R) -> ImageResult<ImageSize> {
    reader.seek(SeekFrom::Start(0x1A))?;

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
        Endian::Big => Ok(((buf[0] as u32) << 16) | ((buf[1] as u32) << 8) | (buf[2] as u32)),
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

fn read_u8<R: BufRead + Seek>(reader: &mut R) -> ImageResult<u8> {
    let mut buf = [0; 1];
    reader.read_exact(&mut buf)?;
    Ok(buf[0])
}

fn read_bits(source: u128, num_bits: usize, offset: usize, size: usize) -> ImageResult<usize> {
    if offset + num_bits < size {
        Ok((source >> offset) as usize & ((1 << num_bits) - 1))
    } else {
        Err(ImageError::CorruptedImage)
    }
}
