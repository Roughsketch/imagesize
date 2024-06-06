use crate::{
    util::{read_u16, Endian},
    ImageResult, ImageSize,
};
use std::io::{BufRead, Seek, SeekFrom};

pub fn size<R: BufRead + Seek>(reader: &mut R) -> ImageResult<ImageSize> {
    reader.seek(SeekFrom::Start(20))?;

    Ok(ImageSize {
        width: read_u16(reader, &Endian::Big)? as usize,
        height: read_u16(reader, &Endian::Big)? as usize,
    })
}

pub fn matches(header: &[u8]) -> bool {
    header.len() >= 12 && &header[0..4] == b"FORM" && &header[8..12] == b"ILBM"
}
