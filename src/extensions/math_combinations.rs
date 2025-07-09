pub use math_combinations::*;
mod math_combinations {
    use core::ops::Div;

    use crate::Factorial;

    #[must_use]
    pub fn combinations<T>(k: usize, n: usize) -> T
    where
        T: Div<Output = T> + Factorial,
    {
        assert!(k <= n);
        T::factorial(n) / T::factorial(k) / T::factorial(n - k)
    }
}
