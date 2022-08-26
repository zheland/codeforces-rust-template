pub use math_factorials::*;
mod math_factorials {
    use core::ops::{AddAssign, MulAssign};

    use crate::One;

    #[derive(Clone, Debug)]
    pub struct Factorials<T> {
        data: Vec<T>,
        mult1: T,
        mult2: T,
    }

    impl<T: One> Factorials<T> {
        pub fn new() -> Self {
            Self {
                data: vec![T::one()],
                mult1: T::one(),
                mult2: T::one(),
            }
        }
    }

    impl<T: One> Factorials<T> {
        pub fn get(&mut self, n: usize) -> T
        where
            T: Copy + MulAssign + AddAssign + One,
        {
            while n >= self.data.len() {
                self.mult2 *= self.mult1;
                self.mult1 += T::one();
                self.data.push(self.mult2);
            }
            self.data[n]
        }
    }
}
