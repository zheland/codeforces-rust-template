pub use iter_min_max::*;
mod iter_min_max {
    use core::cmp::{max, min, Ordering};

    pub fn min_by<T, F: FnOnce(&T, &T) -> Ordering>(lhs: T, rhs: T, compare: F) -> T {
        match compare(&lhs, &rhs) {
            Ordering::Less | Ordering::Equal => lhs,
            Ordering::Greater => rhs,
        }
    }

    pub fn max_by<T, F: FnOnce(&T, &T) -> Ordering>(lhs: T, rhs: T, compare: F) -> T {
        match compare(&lhs, &rhs) {
            Ordering::Less | Ordering::Equal => rhs,
            Ordering::Greater => lhs,
        }
    }

    pub fn min_max_by<T, F: FnOnce(&T, &T) -> Ordering>(lhs: T, rhs: T, compare: F) -> (T, T) {
        match compare(&lhs, &rhs) {
            Ordering::Less | Ordering::Equal => (lhs, rhs),
            Ordering::Greater => (rhs, lhs),
        }
    }

    pub fn min_by_key<T, F: FnMut(&T) -> K, K: Ord>(v1: T, v2: T, mut f: F) -> T {
        min_by(v1, v2, |v1, v2| f(v1).cmp(&f(v2)))
    }

    pub fn max_by_key<T, F: FnMut(&T) -> K, K: Ord>(v1: T, v2: T, mut f: F) -> T {
        max_by(v1, v2, |v1, v2| f(v1).cmp(&f(v2)))
    }

    pub fn min_max_by_key<T, F: FnMut(&T) -> K, K: Ord>(v1: T, v2: T, mut f: F) -> (T, T) {
        min_max_by(v1, v2, |v1, v2| f(v1).cmp(&f(v2)))
    }

    pub trait IteratorMinMax {
        type Item;

        fn min_max(self) -> Option<(Self::Item, Self::Item)>
        where
            Self::Item: Ord;

        fn min_max_by<F>(self, compare: F) -> Option<(Self::Item, Self::Item)>
        where
            F: Clone + FnMut(&Self::Item, &Self::Item) -> Ordering;

        fn min_max_by_key<B, F>(self, f: F) -> Option<(Self::Item, Self::Item)>
        where
            B: Ord,
            F: Clone + FnMut(&Self::Item) -> B;
    }

    impl<I> IteratorMinMax for I
    where
        I: IntoIterator,
        I::Item: Clone,
    {
        type Item = I::Item;

        fn min_max(self) -> Option<(Self::Item, Self::Item)>
        where
            Self::Item: Ord,
        {
            self.into_iter().fold(None, |acc, value| match acc {
                Some(acc) => Some((min(acc.0, value.clone()), max(acc.1, value))),
                None => Some((value.clone(), value)),
            })
        }

        fn min_max_by<F>(self, compare: F) -> Option<(Self::Item, Self::Item)>
        where
            F: Clone + FnMut(&Self::Item, &Self::Item) -> Ordering,
        {
            self.into_iter().fold(None, |acc, value| match acc {
                Some(acc) => Some((
                    min_by(acc.0, value.clone(), compare.clone()),
                    max_by(acc.1, value, compare.clone()),
                )),
                None => Some((value.clone(), value)),
            })
        }

        fn min_max_by_key<B, F>(self, f: F) -> Option<(Self::Item, Self::Item)>
        where
            B: Ord,
            F: Clone + FnMut(&Self::Item) -> B,
        {
            self.into_iter().fold(None, |acc, value| match acc {
                Some(acc) => Some((
                    min_by_key(acc.0, value.clone(), f.clone()),
                    max_by_key(acc.1, value, f.clone()),
                )),
                None => Some((value.clone(), value)),
            })
        }
    }
}
