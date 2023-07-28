pub use modular::*;
mod modular {
    use core::cmp::{Ord, Ordering};
    use core::ops::{
        Add, AddAssign, BitAnd, Div, Mul, MulAssign, Neg, Rem, ShrAssign, Sub, SubAssign,
    };
    use std::ops::DivAssign;

    use crate::{
        gcd, Abs, Five, ModularMul, ModularPow, MulDiv, One, RemEuclid, Ten, Three, Two, Unsigned,
        Zero,
    };

    pub trait ConstValue: Default {
        type Output;
        fn get() -> Self::Output;
    }

    pub trait Value: Copy {
        type Output;
        fn get(self) -> Self::Output;
    }

    pub trait ValueEulersPhi: Value {
        fn eulers_phi(self) -> Self::Output;
    }

    impl<T: Copy> Value for (T,) {
        type Output = T;
        fn get(self) -> Self::Output {
            self.0
        }
    }

    impl<T: Copy + ConstValue> Value for T {
        type Output = <Self as ConstValue>::Output;
        fn get(self) -> Self::Output {
            Self::get()
        }
    }

    #[derive(Clone, Copy, Debug)]
    pub struct ValueWithEulersPhi<T>(T, T);

    impl<T> ValueWithEulersPhi<T> {
        pub fn new(value: T, eulers_phi: T) -> Self {
            Self(value, eulers_phi)
        }
    }

    impl<T: Copy> Value for ValueWithEulersPhi<T> {
        type Output = T;
        fn get(self) -> Self::Output {
            self.0
        }
    }

    impl<T: Copy> ValueEulersPhi for ValueWithEulersPhi<T> {
        fn eulers_phi(self) -> Self::Output {
            self.1
        }
    }

    pub trait PrimeValue: Copy + ConstValue {}
    impl PrimeValue for P1_000_000_007 {}
    impl PrimeValue for P998_244_353 {}

    impl<T: PrimeValue> ValueEulersPhi for T
    where
        T::Output: One + Sub<Output = T::Output>,
    {
        fn eulers_phi(self) -> Self::Output {
            Self::get() - Self::Output::one()
        }
    }

    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct P998_244_353;

    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct P1_000_000_007;

    impl ConstValue for P998_244_353 {
        type Output = u32;
        fn get() -> Self::Output {
            998_244_353
        }
    }

    impl ConstValue for P1_000_000_007 {
        type Output = u32;
        fn get() -> Self::Output {
            1_000_000_007
        }
    }

    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct N998_244_353;

    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct N1_000_000_007;

    impl ConstValue for N998_244_353 {
        type Output = i64;
        fn get() -> Self::Output {
            998_244_353
        }
    }

    impl ConstValue for N1_000_000_007 {
        type Output = i64;
        fn get() -> Self::Output {
            1_000_000_007
        }
    }

    #[allow(clippy::derive_hash_xor_eq)]
    #[derive(Clone, Copy, Debug, Default, Hash)]
    pub struct Modular<T, M>(pub T, pub M);

    impl<T, M> Modular<T, M> {
        pub fn new(value: T, modulus: M) -> Self
        where
            T: RemEuclid + Unsigned,
            M: Clone + Value<Output = T>,
        {
            let value = value.rem_euclid(modulus.get());
            Self(value, modulus)
        }

        pub fn from_raw(value: T, modulus: M) -> Self
        where
            T: Unsigned,
            M: Value<Output = T>,
        {
            Self(value, modulus)
        }

        pub fn from_raw_value(value: T) -> Self
        where
            T: Unsigned,
            M: ConstValue + Default + Value<Output = T>,
        {
            Self(value, M::default())
        }

        pub fn value(self) -> T {
            self.0
        }

        pub fn modulus(self) -> M {
            self.1
        }
    }

    impl<T, M> From<T> for Modular<T, M>
    where
        T: RemEuclid + Unsigned,
        M: Copy + ConstValue<Output = T>,
    {
        fn from(value: T) -> Self {
            Self::new(value, M::default())
        }
    }

    impl<T, M> Neg for Modular<T, M>
    where
        T: Add<Output = T> + PartialOrd + RemEuclid + Sub<Output = T> + Zero,
        M: Value<Output = T>,
    {
        type Output = Self;

        fn neg(self) -> Self {
            if self.0.is_zero() {
                self
            } else {
                Self(self.1.get() - self.0, self.1)
            }
        }
    }

