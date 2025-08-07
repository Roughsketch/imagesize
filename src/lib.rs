#![allow(dead_code)]

use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader, Cursor, Seek};
use std::path::Path;

mod container;
mod formats;
mod util;

pub use container::{
    atc::AtcCompression, dds::DdsCompression, heif::Compression, pkm::PkmCompression,
    pvrtc::PvrtcCompression,
};

/// Groups related compression algorithms regardless of their container format
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CompressionFamily {
    /// Block Compression family (BC1-7, also known as DXT1-5, ATI1-2)
    BlockCompression,
    /// Ericsson Texture Compression family (ETC1, ETC2 variants)
    Etc,
    /// Ericsson Alpha Compression (EAC R11, RG11)
    Eac,
    /// PowerVR Texture Compression
    Pvrtc,
    /// Adaptive Scalable Texture Compression
    Astc,
    /// Adaptive Texture Compression (Qualcomm Adreno)
    Atc,
    /// Uncompressed formats
    Uncompressed,
}

use {
    container::heif::{self},
    formats::*,
};

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
///
/// Many container formats support multiple inner compression formats. For these formats,
/// the enum contains the inner compression type to provide more detailed information:
///
/// - `Dds(DdsCompression)` - DirectDraw Surface with various BC compression formats
/// - `Etc2(PkmCompression)` - ETC/PKM container with ETC1, ETC2, EAC variants
/// - `Eac(PkmCompression)` - EAC formats (unified with ETC2 detection)
/// - `Atc(AtcCompression)` - Adaptive Texture Compression variants
/// - `Pvrtc(PvrtcCompression)` - PowerVR texture compression with 2bpp/4bpp variants
///
/// # Helper Methods
///
/// The `ImageType` provides several helper methods to query compression information 
/// across different container formats:
///
/// - [`compression_family()`](ImageType::compression_family) - Groups related compression algorithms
/// - [`is_block_compressed()`](ImageType::is_block_compressed) - Checks for BC/DXT compression
/// - [`container_format()`](ImageType::container_format) - Returns container format name
/// - [`is_multi_compression_container()`](ImageType::is_multi_compression_container) - Checks if container supports multiple compression types
///
/// # Examples
///
/// ## Basic Format Detection
///
/// ```rust
/// use imagesize::{image_type, ImageType, PkmCompression};
///
/// // Create a PKM header for ETC2 format
/// let mut header = vec![b'P', b'K', b'M', b' ', b'2', b'0'];
/// header.extend_from_slice(&0x0001u16.to_be_bytes()); // ETC2 RGB
/// header.extend_from_slice(&[0x00, 0x40, 0x00, 0x40]); // Extended dimensions
/// header.extend_from_slice(&[0x00, 0x40, 0x00, 0x40]); // Original dimensions
///
/// match image_type(&header).unwrap() {
///     ImageType::Etc2(PkmCompression::Etc2) => println!("This is ETC2 RGB format"),
///     ImageType::Etc2(compression) => println!("This is ETC2 format: {:?}", compression),
///     other => println!("Other format: {:?}", other),
/// }
/// ```
///
/// ## Using Helper Methods for Cross-Container Queries
///
/// ```rust
/// use imagesize::{ImageType, CompressionFamily, DdsCompression, PvrtcCompression};
///
/// // Query compression families across different containers
/// let dds_bc1 = ImageType::Dds(DdsCompression::Bc1);
/// let pvr_etc2 = ImageType::Pvrtc(PvrtcCompression::Etc2Rgb);
/// let png = ImageType::Png;
///
/// // Group related compression algorithms
/// assert_eq!(dds_bc1.compression_family(), Some(CompressionFamily::BlockCompression));
/// assert_eq!(pvr_etc2.compression_family(), Some(CompressionFamily::Etc));
/// assert_eq!(png.compression_family(), None); // Simple formats don't have compression
///
/// // Check for specific compression types
/// assert!(dds_bc1.is_block_compressed());
/// assert!(!pvr_etc2.is_block_compressed());
///
/// // Identify container formats
/// assert_eq!(dds_bc1.container_format(), Some("DDS"));
/// assert_eq!(pvr_etc2.container_format(), Some("PowerVR"));
/// assert_eq!(png.container_format(), None);
///
/// // Check multi-compression support
/// assert!(dds_bc1.is_multi_compression_container()); // DDS supports BC1-7, RGBA, etc.
/// assert!(pvr_etc2.is_multi_compression_container()); // PowerVR supports PVRTC, ETC2, EAC
/// ```
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ImageType {
    /// Animated sprite image format
    /// <https://github.com/aseprite/aseprite>
    #[cfg(feature = "aesprite")]
    Aseprite,
    /// Adaptive Scalable Texture Compression
    #[cfg(feature = "astc")]
    Astc,
    /// Adaptive Texture Compression
    #[cfg(feature = "atc")]
    Atc(AtcCompression),
    /// Standard Bitmap
    #[cfg(feature = "bmp")]
    Bmp,
    /// DirectDraw Surface
    #[cfg(feature = "dds")]
    Dds(DdsCompression),
    /// Ericsson Texture Compression - Alpha Channel (now unified with ETC2)
    #[cfg(feature = "eac")]
    Eac(PkmCompression),
    /// Ericsson Texture Compression 2 (includes ETC1, ETC2 variants)
    #[cfg(feature = "etc2")]
    Etc2(PkmCompression),
    /// OpenEXR
    #[cfg(feature = "exr")]
    Exr,
    /// Farbfeld
    /// <https://tools.suckless.org/farbfeld/>
    #[cfg(feature = "farbfeld")]
    Farbfeld,
    /// Standard GIF
    #[cfg(feature = "gif")]
    Gif,
    /// Radiance HDR
    #[cfg(feature = "hdr")]
    Hdr,
    /// Image Container Format
    #[cfg(feature = "heif")]
    Heif(Compression),
    /// Icon file
    #[cfg(feature = "ico")]
    Ico,
    /// Interleaved Bitmap
    #[cfg(feature = "ilbm")]
    Ilbm,
    /// Standard JPEG
    #[cfg(feature = "jpeg")]
    Jpeg,
    /// JPEG XL
    #[cfg(feature = "jxl")]
    Jxl,
    /// Khronos Texture Container
    #[cfg(feature = "ktx2")]
    Ktx2,
    /// Standard PNG
    #[cfg(feature = "png")]
    Png,
    /// Portable Any Map
    #[cfg(feature = "pnm")]
    Pnm,
    /// PowerVR Texture Compression
    #[cfg(feature = "pvrtc")]
    Pvrtc(PvrtcCompression),
    /// Photoshop Document
    #[cfg(feature = "psd")]
    Psd,
    /// Quite OK Image Format
    /// <https://qoiformat.org/>
    #[cfg(feature = "qoi")]
    Qoi,
    /// Truevision Graphics Adapter
    #[cfg(feature = "tga")]
    Tga,
    /// Standard TIFF
    #[cfg(feature = "tiff")]
    Tiff,
    /// Valve Texture Format
    #[cfg(feature = "vtf")]
    Vtf,
    /// Standard Webp
    #[cfg(feature = "webp")]
    Webp,
}

