use crate::extensions::iter_sums::Sums;
use crate::IntoVec;

#[test]
fn test_sums() {
    assert_eq!(
        vec![1, 10, 2, 20, 3, 30, 4, 40].sums().into_vec(),
        vec![1, 11, 13, 33, 36, 66, 70, 110]
    );
}