    impl<T, M> Add for Modular<T, M>
    where
        T: Add<Output = T> + PartialOrd + RemEuclid + Sub<Output = T>,
        M: Value<Output = T>,
    {
        type Output = Self;

        fn add(self, other: Self) -> Self {
            let value = self.0 + other.0;
            if value >= self.1.get() {
                Self(value - self.1.get(), self.1)
            } else {
                Self(value, self.1)
            }
        }
    }

    impl<T, M> Sub for Modular<T, M>
    where
        T: Add<Output = T> + PartialOrd + Sub<Output = T> + Zero,
        M: Value<Output = T>,
    {
        type Output = Self;

        fn sub(self, other: Self) -> Self {
            if self.0 >= other.0 {
                Self(self.0 - other.0, self.1)
            } else {
                Self(self.0 + self.1.get() - other.0, self.1)
            }
        }
    }

    impl<T, M> Mul for Modular<T, M>
    where
        T: ModularMul,
        M: Value<Output = T>,
    {
        type Output = Self;

        fn mul(self, other: Self) -> Self {
            Self(self.0.modular_mul(other.0, self.1.get()), self.1)
        }
    }

    impl<T, M> Div for Modular<T, M>
    where
        T: ModularMul + ModularPow<T> + Sub<Output = T> + Two + Zero,
        M: PrimeValue<Output = T>,
    {
        type Output = Self;

        fn div(self, other: Self) -> Self {
            self.mul(other.inv().unwrap())
        }
    }

    impl<T, M> AddAssign for Modular<T, M>
    where
        T: Add<Output = T> + Copy + PartialOrd + RemEuclid + Sub<Output = T>,
        M: Value<Output = T>,
    {
        fn add_assign(&mut self, other: Self) {
            self.0 = (*self + other).0;
        }
    }

    impl<T, M> SubAssign for Modular<T, M>
    where
        T: Add<Output = T> + Copy + PartialOrd + Sub<Output = T> + Zero,
        M: Value<Output = T>,
    {
        fn sub_assign(&mut self, other: Self) {
            self.0 = (*self - other).0;
        }
    }

    impl<T, M> MulAssign for Modular<T, M>
    where
        T: Copy + ModularMul,
        M: Value<Output = T>,
    {
        fn mul_assign(&mut self, other: Self) {
            self.0 = (*self * other).0;
        }
    }

    impl<T, M> DivAssign for Modular<T, M>
    where
        T: Copy + ModularMul + ModularPow<T> + Sub<Output = T> + Two + Zero,
        M: PrimeValue<Output = T>,
    {
        fn div_assign(&mut self, other: Self) {
            self.0 = (*self / other).0;
        }
    }

    impl<T, M> Add<T> for Modular<T, M>
    where
        Self: Add<Self, Output = Self>,
        T: RemEuclid + Unsigned,
        M: Value<Output = T>,
    {
        type Output = Self;

        fn add(self, other: T) -> Self {
            let modulus = self.1;
            self + Self::new(other, modulus)
        }
    }

    impl<T, M> Sub<T> for Modular<T, M>
    where
        Self: Sub<Self, Output = Self>,
        T: RemEuclid + Unsigned,
        M: Value<Output = T>,
    {
        type Output = Self;

        fn sub(self, other: T) -> Self {
            let modulus = self.1;
            self - Self::new(other, modulus)
        }
    }

    impl<T, M> Mul<T> for Modular<T, M>
    where
        Self: Mul<Self, Output = Self>,
        T: RemEuclid + Unsigned,
        M: Value<Output = T>,
    {
        type Output = Self;

        fn mul(self, other: T) -> Self {
            let modulus = self.1;
            self * Self::new(other, modulus)
        }
    }

    impl<T, M> Div<T> for Modular<T, M>
    where
        Self: Div<Self, Output = Self>,
        T: RemEuclid + Unsigned,
        M: Value<Output = T>,
    {
        type Output = Self;

        fn div(self, other: T) -> Self {
            let modulus = self.1;
            self / Self::new(other, modulus)
        }
    }

    impl<T, M> AddAssign<T> for Modular<T, M>
    where
        Self: AddAssign<Self>,
        T: RemEuclid + Unsigned,
        M: Value<Output = T>,
    {
        fn add_assign(&mut self, other: T) {
            let modulus = self.1;
            *self += Self::new(other, modulus);
        }
    }

