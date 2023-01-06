[![ci-badge][]][ci] [![docs-badge][]][docs] [![crates.io version]][crates.io link]

# imagesize
Quickly probe the size of various image formats without reading the entire file.

The goal of this crate is to be able to read the dimensions of a supported image without loading unnecessary data, and without pulling in more dependencies. Most reads only require 16 bytes or less, and more complex formats take advantage of skipping junk data.

## Usage
Add the following to your Cargo.toml:
```toml
[dependencies]
imagesize = "0.11"
```

## Supported Image Formats
* Aseprite
* BMP
* GIF
* HEIC / HEIF
* ICO*
* JPEG
* JPEG XL
* PNG
* PSD / PSB
* TIFF
* WEBP

If you have a format you think should be added, feel free to create an issue.

*ICO files can contain multiple images, `imagesize` will give the dimensions of the largest one.

## Examples

### From a file
```rust
let (width, height) = match size("example.webp") {
    Ok(dim) => (dim.width, dim.height),
    Err(why) => println!("Error getting dimensions: {:?}", why)
}
```

### From a vector
Where `magic_partial_download` is a function that downloads a specified amount of bytes from a given url.
```rust
let data: Vec<u8> = magic_partial_download("http://example.com/example.jpg", 0x200);
let (width, height) = match blob_size(&data) {
    Ok(dim) => (dim.width, dim.height),
    Err(why) => println!("Error getting dimensions: {:?}", why)
}
```

[ci]: https://travis-ci.org/Roughsketch/imagesize
[ci-badge]: https://img.shields.io/travis/Roughsketch/imagesize.svg?style=flat-square
[crates.io link]: https://crates.io/crates/imagesize
[crates.io version]: https://img.shields.io/crates/v/imagesize.svg?style=flat-square
[docs]: https://docs.rs/imagesize
[docs-badge]: https://img.shields.io/badge/docs-online-5023dd.svg?style=flat-square