impl ImageType {
    /// Returns the compression family for texture formats
    ///
    /// Groups related compression algorithms regardless of their container format.
    /// Returns None for simple image formats like PNG, JPEG, etc.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use imagesize::{ImageType, CompressionFamily, DdsCompression, PvrtcCompression};
    ///
    /// let dds_type = ImageType::Dds(DdsCompression::Bc1);
    /// assert_eq!(dds_type.compression_family(), Some(CompressionFamily::BlockCompression));
    ///
    /// let pvrtc_etc2_type = ImageType::Pvrtc(PvrtcCompression::Etc2Rgb);
    /// assert_eq!(pvrtc_etc2_type.compression_family(), Some(CompressionFamily::Etc));
    ///
    /// let png_type = ImageType::Png;
    /// assert_eq!(png_type.compression_family(), None);
    /// ```
    pub fn compression_family(&self) -> Option<CompressionFamily> {
        match self {
            #[cfg(feature = "dds")]
            ImageType::Dds(compression) => match compression {
                DdsCompression::Bc1 | DdsCompression::Bc2 | DdsCompression::Bc3 |
                DdsCompression::Bc4 | DdsCompression::Bc5 | DdsCompression::Bc6h |
                DdsCompression::Bc7 => Some(CompressionFamily::BlockCompression),
                DdsCompression::Rgba32 | DdsCompression::Rgb24 => Some(CompressionFamily::Uncompressed),
                DdsCompression::Unknown => None,
            },
            
            #[cfg(feature = "etc2")]
            ImageType::Etc2(compression) => match compression {
                PkmCompression::Etc1 | PkmCompression::Etc2 | PkmCompression::Etc2A1 |
                PkmCompression::Etc2A8 => Some(CompressionFamily::Etc),
                PkmCompression::EacR | PkmCompression::EacRg | PkmCompression::EacRSigned |
                PkmCompression::EacRgSigned => Some(CompressionFamily::Eac),
                PkmCompression::Unknown => None,
            },
            
            #[cfg(feature = "eac")]
            ImageType::Eac(compression) => match compression {
                PkmCompression::EacR | PkmCompression::EacRg | PkmCompression::EacRSigned |
                PkmCompression::EacRgSigned => Some(CompressionFamily::Eac),
                PkmCompression::Etc1 | PkmCompression::Etc2 | PkmCompression::Etc2A1 |
                PkmCompression::Etc2A8 => Some(CompressionFamily::Etc),
                PkmCompression::Unknown => None,
            },
            
            #[cfg(feature = "pvrtc")]
            ImageType::Pvrtc(compression) => match compression {
                PvrtcCompression::Pvrtc2BppRgb | PvrtcCompression::Pvrtc2BppRgba |
                PvrtcCompression::Pvrtc4BppRgb | PvrtcCompression::Pvrtc4BppRgba => 
                    Some(CompressionFamily::Pvrtc),
                PvrtcCompression::Etc2Rgb | PvrtcCompression::Etc2Rgba | 
                PvrtcCompression::Etc2RgbA1 => Some(CompressionFamily::Etc),
                PvrtcCompression::EacR11 | PvrtcCompression::EacRg11 => Some(CompressionFamily::Eac),
                PvrtcCompression::Unknown => None,
            },
            
            #[cfg(feature = "atc")]
            ImageType::Atc(_) => Some(CompressionFamily::Atc),
            
            #[cfg(feature = "astc")]
            ImageType::Astc => Some(CompressionFamily::Astc),
            
            // Simple formats don't have compression families
            _ => None,
        }
    }

