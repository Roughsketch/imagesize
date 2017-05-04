# imagesize
Quickly probe the size of various image formats without reading the entire file.

## Usage
Add the following to your Cargo.toml:
```toml
[dependencies]
imagesize = "0.3"
```
And import it using `extern crate`:
```rust
extern crate imagesize;
```

## Supported Image Formats
* BMP
* GIF
* JPEG
* PNG
* WEBP

## Examples

### Note about *_safe
Both functions shown below have an accompanying safe version, i.e. `get_dimensions_safe` and `get_dimensions_from_blob_safe`. The safe variants have added checks to be more certain an image is a file before returning a size. The non-safe versions will only check the first byte of the file which can be dangerous for formats which have ASCII character magic numbers.

### From a file
```rust
let (width, height) = match get_dimensions("example.webp") {
    Ok(dim) => (dim.width, dim.height),
    Err(why) => println!("Error getting dimensions: {:?}", why)
}
```

### From a vector
Where `magic_partial_download` is a function that downloads a specified amount of bytes from a given url.
```rust
let data: Vec<u8> = magic_partial_download("http://example.com/example.jpg", 0x200);
let (width, height) = match get_dimensions_from_blob(&data) {
    Ok(dim) => (dim.width, dim.height),
    Err(why) => println!("Error getting dimensions: {:?}", why)
}
```
