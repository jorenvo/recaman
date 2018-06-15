extern crate recaman_svg;

use recaman_svg::*;

#[test]
fn empty() {
    assert_eq!(recaman_sequence(0), vec![]);
}

#[test]
fn one() {
    assert_eq!(recaman_sequence(1), vec![0]);
}

#[test]
fn two() {
    assert_eq!(recaman_sequence(2), vec![0, 1]);
}

#[test]
fn three() {
    assert_eq!(recaman_sequence(3), vec![0, 1, 3]);
}

#[test]
fn four() {
    assert_eq!(recaman_sequence(4), vec![0, 1, 3, 6]);
}

#[test]
fn five() {
    assert_eq!(recaman_sequence(5), vec![0, 1, 3, 6, 2]);
}

#[test]
fn seventy() {
    assert_eq!(
        recaman_sequence(70),
        vec![
            0, 1, 3, 6, 2, 7, 13, 20, 12, 21, 11, 22, 10, 23, 9, 24, 8, 25, 43, 62, 42, 63, 41, 18,
            42, 17, 43, 16, 44, 15, 45, 14, 46, 79, 113, 78, 114, 77, 39, 78, 38, 79, 37, 80, 36,
            81, 35, 82, 34, 83, 33, 84, 32, 85, 31, 86, 30, 87, 29, 88, 28, 89, 27, 90, 26, 91,
            157, 224, 156, 225,
        ]
    );
}
