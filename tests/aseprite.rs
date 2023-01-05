#[cfg(test)]
use imagesize::size;

#[test]
fn aseprite_test() {
    let dim = size("test/aseprite/1.ase").unwrap();

    assert_eq!(dim.width, 23);
    assert_eq!(dim.height, 1);

    let dim = size("test/aseprite/2.ase").unwrap();

    assert_eq!(dim.width, 10);
    assert_eq!(dim.height, 20);
}