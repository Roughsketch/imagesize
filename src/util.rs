use crate::{ImageError, ImageResult};
use std::io::{self, BufRead, Read, Seek};

/// Used for TIFF decoding
pub enum Endian {
    Little,
    Big,
}

pub fn read_i32<R: BufRead + Seek>(reader: &mut R, endianness: &Endian) -> ImageResult<i32> {
    let mut attr_size_buf = [0; 4];
    reader.read_exact(&mut attr_size_buf)?;
    match endianness {
        Endian::Little => Ok(i32::from_le_bytes(attr_size_buf)),
        Endian::Big => Ok(i32::from_be_bytes(attr_size_buf)),
    }
}

pub fn read_u32<R: BufRead + Seek>(reader: &mut R, endianness: &Endian) -> ImageResult<u32> {
    let mut buf = [0; 4];
    reader.read_exact(&mut buf)?;

    match endianness {
        Endian::Little => Ok(((buf[3] as u32) << 24)
            | ((buf[2] as u32) << 16)
            | ((buf[1] as u32) << 8)
            | (buf[0] as u32)),
        Endian::Big => Ok(((buf[0] as u32) << 24)
            | ((buf[1] as u32) << 16)
            | ((buf[2] as u32) << 8)
            | (buf[3] as u32)),
    }
}

pub fn read_u24<R: BufRead + Seek>(reader: &mut R, endianness: &Endian) -> ImageResult<u32> {
    let mut buf = [0; 3];
    reader.read_exact(&mut buf)?;

    match endianness {
        Endian::Little => Ok(((buf[2] as u32) << 16) | ((buf[1] as u32) << 8) | (buf[0] as u32)),
        Endian::Big => Ok(((buf[0] as u32) << 16) | ((buf[1] as u32) << 8) | (buf[2] as u32)),
    }
}

pub fn read_u16<R: BufRead + Seek>(reader: &mut R, endianness: &Endian) -> ImageResult<u16> {
    let mut buf = [0; 2];
    reader.read_exact(&mut buf)?;

    match endianness {
        Endian::Little => Ok(((buf[1] as u16) << 8) | (buf[0] as u16)),
        Endian::Big => Ok(((buf[0] as u16) << 8) | (buf[1] as u16)),
    }
}

pub fn read_u8<R: BufRead + Seek>(reader: &mut R) -> ImageResult<u8> {
    let mut buf = [0; 1];
    reader.read_exact(&mut buf)?;
    Ok(buf[0])
}

pub fn read_bits(source: u128, num_bits: usize, offset: usize, size: usize) -> ImageResult<usize> {
    if offset + num_bits < size {
        Ok((source >> offset) as usize & ((1 << num_bits) - 1))
    } else {
        Err(ImageError::CorruptedImage)
    }
}

/// Assumes tags are in format of 4 char string followed by big endian size for tag
pub fn read_tag<R: BufRead + Seek>(reader: &mut R) -> ImageResult<(String, usize)> {
    let mut tag_buf = [0; 4];
    let size = read_u32(reader, &Endian::Big)? as usize;
    reader.read_exact(&mut tag_buf)?;

    Ok((String::from_utf8_lossy(&tag_buf).into_owned(), size))
}

pub fn read_null_terminated_string<R: BufRead>(reader: &mut R) -> io::Result<String> {
    let mut bytes = Vec::new();

    reader.read_until(0, &mut bytes)?;
    bytes.pop();

    String::from_utf8(bytes).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}
