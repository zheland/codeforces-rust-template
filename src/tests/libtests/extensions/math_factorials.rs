use crate::extensions::math_factorials::Factorials;
use crate::Factorial;

#[test]
fn test_factorial_from_0_to_20() {
    let mut factorials = Factorials::new();
    for j in 0..=20 {
        assert_eq!(u64::factorial(j), factorials.get(j));
    }
}

#[test]
fn test_factorial_from_0_to_34() {
    let mut factorials = Factorials::new();
    for j in 0..=34 {
        assert_eq!(u128::factorial(j), factorials.get(j));
    }
}

#[test]
fn test_factorials() {
    let mut factorials: Factorials<i32> = Factorials::new();
    assert_eq!(factorials.get(0), 1);
    assert_eq!(factorials.get(1), 1);
    assert_eq!(factorials.get(2), 2);
    assert_eq!(factorials.get(3), 6);
    assert_eq!(factorials.get(4), 24);
    assert_eq!(factorials.get(5), 120);
}