    impl<T, M> SubAssign<T> for Modular<T, M>
    where
        Self: SubAssign<Self>,
        T: RemEuclid + Unsigned,
        M: Value<Output = T>,
    {
        fn sub_assign(&mut self, other: T) {
            let modulus = self.1;
            *self -= Self::new(other, modulus);
        }
    }

    impl<T, M> MulAssign<T> for Modular<T, M>
    where
        Self: MulAssign<Self>,
        T: RemEuclid + Unsigned,
        M: Value<Output = T>,
    {
        fn mul_assign(&mut self, other: T) {
            let modulus = self.1;
            *self *= Self::new(other, modulus);
        }
    }

    impl<T, M> DivAssign<T> for Modular<T, M>
    where
        Self: DivAssign<Self>,
        T: RemEuclid + Unsigned,
        M: Value<Output = T>,
    {
        fn div_assign(&mut self, other: T) {
            let modulus = self.1;
            *self /= Self::new(other, modulus);
        }
    }

    impl<T, M> Modular<T, M> {
        #[must_use]
        pub fn pow<U>(self, exp: U) -> Self
        where
            T: ModularPow<U>,
            M: Value<Output = T>,
        {
            Self(self.0.modular_pow(exp, self.1.get()), self.1)
        }
    }

    impl<T, M> Modular<T, M>
    where
        T: ModularPow<T> + Sub<Output = T> + Two + Zero,
        M: PrimeValue<Output = T>,
    {
        // https://en.wikipedia.org/wiki/Fermat%27s_little_theorem
        // p: prime && a % p > 0
        // => a ** (p - 1) % p = 1
        // => a ** (p - 2) % p = a ** -1 % p
        pub fn inv(self) -> Option<Self> {
            if self.0.is_zero() {
                None
            } else {
                Some(self.pow(M::get() - T::two()))
            }
        }
    }

    impl<T, M> Modular<T, M>
    where
        T: Abs + Copy + ModularPow<T> + One + PartialOrd + Rem<Output = T> + Sub<Output = T> + Zero,
        M: Value<Output = T> + ValueEulersPhi,
    {
        pub fn inv_with_eulers_phi(self) -> Option<Self> {
            if self.0.is_zero() || gcd(self.0, self.1.get()) > T::one() {
                None
            } else {
                Some(self.pow(self.1.eulers_phi() - T::one()))
            }
        }
    }

    impl<T: Ord, M> Eq for Modular<T, M> {}

    impl<T: Ord, M> Ord for Modular<T, M> {
        fn cmp(&self, other: &Self) -> Ordering {
            self.0.cmp(&other.0)
        }
    }

    impl<T: Ord, M> PartialEq for Modular<T, M> {
        fn eq(&self, other: &Self) -> bool {
            self.0.eq(&other.0)
        }
    }

    impl<T: Ord, M> PartialOrd for Modular<T, M> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            self.0.partial_cmp(&other.0)
        }
    }

    impl<T: Ord, M> PartialEq<T> for Modular<T, M> {
        fn eq(&self, other: &T) -> bool {
            self.0.eq(other)
        }
    }

    impl<T: Ord, M> PartialOrd<T> for Modular<T, M> {
        fn partial_cmp(&self, other: &T) -> Option<Ordering> {
            self.0.partial_cmp(other)
        }
    }

    macro_rules! def {
        ( $name:ident, $fn:ident, $is:ident ) => {
            impl<T, M> $name for Modular<T, M>
            where
                T: $name + PartialOrd + Unsigned,
                M: ConstValue + Default + Value<Output = T>,
            {
                fn $fn() -> Self {
                    assert!(T::$fn() < M::default().get());
                    Self::from_raw_value(T::$fn())
                }

                fn $is(&self) -> bool {
                    self.0 == T::$fn()
                }
            }
        };
    }

    def!(Zero, zero, is_zero);
    def!(One, one, is_one);
    def!(Two, two, is_two);
    def!(Three, three, is_three);
    def!(Five, five, is_five);
    def!(Ten, ten, is_ten);

    impl<T, M> Unsigned for Modular<T, M> {}

    impl<T, M> MulDiv for Modular<T, M>
    where
        Self: Div<Output = Self> + Mul<Output = Self>,
    {
        fn mul_div(self, mul: Self, div: Self) -> Self {
            self.mul(mul).div(div)
        }
    }

    pub type M1_000_000_007 = Modular<u32, P1_000_000_007>;
    pub type M998_244_353 = Modular<u32, P998_244_353>;

    pub type M07 = M1_000_000_007;
    pub type M53 = M998_244_353;
}
