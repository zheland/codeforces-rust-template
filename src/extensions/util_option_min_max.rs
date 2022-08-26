pub use util_option_min_max::*;
mod util_option_min_max {
    pub trait OptionMinMax<T> {
        fn omin(self, other: T) -> T;
        fn omax(self, other: T) -> T;
    }

    impl<T: Ord> OptionMinMax<T> for Option<T> {
        fn omin(self, other: T) -> T {
            match self {
                Some(value) => value.min(other),
                None => other,
            }
        }

        fn omax(self, other: T) -> T {
            match self {
                Some(value) => value.max(other),
                None => other,
            }
        }
    }

    impl<T: Ord> OptionMinMax<Option<T>> for Option<T> {
        fn omin(self, other: Option<T>) -> Option<T> {
            match (self, other) {
                (Some(value), Some(other)) => Some(value.min(other)),
                (Some(value), None) => Some(value),
                (None, Some(other)) => Some(other),
                (None, None) => None,
            }
        }

        fn omax(self, other: Option<T>) -> Option<T> {
            match (self, other) {
                (Some(value), Some(other)) => Some(value.max(other)),
                (Some(value), None) => Some(value),
                (None, Some(other)) => Some(other),
                (None, None) => None,
            }
        }
    }
}
