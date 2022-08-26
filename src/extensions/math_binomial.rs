pub use math_binomial::*;
mod math_binomial {
    use core::cmp::PartialOrd;
    use core::ops::{Add, AddAssign, Div, Mul, Sub};
    use std::cmp::min;
    use std::iter::successors;

    use crate::{Five, MulDiv, One, Ten, Three, Two, Zero};

    pub fn binomial<T>(n: T, k: T) -> T
    where
        T: Add<Output = T>
            + AddAssign
            + Copy
            + MulDiv
            + One
            + Ord
            + PartialOrd
            + Sub<Output = T>
            + Zero,
    {
        *Binomial::with(n, k).get()
    }

    #[derive(Clone, Copy, Debug)]
    pub struct Binomial<T> {
        value: T,
        n: T,
        k: T,
    }

    impl<T> Binomial<T> {
        pub fn get(&self) -> &T {
            &self.value
        }

        pub fn n(&self) -> &T {
            &self.n
        }

        pub fn k(&self) -> &T {
            &self.k
        }
    }

    impl<T> Binomial<T>
    where
        T: One + Copy + Zero,
    {
        pub fn new() -> Self {
            Self {
                value: T::one(),
                n: T::zero(),
                k: T::zero(),
            }
        }

        pub fn with_n(n: T) -> Self {
            Self {
                value: T::one(),
                n,
                k: T::zero(),
            }
        }

        pub fn with_nk(nk: T) -> Self {
            Self {
                value: T::one(),
                n: nk,
                k: nk,
            }
        }
    }

    impl<T> Binomial<T>
    where
        T: Add<Output = T> + AddAssign + Copy + MulDiv + One + PartialOrd + Sub<Output = T> + Zero,
    {
        pub fn with(n: T, k: T) -> Self {
            if k > n {
                Self {
                    value: T::zero(),
                    n,
                    k,
                }
            } else if k <= n - k {
                let mut coeff = Self::with_n(n);
                let mut p = T::zero();
                while p < k {
                    coeff = coeff.inc_k();
                    p += T::one();
                }
                coeff
            } else {
                let mut coeff = Self::with_nk(k);
                let mut p = k;
                while p < n {
                    coeff = coeff.inc_n();
                    p += T::one();
                }
                coeff
            }
        }
    }

    impl<T> Binomial<T>
    where
        T: Copy + Sub<Output = T> + Ord,
    {
        pub fn left(&self) -> Self {
            Self {
                value: self.value,
                n: self.n,
                k: min(self.k, self.n - self.k),
            }
        }
    }

    impl<T> Binomial<T>
    where
        T: Add<Output = T> + Copy + MulDiv + One + Sub<Output = T> + Zero,
    {
        pub fn line(n: T) -> impl Iterator<Item = Binomial<T>> {
            successors(Some(Self::with_n(n)), |coeff| Some(coeff.inc_k()))
        }

        pub fn column(nk: T) -> impl Iterator<Item = Binomial<T>> {
            successors(Some(Self::with_nk(nk)), |coeff| Some(coeff.inc_n()))
        }

        pub fn diag(n: T) -> impl Iterator<Item = Binomial<T>> {
            successors(Some(Self::with_n(n)), |coeff| Some(coeff.inc_nk()))
        }
    }

    impl<T> Binomial<T>
    where
        T: Add<Output = T> + Copy + MulDiv + One + Sub<Output = T>,
    {
        pub fn dec_n(&self) -> Binomial<T> {
            Self {
                value: self.value.mul_div(self.n - self.k, self.n),
                n: self.n - T::one(),
                k: self.k,
            }
        }

        pub fn inc_n(&self) -> Binomial<T> {
            Self {
                value: self
                    .value
                    .mul_div(self.n + T::one(), self.n + T::one() - self.k),
                n: self.n + T::one(),
                k: self.k,
            }
        }

        pub fn dec_nk(&self) -> Binomial<T> {
            Self {
                value: self.value.mul_div(self.k, self.n),
                n: self.n - T::one(),
                k: self.k - T::one(),
            }
        }

        pub fn inc_nk(&self) -> Binomial<T> {
            Self {
                value: self.value.mul_div(self.n + T::one(), self.k + T::one()),
                n: self.n + T::one(),
                k: self.k + T::one(),
            }
        }

        pub fn dec_k(&self) -> Binomial<T> {
            Self {
                value: self.value.mul_div(self.k, self.n + T::one() - self.k),
                n: self.n,
                k: self.k - T::one(),
            }
        }

        pub fn inc_k(&self) -> Binomial<T> {
            Self {
                value: self.value.mul_div(self.n - self.k, self.k + T::one()),
                n: self.n,
                k: self.k + T::one(),
            }
        }
    }
}
