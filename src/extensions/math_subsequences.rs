pub use math_subsequences::*;
mod math_subsequences {
    use core::ops::{Add, Div, Mul};

    use crate::{One, Two};

    pub fn subsequences<T>(n: T) -> T
    where
        T: Add<Output = T> + Clone + Div<Output = T> + Mul<Output = T> + One + Two,
    {
        n.clone() * (n + T::one()) / T::two()
    }
}
