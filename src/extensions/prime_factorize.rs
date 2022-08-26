pub use prime_factorize::*;
mod prime_factorize {
    use core::ops::{AddAssign, BitAnd, DivAssign, Mul, Rem, ShrAssign};

    use crate::{Five, One, Three, TrailingZeros, Two, Zero};

    pub fn factorize_to_vec<T: Factorizable>(value: T) -> Vec<(T, usize)> {
        let mut factorization = Vec::new();
        factorize(value, |pair| factorization.push(pair));
        factorization
    }

    pub fn count_multipliers<T: Factorizable>(value: T) -> usize {
        let mut count = 0;
        factorize(value, |pair| count += pair.1);
        count
    }

    pub fn count_multiplier_primes<T: Factorizable>(value: T) -> usize {
        let mut count = 0;
        factorize(value, |_| count += 1);
        count
    }

    pub trait Factorizable:
        AddAssign
        + BitAnd<Output = Self>
        + Copy
        + DivAssign
        + Five
        + Mul<Output = Self>
        + One
        + PartialEq
        + PartialOrd
        + Rem<Output = Self>
        + ShrAssign<u32>
        + Three
        + TrailingZeros
        + Two
        + Zero
    {
    }

    impl<T> Factorizable for T where
        T: AddAssign
            + BitAnd<Output = T>
            + Copy
            + DivAssign
            + Five
            + Mul<Output = T>
            + One
            + PartialEq
            + PartialOrd
            + Rem<Output = T>
            + ShrAssign<u32>
            + Three
            + TrailingZeros
            + Two
            + Zero
    {
    }

    pub fn factorize<T: Factorizable, F: FnMut((T, usize))>(mut value: T, mut func: F) {
        if (value & T::one()).is_zero() {
            let pow2 = value.trailing_zeros();
            value >>= pow2;
            func((T::two(), pow2 as usize));
        }

        {
            let mut pow3 = 0;
            while (value % T::three()).is_zero() {
                pow3 += 1;
                value /= T::three();
            }
            if pow3 > 0 {
                func((T::three(), pow3));
            }
        }

        let mut divisor: T = T::five();
        while divisor * divisor <= value {
            for _ in 0..2 {
                let mut pow = 0;
                while (value % divisor).is_zero() {
                    pow += 1;
                    value /= divisor;
                }
                if pow > 0 {
                    func((divisor, pow));
                }
                divisor += T::two();
            }
            divisor += T::two();
        }

        if !value.is_one() {
            func((value, 1));
        }
    }
}