    /// Returns true if the image uses block compression (BC/DXT family)
    ///
    /// Block compression includes BC1-7 formats (also known as DXT1-5, ATI1-2).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use imagesize::{ImageType, DdsCompression};
    ///
    /// let bc1_type = ImageType::Dds(DdsCompression::Bc1);
    /// assert!(bc1_type.is_block_compressed());
    ///
    /// let png_type = ImageType::Png;
    /// assert!(!png_type.is_block_compressed());
    /// ```
    pub fn is_block_compressed(&self) -> bool {
        matches!(self.compression_family(), Some(CompressionFamily::BlockCompression))
    }

    /// Returns the container format name for texture formats
    ///
    /// Returns a human-readable string identifying the container format.
    /// Returns None for simple image formats.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use imagesize::{ImageType, DdsCompression, PvrtcCompression};
    ///
    /// let dds_type = ImageType::Dds(DdsCompression::Bc1);
    /// assert_eq!(dds_type.container_format(), Some("DDS"));
    ///
    /// let pvr_type = ImageType::Pvrtc(PvrtcCompression::Pvrtc2BppRgb);
    /// assert_eq!(pvr_type.container_format(), Some("PowerVR"));
    ///
    /// let png_type = ImageType::Png;
    /// assert_eq!(png_type.container_format(), None);
    /// ```
    pub fn container_format(&self) -> Option<&'static str> {
        match self {
            #[cfg(feature = "dds")]
            ImageType::Dds(_) => Some("DDS"),
            
            #[cfg(feature = "etc2")]
            ImageType::Etc2(_) => Some("PKM"),
            
            #[cfg(feature = "eac")]
            ImageType::Eac(_) => Some("PKM"),
            
            #[cfg(feature = "pvrtc")]
            ImageType::Pvrtc(_) => Some("PowerVR"),
            
            #[cfg(feature = "atc")]
            ImageType::Atc(_) => Some("PKM"),  // ATC typically uses PKM containers
            
            #[cfg(feature = "astc")]
            ImageType::Astc => Some("ASTC"),   // Direct ASTC format
            
            #[cfg(feature = "heif")]
            ImageType::Heif(_) => Some("HEIF"),
            
            #[cfg(feature = "ktx2")]
            ImageType::Ktx2 => Some("KTX2"),
            
            // Simple formats don't have containers
            _ => None,
        }
    }

    /// Returns true if the image format supports multiple compression types within the same container
    ///
    /// Some container formats like PowerVR can store different compression algorithms.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use imagesize::{ImageType, PvrtcCompression, DdsCompression};
    ///
    /// let pvr_type = ImageType::Pvrtc(PvrtcCompression::Etc2Rgb);
    /// assert!(pvr_type.is_multi_compression_container());
    ///
    /// let dds_type = ImageType::Dds(DdsCompression::Bc1);
    /// assert!(dds_type.is_multi_compression_container());
    ///
    /// let png_type = ImageType::Png;
    /// assert!(!png_type.is_multi_compression_container());
    /// ```
    pub fn is_multi_compression_container(&self) -> bool {
        match self {
            #[cfg(feature = "dds")]
            ImageType::Dds(_) => true,        // DDS supports BC1-7, RGBA, etc.
            
            #[cfg(feature = "pvrtc")]
            ImageType::Pvrtc(_) => true,      // PowerVR supports PVRTC, ETC2, EAC
            
            #[cfg(feature = "ktx2")]
            ImageType::Ktx2 => true,          // KTX2 supports many formats
            
            _ => false,
        }
    }

    /// Calls the correct image size method based on the image type
    ///
    /// # Arguments
    /// * `reader` - A reader for the data
    pub fn reader_size<R: BufRead + Seek>(&self, reader: &mut R) -> ImageResult<ImageSize> {
        match self {
            #[cfg(feature = "aesprite")]
            ImageType::Aseprite => aesprite::size(reader),
            #[cfg(feature = "astc")]
            ImageType::Astc => astc::size(reader),
            #[cfg(feature = "atc")]
            ImageType::Atc(..) => container::atc::size(reader),
            #[cfg(feature = "bmp")]
            ImageType::Bmp => bmp::size(reader),
            #[cfg(feature = "dds")]
            ImageType::Dds(..) => container::dds::size(reader),
            #[cfg(feature = "eac")]
            ImageType::Eac(..) => container::pkm::size(reader),
            #[cfg(feature = "etc2")]
            ImageType::Etc2(..) => container::pkm::size(reader),
            #[cfg(feature = "exr")]
            ImageType::Exr => exr::size(reader),
            #[cfg(feature = "farbfeld")]
            ImageType::Farbfeld => farbfeld::size(reader),
            #[cfg(feature = "gif")]
            ImageType::Gif => gif::size(reader),
            #[cfg(feature = "hdr")]
            ImageType::Hdr => hdr::size(reader),
            #[cfg(feature = "ico")]
            ImageType::Ico => ico::size(reader),
            #[cfg(feature = "ilbm")]
            ImageType::Ilbm => ilbm::size(reader),
            #[cfg(feature = "jpeg")]
            ImageType::Jpeg => jpeg::size(reader),
            #[cfg(feature = "jxl")]
            ImageType::Jxl => jxl::size(reader),
            #[cfg(feature = "ktx2")]
            ImageType::Ktx2 => ktx2::size(reader),
            #[cfg(feature = "png")]
            ImageType::Png => png::size(reader),
            #[cfg(feature = "pnm")]
            ImageType::Pnm => pnm::size(reader),
            #[cfg(feature = "pvrtc")]
            ImageType::Pvrtc(..) => container::pvrtc::size(reader),
            #[cfg(feature = "psd")]
            ImageType::Psd => psd::size(reader),
            #[cfg(feature = "qoi")]
            ImageType::Qoi => qoi::size(reader),
            #[cfg(feature = "tga")]
            ImageType::Tga => tga::size(reader),
            #[cfg(feature = "tiff")]
            ImageType::Tiff => tiff::size(reader),
            #[cfg(feature = "vtf")]
            ImageType::Vtf => vtf::size(reader),
            #[cfg(feature = "webp")]
            ImageType::Webp => webp::size(reader),

            #[cfg(feature = "heif")]
            ImageType::Heif(..) => heif::size(reader),
        }
    }
}

