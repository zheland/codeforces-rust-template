pub use math_log2::*;
mod math_log2 {
    pub trait Log2
    where
        Self: Sized,
    {
        fn log2_floor(self) -> u32;

        fn log2_ceil(self) -> u32;

        #[must_use]
        fn round_log2_floor(self) -> Self;

        #[must_use]
        fn round_log2_ceil(self) -> Self;
    }

    macro_rules! def {
        ( $($type:ty),* ) => {
            $(
                impl Log2 for $type {
                    fn log2_floor(self) -> u32 {
                        use core::mem::size_of;
                        assert!(self > 0);
                        Self::BITS - self.leading_zeros() - 1
                    }

                    fn log2_ceil(self) -> u32 {
                        assert!(self > 0);
                        let floor = self.log2_floor();
                        let round_floor = 1 << floor;
                        if self == round_floor {
                            floor
                        } else {
                            floor + 1
                        }
                    }

                    fn round_log2_floor(self) -> Self {
                        1 << self.log2_floor()
                    }

                    fn round_log2_ceil(self) -> Self {
                        1 << self.log2_ceil()
                    }
                }
            )*
        };
    }
    def!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
}
