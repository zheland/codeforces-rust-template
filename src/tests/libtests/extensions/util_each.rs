use crate::extensions::util_each::Each;

#[test]
fn test_each() {
    let vec: Vec<_> = [1..=2, 1..=3, 1..=4].each().collect();
    assert_eq!(
        vec,
        [
            [1, 1, 1],
            [1, 1, 2],
            [1, 1, 3],
            [1, 1, 4],
            [1, 2, 1],
            [1, 2, 2],
            [1, 2, 3],
            [1, 2, 4],
            [1, 3, 1],
            [1, 3, 2],
            [1, 3, 3],
            [1, 3, 4],
            [2, 1, 1],
            [2, 1, 2],
            [2, 1, 3],
            [2, 1, 4],
            [2, 2, 1],
            [2, 2, 2],
            [2, 2, 3],
            [2, 2, 4],
            [2, 3, 1],
            [2, 3, 2],
            [2, 3, 3],
            [2, 3, 4],
        ]
    );
}

#[test]
fn test_each_vec() {
    #[allow(clippy::useless_vec)] // false positive
    let vec: Vec<_> = vec![1..=2; 3].each().collect();
    assert_eq!(
        vec,
        [
            [1, 1, 1],
            [1, 1, 2],
            [1, 2, 1],
            [1, 2, 2],
            [2, 1, 1],
            [2, 1, 2],
            [2, 2, 1],
            [2, 2, 2],
        ]
    );
}
