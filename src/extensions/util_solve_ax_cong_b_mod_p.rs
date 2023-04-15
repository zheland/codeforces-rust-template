pub use util_solve_ax_cong_b_mod_p::*;
mod util_solve_ax_cong_b_mod_p {
    use core::ops::{Div, Mul, Rem};

    use crate::{gcd, Abs, ModularInv, Zero};

    pub fn solve_ax_cong_b_mod_p<T>(a: T, b: T, p: T) -> Option<T>
    where
        T: Abs
            + Clone
            + Div<Output = T>
            + Eq
            + Mul<Output = T>
            + PartialEq
            + Rem<Output = T>
            + ModularInv
            + Zero,
    {
        let d = gcd(a.clone(), p.clone());
        if b.clone() % d.clone() == T::zero() {
            let a1 = a / d.clone();
            let b1 = b / d.clone();
            let p1 = p / d;
            Some(a1.modular_inv_prime(p1) * b1)
        } else {
            None
        }
    }
}
