pub use partial_word::*;
mod partial_word {
    use core::borrow::Borrow;
    use core::str::from_utf8;
    use std::io::Write;

    use crate::{def_wr_and_disp_by_ref, Readable, Reader, SliceWord, StrExt, Word, Writable};

    #[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct PartialWord<const N: usize>(pub [u8; N]);

    impl<const N: usize> PartialWord<N> {
        pub fn as_word(&self) -> Word<&[u8]> {
            Word(&self.0[0..self.len()])
        }

        pub fn len(&self) -> usize {
            self.0.iter().take_while(|&&ch| ch != b'\0').count()
        }
    }

    #[allow(single_use_lifetimes)]
    impl<'a, const N: usize> Readable<'a> for PartialWord<N> {
        #[track_caller]
        fn read<R: Reader>(reader: &'a mut R) -> Self {
            let slice = SliceWord::read(reader);
            assert!(
                slice.len() <= N,
                "source word length exceeds target word length"
            );
            let mut word = [0; N];
            word[0..slice.len()].copy_from_slice(slice.0);
            PartialWord(word)
        }
    }

    impl<const N: usize> Writable for &PartialWord<N> {
        #[track_caller]
        fn write<W: Write>(self, writer: &mut W) {
            self.as_word().write(writer);
        }
    }

    def_wr_and_disp_by_ref!((for const N: usize) PartialWord<N>);
}
