use crate::extensions::util_into_range::IsInRange;

#[test]
fn test_into_range() {
    assert_eq!((0).into_range(2..10), None);
    assert_eq!((1).into_range(2..10), None);
    assert_eq!((2).into_range(2..10), Some(2));
    assert_eq!((3).into_range(2..10), Some(3));
    assert_eq!((8).into_range(2..10), Some(8));
    assert_eq!((9).into_range(2..10), Some(9));
    assert_eq!((10).into_range(2..10), None);
    assert_eq!((11).into_range(2..10), None);

    assert_eq!((10).into_range(2..=10), Some(10));
    assert_eq!((11).into_range(2..=10), None);
}
