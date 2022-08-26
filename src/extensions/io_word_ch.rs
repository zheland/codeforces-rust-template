pub use word_ch::*;
mod word_ch {
    use std::io::Write;

    use crate::{def_wr_and_disp_by_ref, Readable, Reader, Writable};

    #[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct WordCh(pub u8);

    #[allow(single_use_lifetimes)]
    impl<'a> Readable<'a> for WordCh {
        #[track_caller]
        fn read<R: Reader>(reader: &'a mut R) -> Self {
            reader.goto_word().unwrap();
            let ch: [u8; 1] = reader.read_chars().unwrap();
            WordCh(ch[0])
        }
    }

    impl Writable for &WordCh {
        #[track_caller]
        fn write<W: Write>(self, writer: &mut W) {
            write!(writer, "{}", self.0 as char).unwrap();
        }
    }

    def_wr_and_disp_by_ref!(() WordCh);
}
