use crate::extensions::iter_prods::Prods;
use crate::IntoVec;

#[test]
fn test_prods() {
    assert_eq!(
        vec![1, 10, 2, 20, 3, 30, 4, 40].prods().into_vec(),
        vec![1, 10, 20, 400, 1200, 36000, 144_000, 5_760_000]
    );
}
