pub use io_dec::*;
mod io_dec {
    use core::borrow::{Borrow, BorrowMut};
    use std::io::Write;

    use crate::{def_wr_and_disp_by_ref, Readable, Reader, SliceWord, Writable};

    #[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct DecBe<T = Vec<u8>>(pub T);

    #[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct DecLe<T = Vec<u8>>(pub T);

    pub type VecDecBe = DecBe<Vec<u8>>;
    pub type VecDecLe = DecLe<Vec<u8>>;
    pub type SliceDecBe<'a> = DecBe<&'a [u8]>;
    pub type SliceDecLe<'a> = DecLe<&'a [u8]>;
    pub type ArrayDecBe<const N: usize> = DecBe<[u8; N]>;
    pub type ArrayDecLe<const N: usize> = DecLe<[u8; N]>;

    impl<T: BorrowMut<[u8]>> DecBe<T> {
        fn into_le(self) -> DecLe<T> {
            let mut digits = self.0;
            digits.borrow_mut().reverse();
            DecLe(digits)
        }
    }

    impl<T: BorrowMut<[u8]>> DecLe<T> {
        fn into_be(self) -> DecBe<T> {
            let mut digits = self.0;
            digits.borrow_mut().reverse();
            DecBe(digits)
        }
    }

    #[allow(single_use_lifetimes)]
    impl<'a, const N: usize> Readable<'a> for DecBe<[u8; N]> {
        #[track_caller]
        fn read<R: Reader>(reader: &'a mut R) -> Self {
            let word = SliceWord::read(reader);
            assert!(
                word.len() <= N,
                "source word length exceeds target word length"
            );
            let mut dec = [0; N];
            dec[N - word.len()..N].copy_from_slice(word.0);
            for value in &mut dec[N - word.len()..N] {
                assert!(*value >= b'0' && *value <= b'9');
                *value -= b'0';
            }
            DecBe(dec)
        }
    }

    #[allow(single_use_lifetimes)]
    impl<'a, const N: usize> Readable<'a> for DecLe<[u8; N]> {
        #[track_caller]
        fn read<R: Reader>(reader: &'a mut R) -> Self {
            let word = SliceWord::read(reader);
            assert!(
                word.len() <= N,
                "source word length exceeds target word length"
            );
            let mut dec = [0; N];
            dec[0..word.len()].copy_from_slice(word.0);
            for value in &mut dec[0..word.len()] {
                assert!(*value >= b'0' && *value <= b'9');
                *value -= b'0';
            }
            dec[0..word.len()].reverse();
            DecLe(dec)
        }
    }

    fn read_dec_be_iter<R: Reader>(reader: &'_ mut R) -> impl '_ + DoubleEndedIterator<Item = u8> {
        let word = SliceWord::read(reader);
        word.0.iter().map(|ch| {
            assert!((b'0'..=b'9').contains(ch));
            ch - b'0'
        })
    }

    #[allow(single_use_lifetimes)]
    impl<'a> Readable<'a> for DecBe<Vec<u8>> {
        #[track_caller]
        fn read<R: Reader>(reader: &'a mut R) -> Self {
            DecBe(read_dec_be_iter(reader).collect())
        }
    }

    #[allow(single_use_lifetimes)]
    impl<'a> Readable<'a> for DecLe<Vec<u8>> {
        #[track_caller]
        fn read<R: Reader>(reader: &'a mut R) -> Self {
            DecLe(read_dec_be_iter(reader).rev().collect())
        }
    }

    impl<T: Borrow<[u8]>> Writable for &DecBe<T> {
        #[track_caller]
        fn write<W: Write>(self, writer: &mut W) {
            for &ch in self.0.borrow().iter().skip_while(|&&ch| ch == b'\0') {
                assert!(ch <= 9);
                write!(writer, "{}", (ch + b'0') as char).unwrap();
            }
        }
    }

    impl<T: Borrow<[u8]>> Writable for &DecLe<T> {
        #[track_caller]
        fn write<W: Write>(self, writer: &mut W) {
            for &ch in self.0.borrow().iter().rev().skip_while(|&&ch| ch == b'\0') {
                assert!(ch <= 9);
                write!(writer, "{}", (ch + b'0') as char).unwrap();
            }
        }
    }

    def_wr_and_disp_by_ref!((for T: Borrow<[u8]>) DecBe<T>);
    def_wr_and_disp_by_ref!((for T: Borrow<[u8]>) DecLe<T>);
}
