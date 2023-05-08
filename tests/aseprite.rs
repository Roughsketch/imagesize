#[cfg(test)]
use imagesize::{size, ImageSize};

#[test]
fn aseprite_test() {
    let dim = size("tests/images/aseprite/1.ase").unwrap();

    assert_eq!(dim, ImageSize { width: 23, height: 1 });

    let dim = size("tests/images/aseprite/2.ase").unwrap();

    assert_eq!(dim, ImageSize { width: 10, height: 20 });
}
