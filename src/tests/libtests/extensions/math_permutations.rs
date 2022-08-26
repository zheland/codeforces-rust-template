use crate::extensions::math_permutations::{k_permutations, permutations};

#[test]
fn test_permutations() {
    assert_eq!(permutations::<u64>(1), 1);
    assert_eq!(permutations::<u64>(2), 2);
    assert_eq!(permutations::<u64>(3), 6);
    assert_eq!(permutations::<u64>(4), 24);
}

#[test]
fn test_k_permutations() {
    assert_eq!(k_permutations::<u64>(1, 4), 4);
    assert_eq!(k_permutations::<u64>(2, 4), 12);
    assert_eq!(k_permutations::<u64>(3, 4), 24);
    assert_eq!(k_permutations::<u64>(4, 4), 24);
}
