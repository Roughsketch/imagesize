[![crates.io version]][crates.io link] [![docs-badge][]][docs]

# imagesize
Quickly probe the size of various image formats without reading the entire file.

The goal of this crate is to be able to read the dimensions of a supported image without loading unnecessary data, and without pulling in more dependencies. Most reads only require 16 bytes or less, and more complex formats take advantage of skipping junk data.

## Features
- **Fast**: Reads only the necessary bytes to determine image dimensions
- **Lightweight**: Minimal dependencies
- **Texture Format Support**: Detects compression algorithms in DDS, PowerVR, PKM, and other texture containers  
- **Cross-Container Queries**: Helper methods to identify compression families across different container formats
- **Backward Compatible**: All existing APIs remain unchanged

## Usage
Add the following to your Cargo.toml:
```toml
[dependencies]
imagesize = "0.14"
```

## Supported Image Formats

### Simple Image Formats
* Aseprite
* Avif
* BMP
* EXR
* Farbfeld
* GIF
* HDR
* HEIC / HEIF
* ICO*
* ILBM (IFF)
* JPEG
* JPEG XL
* PNG
* PNM (PBM, PGM, PPM)
* PSD / PSB
* QOI
* TGA
* TIFF
* VTF
* WEBP

### Texture Container Formats with Compression Detection
* **DDS** - DirectDraw Surface with BC1-7 (DXT1-5) compression detection
* **PKM** - ETC1/ETC2/EAC compressed textures  
* **PowerVR** - PVRTC, ETC2, and EAC compressed textures
* **ATC** - Adaptive Texture Compression (Qualcomm Adreno)
* **ASTC** - Adaptive Scalable Texture Compression
* **KTX2** - Khronos Texture Container

If you have a format you think should be added, feel free to create an issue.

*ICO files can contain multiple images, `imagesize` will give the dimensions of the largest one.

## Examples

### From a file
```rust
match imagesize::size("example.webp") {
    Ok(size) => println!("Image dimensions: {}x{}", size.width, size.height),
    Err(why) => println!("Error getting dimensions: {:?}", why)
}
```

### From a vector
```rust
let data = vec![0x47, 0x49, 0x46, 0x38, 0x39, 0x61, 0x64, 0x00, 0x64, 0x00];
match imagesize::blob_size(&data) {
    Ok(size) => println!("Image dimensions: {}x{}", size.width, size.height),
    Err(why) => println!("Error getting dimensions: {:?}", why),
}
```

### Texture Format Detection
For texture container formats, you can detect both the container type and compression algorithm:

```rust
use imagesize::{image_type, ImageType, CompressionFamily};

let data = std::fs::read("texture.dds").unwrap();
match image_type(&data) {
    Ok(ImageType::Dds(compression)) => {
        println!("DDS texture with {:?} compression", compression);
    }
    Ok(ImageType::Pvrtc(compression)) => {
        println!("PowerVR texture with {:?} compression", compression);
    }
    Ok(other) => println!("Other format: {:?}", other),
    Err(e) => println!("Error: {:?}", e),
}
```

### Cross-Container Compression Queries
Use helper methods to query compression information across different container formats:

```rust
use imagesize::{image_type, CompressionFamily};

let data = std::fs::read("texture.pvr").unwrap();
if let Ok(img_type) = image_type(&data) {
    // Group related compression algorithms regardless of container
    match img_type.compression_family() {
        Some(CompressionFamily::Etc) => println!("ETC family compression"),
        Some(CompressionFamily::BlockCompression) => println!("BC/DXT compression"),
        Some(CompressionFamily::Pvrtc) => println!("PVRTC compression"),
        _ => println!("Other or no compression"),
    }
    
    // Query container and compression properties
    if img_type.is_block_compressed() {
        println!("Uses block compression (BC1-7)");
    }
    
    if let Some(container) = img_type.container_format() {
        println!("Container format: {}", container);
    }
    
    if img_type.is_multi_compression_container() {
        println!("Container supports multiple compression types");
    }
}
```

[crates.io link]: https://crates.io/crates/imagesize
[crates.io version]: https://img.shields.io/crates/v/imagesize.svg?style=flat-square
[docs]: https://docs.rs/imagesize
[docs-badge]: https://img.shields.io/badge/docs-online-5023dd.svg?style=flat-square
