use std::io::{self, BufRead, Read, Seek, SeekFrom};

use crate::{ImageResult, ImageSize};

fn read_null_terminated_string<R: Read>(reader: &mut R) -> io::Result<String> {
    let mut bytes = Vec::new();
    loop {
        let mut byte = [0; 1];
        reader.read_exact(&mut byte)?;
        if byte[0] == 0 {
            break;
        }
        bytes.push(byte[0]);
    }
    String::from_utf8(bytes).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

pub fn size<R: BufRead + Seek>(reader: &mut R) -> ImageResult<ImageSize> {
    reader.seek(SeekFrom::Start(0))?;

    // Read and check the magic number (4 bytes)
    let mut magic_number = [0; 4];
    reader.read_exact(&mut magic_number)?;
    if magic_number != [0x76, 0x2f, 0x31, 0x01] {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid EXR magic number").into());
    }

    // Skip the version field (4 bytes)
    reader.seek(SeekFrom::Current(4))?;

    // Read header attributes until we find the dataWindow attribute
    loop {
        let attr_name = read_null_terminated_string(reader)?;
        if attr_name.is_empty() {
            break; // End of the header
        }

        let attr_type = read_null_terminated_string(reader)?;

        let mut attr_size_buf = [0; 4];
        reader.read_exact(&mut attr_size_buf)?;
        let attr_size = u32::from_le_bytes(attr_size_buf);

        if attr_name == "dataWindow" && attr_type == "box2i" {
            // Read the data window values
            let mut data_window_buf = vec![0; attr_size as usize];
            reader.read_exact(&mut data_window_buf)?;

            let x_min = i32::from_le_bytes([
                data_window_buf[0],
                data_window_buf[1],
                data_window_buf[2],
                data_window_buf[3],
            ]);
            let y_min = i32::from_le_bytes([
                data_window_buf[4],
                data_window_buf[5],
                data_window_buf[6],
                data_window_buf[7],
            ]);
            let x_max = i32::from_le_bytes([
                data_window_buf[8],
                data_window_buf[9],
                data_window_buf[10],
                data_window_buf[11],
            ]);
            let y_max = i32::from_le_bytes([
                data_window_buf[12],
                data_window_buf[13],
                data_window_buf[14],
                data_window_buf[15],
            ]);

            let width = (x_max - x_min + 1) as usize;
            let height = (y_max - y_min + 1) as usize;

            return Ok(ImageSize { width, height });
        } else {
            // Skip the attribute value
            reader.seek(SeekFrom::Current(attr_size as i64))?;
        }
    }

    Err(io::Error::new(io::ErrorKind::InvalidData, "Data window not found").into())
}

pub fn matches(header: &[u8]) -> bool {
    let exr_magic_number = [0x76, 0x2f, 0x31, 0x01];
    header.starts_with(&exr_magic_number)
}
