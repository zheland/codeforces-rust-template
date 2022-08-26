pub use iter_sums::*;
mod iter_sums {
    use core::iter::FusedIterator;
    use core::ops::AddAssign;

    use crate::Zero;

    pub trait Sums {
        type Output;
        fn sums(self) -> Self::Output;
    }

    #[derive(Clone, Debug)]
    pub struct SumsIter<T, I>(T, I);

    impl<T, I> SumsIter<T, I>
    where
        T: Zero,
        I: Iterator<Item = T>,
    {
        pub fn new(iter: I) -> Self {
            Self(T::zero(), iter)
        }
    }

    impl<T, I> Sums for I
    where
        T: Zero,
        I: IntoIterator<Item = T>,
    {
        type Output = SumsIter<T, I::IntoIter>;
        fn sums(self) -> Self::Output {
            SumsIter::new(self.into_iter())
        }
    }

    impl<T, I> Iterator for SumsIter<T, I>
    where
        T: AddAssign + Clone,
        I: Iterator<Item = T>,
    {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            if let Some(next) = self.1.next() {
                self.0 += next;
                Some(self.0.clone())
            } else {
                None
            }
        }
    }

    impl<T, I> FusedIterator for SumsIter<T, I>
    where
        T: AddAssign + Clone,
        I: Iterator<Item = T>,
    {
    }
}
