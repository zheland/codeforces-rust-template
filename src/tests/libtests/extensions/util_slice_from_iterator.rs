use crate::extensions::util_slice_from_iterator::SliceFromIterator;

#[test]
fn test_slice_from_iterator() {
    let mut array = [1, 2, 3, 4];
    assert_eq!(array.from_iter(vec![5].into_iter()), 1);
    assert_eq!(array, [5, 2, 3, 4]);
    assert_eq!(array.from_iter(vec![6, 7].into_iter()), 2);
    assert_eq!(array, [6, 7, 3, 4]);
    assert_eq!(array.from_iter(vec![7, 8, 9].into_iter()), 3);
    assert_eq!(array, [7, 8, 9, 4]);
    assert_eq!(array.from_iter(vec![8, 9, 10, 11].into_iter()), 4);
    assert_eq!(array, [8, 9, 10, 11]);
}

#[test]
#[should_panic]
fn test_slice_from_iterator_panic() {
    let mut array = [1, 2, 3, 4];
    assert_eq!(array.from_iter(vec![9, 10, 11, 12, 13].into_iter()), 4);
    assert_eq!(array, [9, 10, 11, 12]);
}
