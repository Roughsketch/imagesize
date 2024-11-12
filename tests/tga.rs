#[cfg(test)]
use imagesize::{size, ImageSize};

#[test]
#[cfg(feature = "tga")]
fn tga_test() {
    let dim = size("tests/images/tga/test.tga").unwrap();
    assert_eq!(
        dim,
        ImageSize {
            width: 100,
            height: 67
        }
    );
}

#[test]
#[cfg(feature = "tga")]
fn tga_type_1_test() {
    let dim = size("tests/images/tga/type_1.tga").unwrap();
    assert_eq!(
        dim,
        ImageSize {
            width: 350,
            height: 350
        }
    );
}

#[test]
#[cfg(feature = "tga")]
fn tga_type_2_test() {
    let dim = size("tests/images/tga/type_2.tga").unwrap();
    assert_eq!(
        dim,
        ImageSize {
            width: 100,
            height: 67
        }
    );
}

#[test]
#[cfg(feature = "tga")]
fn tga_type_10_test() {
    let dim = size("tests/images/tga/type_10.tga").unwrap();
    assert_eq!(
        dim,
        ImageSize {
            width: 640,
            height: 480
        }
    );
}

#[test]
#[cfg(feature = "tga")]
fn tga_verify() {
    let mut paths = Vec::new();
    for file in walkdir::WalkDir::new("tests/images/tga")
        .into_iter()
        .filter_map(|file| file.ok())
    {
        if file.metadata().unwrap().is_file() {
            paths.push(std::fs::canonicalize(file.path()).unwrap());
        }
    }

    for path in paths {
        assert!(imagesize::size(&path).is_ok());
    }
}
