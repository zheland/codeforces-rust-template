pub use slice_from_iterator::*;
mod slice_from_iterator {
    pub trait SliceFromIterator {
        type Item;
        fn set_from_iter<I: Iterator<Item = Self::Item>>(&mut self, iter: I) -> usize;
    }

    impl<T> SliceFromIterator for [T] {
        type Item = T;
        #[track_caller]
        fn set_from_iter<I: Iterator<Item = Self::Item>>(&mut self, iter: I) -> usize {
            let mut len = 0;
            for value in iter {
                assert!(len < self.len());
                self[len] = value;
                len += 1;
            }
            len
        }
    }
}
