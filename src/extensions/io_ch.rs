pub use ch::*;
mod ch {
    use std::io::Write;

    use crate::{def_wr_and_disp_by_ref, Readable, Reader, Writable};

    #[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct Ch(pub u8);

    #[allow(single_use_lifetimes)]
    impl<'a> Readable<'a> for Ch {
        #[track_caller]
        fn read<R: Reader>(reader: &'a mut R) -> Self {
            let ch: [u8; 1] = loop {
                match reader.read_chars() {
                    Some(chs) => break chs,
                    None => {
                        reader.skip_line().unwrap();
                    }
                }
            };
            Ch(ch[0])
        }
    }

    impl Writable for &Ch {
        #[track_caller]
        fn write<W: Write>(self, writer: &mut W) {
            write!(writer, "{}", self.0 as char).unwrap();
        }
    }

    def_wr_and_disp_by_ref!(() Ch);
}
