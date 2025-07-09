use crate::extensions::math_log10::{
    Log10, Log10MinWith, I128_POWERS_OF_10, I16_POWERS_OF_10, I32_POWERS_OF_10, I64_POWERS_OF_10,
    I8_POWERS_OF_10, ISIZE_POWERS_OF_10, U128_POWERS_OF_10, U16_POWERS_OF_10, U32_POWERS_OF_10,
    U64_POWERS_OF_10, U8_POWERS_OF_10, USIZE_POWERS_OF_10,
};
use crate::{CheckedMul, One, Ten};

#[test]
fn test_log10_floor() {
    assert_eq!(1.log10_floor(), 0);
    assert_eq!(2.log10_floor(), 0);
    assert_eq!(9.log10_floor(), 0);
    assert_eq!(10.log10_floor(), 1);
    assert_eq!(11.log10_floor(), 1);
    assert_eq!(99.log10_floor(), 1);
    assert_eq!(100.log10_floor(), 2);
    assert_eq!(101.log10_floor(), 2);
    assert_eq!(999.log10_floor(), 2);
    assert_eq!(1_000.log10_floor(), 3);
}

#[test]
fn test_log10_ceil() {
    assert_eq!(1.log10_ceil(), 0);
    assert_eq!(2.log10_ceil(), 1);
    assert_eq!(9.log10_ceil(), 1);
    assert_eq!(10.log10_ceil(), 1);
    assert_eq!(11.log10_ceil(), 2);
    assert_eq!(99.log10_ceil(), 2);
    assert_eq!(100.log10_ceil(), 2);
    assert_eq!(101.log10_ceil(), 3);
    assert_eq!(999.log10_ceil(), 3);
    assert_eq!(1_000.log10_ceil(), 3);
}

#[test]
fn test_log10_floor_min_with() {
    assert_eq!(1.log10_floor_min_with(2), 0);
    assert_eq!(2.log10_floor_min_with(2), 0);
    assert_eq!(9.log10_floor_min_with(2), 0);
    assert_eq!(10.log10_floor_min_with(2), 1);
    assert_eq!(11.log10_floor_min_with(2), 1);
    assert_eq!(99.log10_floor_min_with(2), 1);
    assert_eq!(100.log10_floor_min_with(2), 2);
    assert_eq!(101.log10_floor_min_with(2), 2);
    assert_eq!(999.log10_floor_min_with(2), 2);
    assert_eq!(1_000.log10_floor_min_with(2), 2);
    assert_eq!(1_001.log10_floor_min_with(2), 2);
    assert_eq!(9_999.log10_floor_min_with(2), 2);
}

#[test]
fn test_log10_ceil_min_with() {
    assert_eq!(1.log10_ceil_min_with(2), 0);
    assert_eq!(2.log10_ceil_min_with(2), 1);
    assert_eq!(9.log10_ceil_min_with(2), 1);
    assert_eq!(10.log10_ceil_min_with(2), 1);
    assert_eq!(11.log10_ceil_min_with(2), 2);
    assert_eq!(99.log10_ceil_min_with(2), 2);
    assert_eq!(100.log10_ceil_min_with(2), 2);
    assert_eq!(101.log10_ceil_min_with(2), 2);
    assert_eq!(999.log10_ceil_min_with(2), 2);
    assert_eq!(1_000.log10_ceil_min_with(2), 2);
    assert_eq!(1_001.log10_floor_min_with(2), 2);
    assert_eq!(9_999.log10_floor_min_with(2), 2);
}

#[test]
fn test_round_log10_floor() {
    assert_eq!(1.round_log10_floor(), 1);
    assert_eq!(2.round_log10_floor(), 1);
    assert_eq!(9.round_log10_floor(), 1);
    assert_eq!(10.round_log10_floor(), 10);
    assert_eq!(11.round_log10_floor(), 10);
    assert_eq!(99.round_log10_floor(), 10);
    assert_eq!(100.round_log10_floor(), 100);
    assert_eq!(101.round_log10_floor(), 100);
    assert_eq!(999.round_log10_floor(), 100);
    assert_eq!(1_000.round_log10_floor(), 1_000);
}

#[test]
fn test_round_log10_ceil() {
    assert_eq!(1.round_log10_ceil(), 1);
    assert_eq!(2.round_log10_ceil(), 10);
    assert_eq!(9.round_log10_ceil(), 10);
    assert_eq!(10.round_log10_ceil(), 10);
    assert_eq!(11.round_log10_ceil(), 100);
    assert_eq!(99.round_log10_ceil(), 100);
    assert_eq!(100.round_log10_ceil(), 100);
    assert_eq!(101.round_log10_ceil(), 1_000);
    assert_eq!(999.round_log10_ceil(), 1_000);
    assert_eq!(1_000.round_log10_ceil(), 1_000);
}

#[test]
fn test_all_powers_of_10() {
    fn test_powers<T>(powers: &[T])
    where
        T: core::fmt::Debug + CheckedMul + One + Ten + PartialEq<T>,
    {
        let mut value: T = T::one();
        let mut j = 0;
        loop {
            assert_eq!(value, powers[j]);
            j += 1;
            value = match value.checked_mul(T::ten()) {
                Some(value) => value,
                None => break,
            };
        }
        assert_eq!(j, powers.len());
    }

    test_powers(U8_POWERS_OF_10);
    test_powers(I8_POWERS_OF_10);
    test_powers(U16_POWERS_OF_10);
    test_powers(I16_POWERS_OF_10);
    test_powers(U32_POWERS_OF_10);
    test_powers(I32_POWERS_OF_10);
    test_powers(U64_POWERS_OF_10);
    test_powers(I64_POWERS_OF_10);
    test_powers(U128_POWERS_OF_10);
    test_powers(I128_POWERS_OF_10);
    test_powers(USIZE_POWERS_OF_10);
    test_powers(ISIZE_POWERS_OF_10);
}
