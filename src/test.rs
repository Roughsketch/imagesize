
#[cfg(test)]
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
fn bmp_test_safe() {
    match get_dimensions_safe("test/test.bmp") {
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
fn gif_test_safe() {
    match get_dimensions_safe("test/test.gif") {
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
fn jpeg_test_safe() {
    match get_dimensions_safe("test/test.jpg") {
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
fn png_test_safe() {
    match get_dimensions_safe("test/test.png") {
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

#[test]
fn webp_test_safe() {
    match get_dimensions_safe("test/test.webp") {
        Ok(dim) => {
            assert_eq!(dim.width, 716);
            assert_eq!(dim.height, 716);
        }
        Err(why) => println!("Error getting dimensions: {:?}", why)
    }
}

#[test]
fn blob_test() {
    //  PNG Header with size 123x321
    let data = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 
                    0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52, 
                    0x00, 0x00, 0x00, 0x7B, 0x00, 0x00, 0x01, 0x41,
                    0x08, 0x06, 0x00, 0x00, 0x00, 0x9A, 0x38, 0xC4];

    match get_dimensions_from_blob(&data) {
        Ok(dim) => {
            assert_eq!(dim.width, 123);
            assert_eq!(dim.height, 321);
        }
        Err(why) => println!("Error getting dimensions: {:?}", why)
    }
}

#[test]
fn blob_too_small_test() {
    let data = vec![0x89, 0x00, 0x01, 0x02];
    assert_eq!(get_dimensions_from_blob(&data).is_err(), true);
}

#[test]
fn blob_test_safe() {
    //  PNG Header with size 123x321
    let data = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 
                    0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52, 
                    0x00, 0x00, 0x00, 0x7B, 0x00, 0x00, 0x01, 0x41,
                    0x08, 0x06, 0x00, 0x00, 0x00, 0x9A, 0x38, 0xC4];

    match get_dimensions_from_blob_safe(&data) {
        Ok(dim) => {
            assert_eq!(dim.width, 123);
            assert_eq!(dim.height, 321);
        }
        Err(why) => println!("Error getting dimensions: {:?}", why)
    }
}

#[test]
fn blob_test_fail_safe() {
    //  Invalid PNG header (0x51 instead of 0x50)
    let data = vec![0x89, 0x51, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 
                    0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52, 
                    0x00, 0x00, 0x00, 0x7B, 0x00, 0x00, 0x01, 0x41,
                    0x08, 0x06, 0x00, 0x00, 0x00, 0x9A, 0x38, 0xC4];

    assert_eq!(get_dimensions_from_blob_safe(&data).is_err(), true);
}

#[test]
fn blob_too_small_test_safe() {
    let data = vec![0x89, 0x00, 0x01, 0x02];
    assert_eq!(get_dimensions_from_blob_safe(&data).is_err(), true);
}