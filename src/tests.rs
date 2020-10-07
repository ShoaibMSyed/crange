use crate::{Range, RangeGet};

#[test]
fn range_simple() {
    assert_eq!([1, 2, 3, 4], [1, 2, 3, 4, 5][Range::<4>]);
}

#[test]
fn range_offset() {
    assert_eq!([2, 3, 4, 5], [1, 2, 3, 4, 5][1..][Range::<4>]);
}

#[test]
#[should_panic(expected = "range index out of bounds")]
fn range_panic() {
    [0, 1, 2][Range::<4>];
}

#[test]
fn range_get_simple() {
    assert_eq!(Some(&[1, 2, 3, 4]), [1, 2, 3, 4, 5].get_range::<4>());
}

#[test]
fn range_get_offset() {
    assert_eq!(Some(&[2, 3, 4, 5]), [1, 2, 3, 4, 5][1..].get_range::<4>());
}

#[test]
fn range_get_none() {
    assert_eq!(None, [0, 1, 2].get_range::<4>());
}