use crate::extensions::math_log2::Log2;

#[test]
fn test_log2_floor() {
    assert_eq!(1.log2_floor(), 0);
    assert_eq!(2.log2_floor(), 1);
    assert_eq!(3.log2_floor(), 1);
    assert_eq!(4.log2_floor(), 2);
    assert_eq!(5.log2_floor(), 2);
    assert_eq!(6.log2_floor(), 2);
    assert_eq!(7.log2_floor(), 2);
    assert_eq!(8.log2_floor(), 3);
    assert_eq!(9.log2_floor(), 3);
}

#[test]
fn test_log2_ceil() {
    assert_eq!(1.log2_ceil(), 0);
    assert_eq!(2.log2_ceil(), 1);
    assert_eq!(3.log2_ceil(), 2);
    assert_eq!(4.log2_ceil(), 2);
    assert_eq!(5.log2_ceil(), 3);
    assert_eq!(6.log2_ceil(), 3);
    assert_eq!(7.log2_ceil(), 3);
    assert_eq!(8.log2_ceil(), 3);
    assert_eq!(9.log2_ceil(), 4);
}

#[test]
fn test_round_log2_floor() {
    assert_eq!(1.round_log2_floor(), 1);
    assert_eq!(2.round_log2_floor(), 2);
    assert_eq!(3.round_log2_floor(), 2);
    assert_eq!(4.round_log2_floor(), 4);
    assert_eq!(5.round_log2_floor(), 4);
    assert_eq!(6.round_log2_floor(), 4);
    assert_eq!(7.round_log2_floor(), 4);
    assert_eq!(8.round_log2_floor(), 8);
    assert_eq!(9.round_log2_floor(), 8);
}

#[test]
fn test_round_log2_ceil() {
    assert_eq!(1.round_log2_ceil(), 1);
    assert_eq!(2.round_log2_ceil(), 2);
    assert_eq!(3.round_log2_ceil(), 4);
    assert_eq!(4.round_log2_ceil(), 4);
    assert_eq!(5.round_log2_ceil(), 8);
    assert_eq!(6.round_log2_ceil(), 8);
    assert_eq!(7.round_log2_ceil(), 8);
    assert_eq!(8.round_log2_ceil(), 8);
    assert_eq!(9.round_log2_ceil(), 16);
}
