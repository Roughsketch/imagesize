
#[cfg(test)]
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use imagesize::ImageSize;

#[test]
fn ord_equality() {
    let a = ImageSize { width: 1920, height: 1080 };
    let b = ImageSize { width: 1080, height: 1920 };

    assert_ne!(a, b);
    assert_eq!(a.cmp(&b), Ordering::Greater);
}

#[test]
fn ord_set() {
    let a = ImageSize { width: 1920, height: 1080 };
    let b = ImageSize { width: 1080, height: 1920 };

    let mut s = BTreeSet::new();
    s.insert(a);
    s.insert(b);

    assert_eq!(s.len(), 2);
    assert!(s.contains(&b));
}

#[test]
fn ord_map() {
    let a = ImageSize { width: 1920, height: 1080 };
    let b = ImageSize { width: 1080, height: 1920 };

    let mut m = BTreeMap::new();
    m.insert(a, "landscape");
    m.insert(b, "portrait");

    assert_eq!(m.len(), 2);
    assert_eq!(m.get(&a), Some("landscape").as_ref());
}

#[test]
fn ord_tie_break() {
    let a = ImageSize { width: 10, height: 100 };
    let b = ImageSize { width: 100, height: 10 };

    assert_eq!(a.cmp(&b), Ordering::Less);
}