use crate::extensions::iter_min_max::IteratorMinMax;

#[test]
fn test_iterator_min_max() {
    let a = [100, 2, 50, 3, 600, 9, 2, 29];
    assert_eq!(a.iter().min_max(), Some((&2, &600)));
    assert_eq!(std::iter::empty::<i32>().min_max(), None);
    assert_eq!(std::iter::once(55).min_max(), Some((55, 55)));
}
