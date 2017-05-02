# imagesize
Quickly probe the size of various image formats without reading the entire file.

## Usage
Add the following to your Cargo.toml:
```toml
[dependencies]
imagesize = "0.2"
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

## Example
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
