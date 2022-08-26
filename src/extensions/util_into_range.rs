pub use util_into_range::*;
mod util_into_range {
    use core::ops::RangeBounds;

    pub trait IsInRange: Sized {
        fn into_range<R: RangeBounds<Self>>(self, range: R) -> Option<Self>;
    }

    impl<T> IsInRange for T
    where
        T: Copy + Ord + PartialOrd,
    {
        fn into_range<R: RangeBounds<Self>>(self, range: R) -> Option<Self> {
            if range.contains(&self) {
                Some(self)
            } else {
                None
            }
        }
    }
}
