pub mod aesprite;
pub mod bmp;
pub mod gif;
pub mod heif;
pub mod ico;
pub mod jpeg;
pub mod jxl;
pub mod png;
pub mod psd;
pub mod tiff;
pub mod webp;

use crate::{ImageError, ImageResult, ImageType};

pub fn image_type(header: &[u8]) -> ImageResult<ImageType> {
    // Currently there are no formats where 1 byte is enough to determine format
    if header.len() < 2 {
        return Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "Not enough data").into());
    }

    // This is vaguely organized in what I assume are the most commonly used formats.
    // I don't know how much this matters for actual execution time.
    if jpeg::matches(header) {
        return Ok(ImageType::Jpeg);
    } 
    
    if png::matches(header) {
        return Ok(ImageType::Png);
    } 
    
    if gif::matches(header) {
        return Ok(ImageType::Gif);
    } 
    
    if tiff::matches(header) {
        return Ok(ImageType::Tiff);
    } 
    
    if webp::matches(header) {
        return Ok(ImageType::Webp);
    } 
    
    if heif::matches(header) {
        return Ok(ImageType::Heif);
    } 
    
    if jxl::matches(header) {
        return Ok(ImageType::Jxl);
    } 
    
    if bmp::matches(header) {
        return Ok(ImageType::Bmp);
    }
    
    if psd::matches(header) {
        return Ok(ImageType::Psd);
    } 
    
    if ico::matches(header) {
        return Ok(ImageType::Ico);
    } 
    
    if aesprite::matches(header) {
        return Ok(ImageType::Aseprite);
    } 
    
    Err(ImageError::NotSupported)
}