/// Holds the size information of an image.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ImageSize {
    /// Width of an image in pixels.
    pub width: usize,
    /// Height of an image in pixels.
    pub height: usize,
}

impl Ord for ImageSize {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.width * self.height).cmp(&(other.width * other.height))
    }
}

impl PartialOrd for ImageSize {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
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
    formats::image_type(&mut Cursor::new(header))
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
/// The minimum data required is 12 bytes. Anything shorter will return [`ImageError::IoError`].
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
pub fn size<P: AsRef<Path>>(path: P) -> ImageResult<ImageSize> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    reader_size(reader)
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
/// The minimum data required is 12 bytes. Anything shorter will return [`ImageError::IoError`].
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
    let reader = Cursor::new(data);
    reader_size(reader)
}

/// Get the image size from a reader
///
/// # Arguments
/// * `reader` - A reader for the data
///
/// # Error
///
/// This method will return an [`ImageError`] under the following conditions:
///
/// * The header isn't recognized as a supported image format
/// * The data isn't long enough to find the size for the given format
///
/// The minimum data required is 12 bytes. Anything shorter will return [`ImageError::IoError`].
///
/// # Examples
///
/// ```
/// use std::io::Cursor;
/// use imagesize::reader_size;
///
/// // PNG Header with size 123x321
/// let reader = Cursor::new([
///     0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A,
///     0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52,
///     0x00, 0x00, 0x00, 0x7B, 0x00, 0x00, 0x01, 0x41,
///     0x08, 0x06, 0x00, 0x00, 0x00, 0x9A, 0x38, 0xC4
/// ]);
///
/// match reader_size(reader) {
///     Ok(dim) => {
///         assert_eq!(dim.width, 123);
///         assert_eq!(dim.height, 321);
///     }
///     Err(why) => println!("Error getting reader size: {:?}", why)
/// }
/// ```
///
/// [`ImageError`]: enum.ImageError.html
pub fn reader_size<R: BufRead + Seek>(mut reader: R) -> ImageResult<ImageSize> {
    reader_type(&mut reader)?.reader_size(&mut reader)
}

/// Get the image type from a reader
///
/// # Arguments
/// * `reader` - A reader for the data
///
/// # Remarks
///
/// This will check the header to determine what image type the data is.
pub fn reader_type<R: BufRead + Seek>(mut reader: R) -> ImageResult<ImageType> {
    formats::image_type(&mut reader)
}
