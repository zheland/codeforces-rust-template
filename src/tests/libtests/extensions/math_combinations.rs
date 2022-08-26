use crate::extensions::math_combinations::combinations;

#[test]
fn test_combinations() {
    assert_eq!(combinations::<u64>(0, 4), 1);
    assert_eq!(combinations::<u64>(1, 4), 4);
    assert_eq!(combinations::<u64>(2, 4), 6);
    assert_eq!(combinations::<u64>(3, 4), 4);
    assert_eq!(combinations::<u64>(4, 4), 1);
}
