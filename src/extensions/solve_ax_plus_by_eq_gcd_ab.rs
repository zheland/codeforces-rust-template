pub use solve_ax_plus_by_eq_gcd_ab::*;
mod solve_ax_plus_by_eq_gcd_ab {
    use core::ops::{Div, Mul, Rem, Sub};

    use crate::{Abs, One, Zero};

    /// Solves: ax + by = gcd(a, b).
    /// Extended euclidean algorithm.
    pub fn solve_ax_plus_by_eq_gcd_ab<T>(lhs: T, rhs: T) -> (T, T, T)
    where
        T: Abs
            + Clone
            + Div<Output = T>
            + Mul<Output = T>
            + One
            + Rem<Output = T>
            + Sub<Output = T>
            + Zero,
    {
        if rhs.is_zero() {
            (lhs.abs(), T::one(), T::zero())
        } else {
            let (gcd, x, y) = solve_ax_plus_by_eq_gcd_ab(rhs.clone(), lhs.clone() % rhs.clone());
            (gcd, y.clone(), x - (lhs / rhs) * y)
        }
    }
}
