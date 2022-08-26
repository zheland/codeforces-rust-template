pub use iter_prods::*;
mod iter_prods {
    use core::iter::FusedIterator;
    use core::ops::MulAssign;

    use crate::One;

    pub trait Prods {
        type Output;
        fn prods(self) -> Self::Output;
    }

    #[derive(Clone, Debug)]
    pub struct ProdsIter<T, I>(T, I);

    impl<T, I> ProdsIter<T, I>
    where
        T: One,
        I: Iterator<Item = T>,
    {
        pub fn new(iter: I) -> Self {
            Self(T::one(), iter)
        }
    }

    impl<T, I> Prods for I
    where
        T: One,
        I: IntoIterator<Item = T>,
    {
        type Output = ProdsIter<T, I::IntoIter>;
        fn prods(self) -> Self::Output {
            ProdsIter::new(self.into_iter())
        }
    }

    impl<T, I> Iterator for ProdsIter<T, I>
    where
        T: MulAssign + Clone,
        I: Iterator<Item = T>,
    {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            if let Some(next) = self.1.next() {
                self.0 *= next;
                Some(self.0.clone())
            } else {
                None
            }
        }
    }

    impl<T, I> FusedIterator for ProdsIter<T, I>
    where
        T: MulAssign + Clone,
        I: Iterator<Item = T>,
    {
    }
}
