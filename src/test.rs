#[cfg(test)]
use ::*;

#[test]
fn apng_test() {
    let dim = size("test/test.apng").unwrap();
    assert_eq!(dim.width, 100);
    assert_eq!(dim.height, 100);
}

#[test]
fn bmp_test() {
    let dim = size("test/test.bmp").unwrap();
    assert_eq!(dim.width, 512);
    assert_eq!(dim.height, 512);
}

#[test]
fn gif_test() {
    let dim = size("test/test.gif").unwrap();
    assert_eq!(dim.width, 100);
    assert_eq!(dim.height, 100);
}

#[test]
fn heif_test() {
    let dim = size("test/test.heic").unwrap();
    assert_eq!(dim.width, 1280);
    assert_eq!(dim.height, 720);
}

#[test]
fn jpeg_test() {
    let dim = size("test/test.jpg").unwrap();
    assert_eq!(dim.width, 690);
    assert_eq!(dim.height, 298);
}

#[test]
fn jpeg_extra_info_test() {
    let dim = size("test/extra.jpg").unwrap();
    assert_eq!(dim.width, 1500);
    assert_eq!(dim.height, 844);
}

#[test]
fn png_test() {
    let dim = size("test/test.png").unwrap();
    assert_eq!(dim.width, 690);
    assert_eq!(dim.height, 298);
}

#[test]
fn psd_test() {
    let dim = size("test/test.psd").unwrap();
    assert_eq!(dim.width, 500);
    assert_eq!(dim.height, 500);
}

#[test]
fn tiff_test() {
    let dim = size("test/test.tif").unwrap();
    assert_eq!(dim.width, 1419);
    assert_eq!(dim.height, 1001);
}

#[test]
fn webp_test() {
    let dim = size("test/test.webp").unwrap();
    assert_eq!(dim.width, 716);
    assert_eq!(dim.height, 716);
}

#[test]
fn riffx_webp_test() {
    let dim = size("test/riffx.webp").unwrap();
    assert_eq!(dim.width, 128);
    assert_eq!(dim.height, 128);
}

#[test]
fn blob_test() {
    //  PNG Header with size 123x321
    let data = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 
                    0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52, 
                    0x00, 0x00, 0x00, 0x7B, 0x00, 0x00, 0x01, 0x41,
                    0x08, 0x06, 0x00, 0x00, 0x00, 0x9A, 0x38, 0xC4];

    let dim = blob_size(&data).unwrap();
    assert_eq!(dim.width, 123);
    assert_eq!(dim.height, 321);
}

#[test]
fn blob_too_small_test() {
    let data = vec![0x89, 0x00, 0x01, 0x02];
    assert_eq!(blob_size(&data).is_err(), true);
}

#[test]
fn blob_test_fail() {
    //  Invalid PNG header (0x51 instead of 0x50)
    let data = vec![0x89, 0x51, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 
                    0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52, 
                    0x00, 0x00, 0x00, 0x7B, 0x00, 0x00, 0x01, 0x41,
                    0x08, 0x06, 0x00, 0x00, 0x00, 0x9A, 0x38, 0xC4];

    assert_eq!(blob_size(&data).is_err(), true);
}

#[test]
fn gif_blob_too_small_test() {
    let data = vec![0x47, 0x49, 0x46, 0x38];
    assert_eq!(blob_size(&data).is_err(), true);
}

#[test]
fn issue_9_test() {
    let dim = size("test/issue-9.jpg").unwrap();
    assert_eq!(dim.width, 1360);
    assert_eq!(dim.height, 1904);
}

#[test]
fn jpg_unexpected_eof() {
    let dim = size("test/unexpected_eof.jpg").unwrap();
    assert_eq!(dim.width, 3047);
    assert_eq!(dim.height, 2008);
}

#[test]
fn fuzzer_crashes_fixed() {
    use std::{fs, io};

    let mut entries = fs::read_dir("test/fuzz_crashes").unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>().unwrap();

    entries.sort();

    for entry in entries {
        let data = get_file_as_byte_vec(entry);
        let _ = blob_size(&data);
    }
}

fn get_file_as_byte_vec(filename: std::path::PathBuf) -> Vec<u8> {
    use std::fs;
    let mut f = File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
}