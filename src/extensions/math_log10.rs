pub use math_log10::*;
mod math_log10 {
    pub trait PowersOf10
    where
        Self: Sized,
    {
        fn powers_of_10() -> &'static [Self];
    }

    pub trait Log10
    where
        Self: Sized,
    {
        fn log10_floor(self) -> u32;
        fn log10_ceil(self) -> u32;
        fn round_log10_floor(self) -> Self;
        fn round_log10_ceil(self) -> Self;
    }

    pub trait Log10MinWith
    where
        Self: Sized,
    {
        fn log10_floor_min_with(self, rhs: u32) -> u32;
        fn log10_ceil_min_with(self, rhs: u32) -> u32;
    }

    macro_rules! def {
        ( $ty:ty, $const:ident, [$($tt:tt)*] ) => { def!($ty, $const, [$($tt)*], 1, [1]); };
        ( $ty:ty, $const:ident, [$first:tt $($rest:tt)*], $value:expr, [$($pows:expr),*] ) => {
            def!($ty, $const, [$($rest)*], $value * 10, [$($pows,)* $value * 10]);
        };
        ( $ty:ty, $const:ident, [], $value:expr, [$($pows:expr),*] ) => {
            pub const $const: &[$ty] = &[ $($pows),* ];
            impl PowersOf10 for $ty {
                fn powers_of_10() -> &'static [Self] {
                    $const
                }
            }
        };
    }
    def!(i8, I8_POWERS_OF_10, [,,]);
    def!(u8, U8_POWERS_OF_10, [,,]);
    def!(i16, I16_POWERS_OF_10, [,,,,]);
    def!(u16, U16_POWERS_OF_10, [,,,,]);
    def!(i32, I32_POWERS_OF_10, [,,,,,,,,,]);
    def!(u32, U32_POWERS_OF_10, [,,,,,,,,,]);
    def!(i64, I64_POWERS_OF_10, [,,,,,,,,,,,,,,,,,,]);
    def!(u64, U64_POWERS_OF_10, [,,,,,,,,,,,,,,,,,,,]);
    def!(i128, I128_POWERS_OF_10, [,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,]);
    def!(u128, U128_POWERS_OF_10, [,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,]);

    #[cfg(target_pointer_width = "32")]
    def!(isize, ISIZE_POWERS_OF_10, [,,,,,,,,,]);
    #[cfg(target_pointer_width = "64")]
    def!(isize, ISIZE_POWERS_OF_10, [,,,,,,,,,,,,,,,,,,]);

    #[cfg(target_pointer_width = "32")]
    def!(usize, USIZE_POWERS_OF_10, [,,,,,,,,,]);
    #[cfg(target_pointer_width = "64")]
    def!(usize, USIZE_POWERS_OF_10, [,,,,,,,,,,,,,,,,,,,]);

    macro_rules! def {
        ( $($type:ty),* ) => {
            $(
                impl Log10 for $type {
                    fn log10_floor(self) -> u32 {
                        assert!(self > 0);
                        let powers = Self::powers_of_10();
                        powers
                            .binary_search(&self)
                            .map_or_else(|err| err - 1, |ok| ok) as u32
                    }

                    fn log10_ceil(self) -> u32 {
                        assert!(self > 0);
                        let powers = Self::powers_of_10();
                        powers.binary_search(&self).map_or_else(|err| err, |ok| ok) as u32
                    }

                    fn round_log10_floor(self) -> Self {
                        Self::powers_of_10()[self.log10_floor() as usize]
                    }

                    fn round_log10_ceil(self) -> Self {
                        Self::powers_of_10()[self.log10_ceil() as usize]
                    }
                }

                impl Log10MinWith for $type {
                    fn log10_floor_min_with(self, rhs: u32) -> u32 {
                        assert!(self > 0);
                        let powers = &Self::powers_of_10()[0..=rhs as usize];
                        powers
                            .binary_search(&self)
                            .map_or_else(|err| err - 1, |ok| ok) as u32
                    }

                    fn log10_ceil_min_with(self, rhs: u32) -> u32 {
                        assert!(self > 0);
                        let powers = &Self::powers_of_10()[0..=rhs as usize];
                        powers
                            .binary_search(&self)
                            .map_or_else(|err| (err as u32).min(rhs), |ok| ok as u32)
                    }
                }
            )*
        };
    }
    def!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
}
