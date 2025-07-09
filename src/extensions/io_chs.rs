pub use io_chs::*;
mod io_chs {
    use core::str::from_utf8;
    use std::io::Write;

    use crate::{def_wr_and_disp_by_ref, Readable, Reader, Writable};

    #[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct Chs<const N: usize>(pub [u8; N]);

    #[allow(single_use_lifetimes)]
    impl<'a, const N: usize> Readable<'a> for Chs<N> {
        #[track_caller]
        fn read<R: Reader>(reader: &'a mut R) -> Self {
            if reader.get_line().is_empty() {
                reader.skip_line().unwrap();
            }
            let chs = reader
                .read_chars()
                .expect("target word length exceeds source line length");
            Self(chs)
        }
    }

    impl<const N: usize> Writable for &Chs<N> {
        #[track_caller]
        fn write<W: Write>(self, writer: &mut W) {
            write!(writer, "{}", from_utf8(&self.0).unwrap()).unwrap();
        }
    }

    def_wr_and_disp_by_ref!((for const N: usize) Chs<N>);
}
