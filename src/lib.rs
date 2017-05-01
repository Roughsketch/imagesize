use std::fs::File;
use std::path::Path;
use std::io::Read;
use std::io::{BufRead, BufReader, Seek};

#[cfg(test)]
mod tests {
    use *;

    #[test]
    fn bmp_test() {
        match get_dimensions("test/test.bmp") {
            Ok(dim) => {
                assert_eq!(dim.width, 512);
                assert_eq!(dim.height, 512);
            }
            Err(why) => println!("Error getting dimensions: {:?}", why)
        }
    }

    #[test]
    fn gif_test() {
        match get_dimensions("test/test.gif") {
            Ok(dim) => {
                assert_eq!(dim.width, 100);
                assert_eq!(dim.height, 100);
            }
            Err(why) => println!("Error getting dimensions: {:?}", why)
        }
    }

    #[test]
    fn jpeg_test() {
        match get_dimensions("test/test.jpg") {
            Ok(dim) => {
                assert_eq!(dim.width, 690);
                assert_eq!(dim.height, 298);
            }
            Err(why) => println!("Error getting dimensions: {:?}", why)
        }
    }

    #[test]
    fn png_test() {
        match get_dimensions("test/test.png") {
            Ok(dim) => {
                assert_eq!(dim.width, 2000);
                assert_eq!(dim.height, 2000);
            }
            Err(why) => println!("Error getting dimensions: {:?}", why)
        }
    }

    #[test]
    fn webp_test() {
        match get_dimensions("test/test.webp") {
            Ok(dim) => {
                assert_eq!(dim.width, 716);
                assert_eq!(dim.height, 716);
            }
            Err(why) => println!("Error getting dimensions: {:?}", why)
        }
    }
}

pub enum ImageType {
    BMP,
    GIF,
    JPEG,
    PNG,
    WEBP,
}

#[derive(Debug)]
pub enum ImageError {
    NotSupported(String),
    IoError(std::io::Error),
}

impl From<std::io::Error> for ImageError {
    fn from(err: std::io::Error) -> ImageError {
        ImageError::IoError(err)
    }
}

pub type ImageResult<T> = Result<T, ImageError>;

pub struct Dimensions {
    pub width: usize,
    pub height: usize,
}

pub fn get_dimensions<P>(path: P) -> ImageResult<Dimensions> where P: AsRef<Path> {
    let file = try!(File::open(path));
    let mut reader = BufReader::new(file);

    let mut header = [0; 1];
    try!(reader.read_exact(&mut header));

    match header[0] {
        0xFF => get_jpeg_dimensions(&mut reader),
        0x89 => get_png_dimensions(&mut reader),
        b'R' => get_webp_dimensions(&mut reader),
        b'G' => get_gif_dimensions(&mut reader),
        b'B' => get_bmp_dimensions(&mut reader),
        _ => Err(ImageError::NotSupported("Could not decode image.".to_owned()))
    }
}

fn get_bmp_dimensions(reader: &mut BufReader<File>) -> ImageResult<Dimensions> {
    let mut buffer = [0; 8];
    try!(reader.seek(std::io::SeekFrom::Start(0x12)));
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

fn get_gif_dimensions(reader: &mut BufReader<File>) -> ImageResult<Dimensions> {
    let mut buffer = [0; 4];
    try!(reader.seek(std::io::SeekFrom::Start(6)));
    try!(reader.read_exact(&mut buffer));

    Ok(Dimensions {
        width:  ((buffer[0] as usize) | ((buffer[1] as usize) << 8)),
        height: ((buffer[2] as usize) | ((buffer[3] as usize) << 8))
    })
}

fn get_jpeg_dimensions(reader: &mut BufReader<File>) -> ImageResult<Dimensions> {
    let mut search = Vec::new();
    let mut buffer = [0; 4];
    let mut page = [0; 1];

    loop {
        let _ = try!(reader.read_until(0xFF, &mut search));
        try!(reader.take(1).read(&mut page));
        if page[0] == 0xC0 {
            try!(reader.seek(std::io::SeekFrom::Current(3)));
            break;
        }
    }

    try!(reader.read_exact(&mut buffer));

    Ok(Dimensions {
        width:  ((buffer[3] as usize) | ((buffer[2] as usize) << 8)),
        height: ((buffer[1] as usize) | ((buffer[0] as usize) << 8))
    })
}

fn get_png_dimensions(reader: &mut BufReader<File>) -> ImageResult<Dimensions> {
    let mut buffer = [0; 8];
    try!(reader.seek(std::io::SeekFrom::Start(16)));
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

fn get_webp_dimensions(reader: &mut BufReader<File>) -> ImageResult<Dimensions> {
    let mut buffer = [0; 6];
    try!(reader.seek(std::io::SeekFrom::Start(0x18)));
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
