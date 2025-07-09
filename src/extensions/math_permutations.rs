pub use math_permutations::*;
mod math_permutations {
    use core::ops::Div;

    use crate::Factorial;

    #[must_use]
    pub fn permutations<T>(n: usize) -> T
    where
        T: Factorial,
    {
        T::factorial(n)
    }

    #[must_use]
    pub fn k_permutations<T>(k: usize, n: usize) -> T
    where
        T: Div<Output = T> + Factorial,
    {
        assert!(k <= n);
        T::factorial(n) / T::factorial(n - k)
    }
}
