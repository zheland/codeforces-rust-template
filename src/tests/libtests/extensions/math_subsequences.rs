use crate::extensions::math_subsequences::subsequences;

#[test]
fn test_subsequences() {
    assert_eq!(subsequences(1), 1);
    assert_eq!(subsequences(2), 3);
    assert_eq!(subsequences(3), 6);
    assert_eq!(subsequences(4), 10);
}
