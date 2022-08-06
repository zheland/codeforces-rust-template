#![warn(
    clippy::all,
    clippy::pedantic,
    meta_variable_misuse,
    missing_abi,
    non_ascii_idents,
    pointer_structural_match,
    rust_2018_idioms,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_lifetimes,
    unused_qualifications,
    unused_results,
    variant_size_differences
)]
#![allow(
    clippy::many_single_char_names,
    clippy::missing_errors_doc,
    dead_code,
    non_snake_case,
    unused_imports,
    unused_macros
)]

#[cfg(test)]
mod tests;

use core::borrow::{Borrow, BorrowMut};
use core::cell::RefCell;
use core::cmp::Ordering::{Equal, Greater, Less};
use core::cmp::{max, min};
use core::convert::Infallible;
use core::fmt::{Debug, Display, Formatter, Result as FmtResult};
use core::iter::{empty, once, repeat};
use core::marker::PhantomData;
use core::mem::{replace, swap, take};
use core::str::{from_utf8, FromStr};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque};
use std::io::{
    sink, stderr, stdin, stdout, BufRead, BufReader, BufWriter, Error as IoError,
    ErrorKind as IoErrorKind, Result as IoResult, Stderr, Stdin, Stdout, Write,
};
use std::sync::Arc;

#[cfg(test)]
use crate::tests::{test_with_examples, test_with_interactor, ChannelReader, ChannelWriter};

pub fn problem<I: ReaderExt + WriterExt>(io: &mut I) {
    let a: i32 = io.re();
    let b: i32 = io.re();
    io.li(a);
    io.li(b);
    io.li(a + b);
    [1, 2].sort()
}

const EXAMPLES: &str = r####"
----
====
1
2
----
1
2
3
====
4
5
----
4
5
9
"####;

#[derive(Clone, Debug)]
pub struct Preset {}

#[allow(unused_variables, clippy::needless_pass_by_value)]
pub fn interactor<I: ReaderExt + WriterExt>(io: &mut I, preset: Preset) {
    io.li(1i32);
    io.li(2i32);
    io.fl();
    let a: i32 = io.re();
    let b: i32 = io.re();
    let c: i32 = io.re();
    assert_eq!(a, 1, "{preset:?}");
    assert_eq!(b, 2, "{preset:?}");
    assert_eq!(c, 3, "{preset:?}");
}

/*
    map1, map2, map3, map4, map5
    filter1, filter2, filter3, ...
    swap, swap1, swap2, swap3, ...
    rev, ...
    sort, ...
    first, ...
    second, ...
    third, ...

    .jo() -> JoinedIter, JoinedTuple
    .wo() -> SeparatedIter, SeparatedTuple
    .li() -> SeparatedIter, SeparatedTuple
    .sep(",") -> SeparatedIter, SeparatedTuple

    d! { exec-on-debug }
    Ascii<T = Vec<u8>>
    Dec<T = Vec<u8>>
    Io::{re}
    Io::{wo, ln, fl, wrln, wrlnfl, wr2, ask, ans}
*/

// ========

pub fn main() {
    let mut io = Io::from_stdio();
    problem(&mut io);
}

#[test]
fn test_examples() {
    test_with_examples(|io| problem(io), EXAMPLES);
}

#[test]
fn test_interactor() {
    #[cfg(all(test, feature = "libtests"))]
    test_with_interactor(problem, |io| interactor(io, Preset {}))
}

#[macro_export]
macro_rules! d {
    ( $($arg:tt)* ) => ( if cfg!(debug_assertions) { $($arg)* } )
}

// ========

use io::*;
mod io {
    use std::io::{stdin, stdout, BufReader, BufWriter, Stdin, Stdout};

    use crate::{LineReader, WordWriter};

    #[derive(Clone, Debug)]
    pub struct Io<R, W> {
        pub reader: LineReader<R>,
        pub writer: WordWriter<W>,
    }

    impl<R, W> Io<R, W> {
        pub fn new(reader: R, writer: W) -> Self {
            Self {
                reader: LineReader::new(reader),
                writer: WordWriter::new(writer),
            }
        }
    }

    impl Io<BufReader<Stdin>, BufWriter<Stdout>> {
        pub fn from_stdio() -> Self {
            let reader = LineReader::new(BufReader::new(stdin()));
            let writer = WordWriter::new(BufWriter::new(stdout()));
            Self { reader, writer }
        }
    }
}

// ========

use reader::*;
mod reader {
    use std::io::Result as IoResult;

    use crate::StrExt;

    pub trait Reader {
        fn get_line(&self) -> &[u8];
        fn skip_line(&mut self) -> IoResult<()>;
        fn read_until<F: Fn(&[u8]) -> Option<usize>>(&mut self, offset_fn: F) -> IoResult<&[u8]>;
        fn read_chars<const N: usize>(&mut self) -> Option<[u8; N]>;

        fn goto_word(&mut self) -> IoResult<()> {
            let _: &[u8] = self.read_until(|line| line.word_start_offset())?;
            Ok(())
        }
    }
}

// ========

use reader_ext::*;
mod reader_ext {
    use std::io::BufRead;

    use crate::{Io, Readable};

    pub trait ReaderExt {
        fn re<'a, T: Readable<'a>>(&'a mut self) -> T;
    }

    impl<R: BufRead, W> ReaderExt for Io<R, W> {
        fn re<'a, T: Readable<'a>>(&'a mut self) -> T {
            self.reader.re()
        }
    }
}

// ========

use writer_ext::*;
mod writer_ext {
    use std::io::Write;

    use crate::{Io, Writable};

    pub trait WriterExt {
        fn jo<T: Writable>(&mut self, value: T);
        fn wo<T: Writable>(&mut self, value: T);
        fn ln(&mut self);
        fn fl(&mut self);

        fn ask(&mut self) {
            self.wo('?');
        }

        fn ans(&mut self) {
            self.wo('!');
        }

        fn li<T: Writable>(&mut self, value: T) {
            self.wo(value);
            self.ln();
        }

        fn jo2<T: Writable>(&mut self, value: T) -> &mut Self {
            self.jo(value);
            self
        }

        fn wo2<T: Writable>(&mut self, value: T) -> &mut Self {
            self.wo(value);
            self
        }

        fn ln2(&mut self) -> &mut Self {
            self.ln();
            self
        }

        fn li2<T: Writable>(&mut self, value: T) -> &mut Self {
            self.li(value);
            self
        }

        fn ask2<T: Writable>(&mut self) -> &mut Self {
            self.ask();
            self
        }

        fn ans2(&mut self) -> &mut Self {
            self.ans();
            self
        }
    }

    impl<R, W: Write> WriterExt for Io<R, W> {
        fn jo<T: Writable>(&mut self, value: T) {
            self.writer.jo(value);
        }

        fn wo<T: Writable>(&mut self, value: T) {
            self.writer.wo(value);
        }

        fn ln(&mut self) {
            self.writer.ln();
        }

        fn fl(&mut self) {
            self.writer.fl();
        }
    }
}

// ========

use line_reader::*;
mod line_reader {
    use std::io::{BufRead, Error as IoError, ErrorKind as IoErrorKind, Result as IoResult};

    use crate::{Readable, Reader, ReaderExt, RemainingBytes};

    #[derive(Clone, Debug)]
    pub struct LineReader<R> {
        reader: R,
        line: RemainingBytes,
    }

    impl<R> LineReader<R> {
        pub fn new(reader: R) -> Self {
            Self {
                reader,
                line: RemainingBytes::new(),
            }
        }

        pub fn as_reader(&self) -> &R {
            &self.reader
        }

        pub fn into_reader(self) -> R {
            self.reader
        }
    }

    impl<R: BufRead> Reader for LineReader<R> {
        fn get_line(&self) -> &[u8] {
            self.line.as_bytes()
        }

        fn skip_line(&mut self) -> IoResult<()> {
            let mut line = String::new();
            let len = self.reader.read_line(&mut line)?;
            if len == 0 {
                return Err(IoError::from(IoErrorKind::UnexpectedEof));
            }
            self.line = RemainingBytes::from(line);

            Ok(())
        }

        fn read_until<F: Fn(&[u8]) -> Option<usize>>(&mut self, offset_fn: F) -> IoResult<&[u8]> {
            loop {
                let len = offset_fn(self.line.as_bytes());
                match len {
                    Some(len) => return Ok(self.line.take(len)),
                    None => self.skip_line()?,
                }
            }
        }

        fn read_chars<const N: usize>(&mut self) -> Option<[u8; N]> {
            let len = self.line.as_bytes().len();
            if len >= N {
                Some(self.line.take_array())
            } else {
                None
            }
        }
    }

    impl<R: BufRead> ReaderExt for LineReader<R> {
        fn re<'a, T: Readable<'a>>(&'a mut self) -> T {
            T::read(self)
        }
    }
}

// ========

use word_writer::*;
mod word_writer {
    use std::io::Write;

    use crate::{Writable, WriterExt};

    #[derive(Clone, Debug)]
    pub struct WordWriter<W> {
        writer: W,
        is_seperator_needed: bool,
    }

    impl<W> WordWriter<W> {
        pub fn new(writer: W) -> Self {
            Self {
                writer,
                is_seperator_needed: false,
            }
        }

        pub fn as_writer(&self) -> &W {
            &self.writer
        }

        pub fn into_writer(self) -> W {
            self.writer
        }
    }

    impl<W: Write> WriterExt for WordWriter<W> {
        fn jo<T: Writable>(&mut self, value: T) {
            value.write(&mut self.writer);
            self.is_seperator_needed = true;
        }

        fn wo<T: Writable>(&mut self, value: T) {
            if self.is_seperator_needed {
                write!(self.writer, " ").unwrap();
            }
            value.write(&mut self.writer);
            self.is_seperator_needed = true;
        }

        fn ln(&mut self) {
            writeln!(self.writer).unwrap();
            self.is_seperator_needed = false;
        }

        fn fl(&mut self) {
            self.writer.flush().unwrap()
        }
    }
}

// ========

use readable::*;
mod readable {
    use core::str::{from_utf8, FromStr};

    use crate::{Reader, SliceWord, Word};

    #[allow(single_use_lifetimes)]
    pub trait Readable<'a>: Sized {
        fn read<R: Reader>(reader: &'a mut R) -> Self;
    }

    macro_rules! def {
        ( $( $ty:ty ),* $(,)? ) => {
            $(
                #[allow(single_use_lifetimes)]
                impl<'a> Readable<'a> for $ty {
                    #[track_caller]
                    fn read<R: Reader>(reader: &'a mut R) -> Self {
                        let word = SliceWord::read(reader);
                        FromStr::from_str(from_utf8(word.0).unwrap()).unwrap()
                    }
                }
            )*
        };
    }
    def! {
        u8, u16, u32, u64, u128, usize,
        i8, i16, i32, i64, i128, isize,
        bool, f32, f64,
    }

    macro_rules! def {
        ( $($field:tt: $type:ident),* ) => {
            def!(@impl ());
            def!(@impl ()( $($field: $type),* ));
        };
        ( @impl ( $($field:tt: $type:ident),* ) ) => {
            #[allow(single_use_lifetimes)]
            impl< 'a, $($type),* > Readable<'a> for ( $($type,)* )
            where
                $($type: 'static + for<'b> Readable<'b>,)*
            {
                #[track_caller]
                #[allow(unused_variables, clippy::unused_unit)]
                fn read<R: Reader>(reader: &'a mut R) -> Self {
                    ( $($type::read(reader),)* )
                }
            }
        };
        ( @impl ( $($field:tt: $type:ident),* )() ) => {};
        (
            @impl
            ( $($field:tt: $type:ident),* )
            ( $next_field:tt: $next_type:ident
                $(, $rest_fields:tt: $rest_types:ident)*
            )
        ) => {
            def!(@impl ( $($field: $type,)* $next_field: $next_type ));
            def!(
                @impl ( $($field: $type,)* $next_field: $next_type )
                ( $($rest_fields: $rest_types),* )
            );
        };
    }
    def!(0: T0, 1: T1, 2: T2, 3: T3, 4: T4, 5: T5, 6: T6, 7: T7);

    #[allow(single_use_lifetimes)]
    impl<'a, T, const N: usize> Readable<'a> for [T; N]
    where
        T: 'static + for<'b> Readable<'b>,
    {
        #[track_caller]
        fn read<R: Reader>(reader: &'a mut R) -> Self {
            [(); N].map(|()| T::read(reader))
        }
    }
}

// ========

use writable::*;
mod writable {
    use core::fmt::{Display, Formatter};
    use core::num::{
        NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
        NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
    };
    use std::io::Write;

    pub trait Writable {
        fn write<W: Write>(self, writer: &mut W);
    }

    #[macro_export]
    macro_rules! def_wr_by_ref {
        ( ( $( for $($args:tt)* )? ) $ty:ty ) => {
            #[allow(unused_qualifications)]
            impl $( < $($args)* > )? crate::Writable for $ty {
                fn write<W: ::std::io::Write>(self, writer: &mut W) {
                    crate::Writable::write(&self, writer)
                }
            }

            #[allow(unused_qualifications)]
            impl $( < $($args)* > )? crate::Writable for &mut $ty {
                fn write<W: ::std::io::Write>(self, writer: &mut W) {
                    crate::Writable::write(&*self, writer)
                }
            }
        };
    }

    #[macro_export]
    macro_rules! def_wr_and_disp_by_ref {
        ( ( $( for $($args:tt)* )? ) $ty:ty ) => {
            crate::def_wr_by_ref!( ( $( for $($args)* )? ) $ty );

            impl $( < $($args)* > )? core::fmt::Display for $ty {
                fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) ->  Result<(), core::fmt::Error> {
                    let mut buffer = Vec::new();
                    Writable::write(self, &mut buffer);
                    fmt.write_str(::core::str::from_utf8(&buffer).unwrap())
                }
            }
        };
    }

    macro_rules! def_wr_and_ref_wr {
        ( impl $( ( $($args:tt)* ) )? Writable for $ty:ty {
            $($tt:tt)*
        } ) => {
            impl $( < $($args)* > )? Writable for $ty {
                $($tt)*
            }

            impl $( < $($args)* > )? Writable for &$ty {
                $($tt)*
            }

            impl $( < $($args)* > )? Writable for &mut $ty {
                $($tt)*
            }
        };
    }

    def_wr_and_ref_wr! {
        impl(T: Display, E: Display) Writable for Result<T, E> {
            fn write<W: Write>(self, writer: &mut W) {
                match self {
                    Ok(ok) => write!(writer, "{}", ok).unwrap(),
                    Err(err) => write!(writer, "{}", err).unwrap(),
                }
            }
        }
    }

    macro_rules! def {
        ( $( $ty:ty ),* $(,)? ) => {
            $(
                impl Writable for &$ty {
                    fn write<W: Write>(self, writer: &mut W) {
                        write!(writer, "{}", &self).unwrap();
                    }
                }
                def_wr_by_ref!(() $ty);
            )*
        };
    }
    def! {
        u8, u16, u32, u64, u128, usize,
        i8, i16, i32, i64, i128, isize,
        NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize,
        NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize,
        bool, f32, f64,
        char, &str, String
    }
}

// ========

use str_ext::*;
mod str_ext {
    pub trait StrExt {
        fn word_start_offset(self) -> Option<usize>;
        fn word_end_offset(self) -> Option<usize>;
    }

    impl StrExt for &str {
        fn word_start_offset(self) -> Option<usize> {
            self.bytes().position(|ch| !ch.is_ascii_whitespace())
        }

        fn word_end_offset(self) -> Option<usize> {
            self.bytes().position(|ch| ch.is_ascii_whitespace())
        }
    }

    impl StrExt for &[u8] {
        fn word_start_offset(self) -> Option<usize> {
            self.iter().position(|ch| !ch.is_ascii_whitespace())
        }

        fn word_end_offset(self) -> Option<usize> {
            self.iter().position(|ch| ch.is_ascii_whitespace())
        }
    }
}

// ========

use array_ext::*;
mod array_ext {
    pub trait ArrayExt {
        fn rev(self) -> Self;
    }

    impl<T, const N: usize> ArrayExt for [T; N] {
        fn rev(mut self) -> Self {
            self.reverse();
            self
        }
    }
}

// ========

use remaining_bytes::*;
mod remaining_bytes {
    #[derive(Clone, Debug, Default)]
    pub struct RemainingBytes {
        bytes: Vec<u8>,
        offset: usize,
    }

    impl RemainingBytes {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn take(&mut self, len: usize) -> &[u8] {
            let bytes = &self.bytes[self.offset..self.offset + len];
            self.offset += len;
            bytes
        }

        pub fn take_array<const N: usize>(&mut self) -> [u8; N] {
            let bytes = self.bytes[self.offset..self.offset + N].try_into().unwrap();
            self.offset += N;
            bytes
        }

        pub fn as_bytes(&self) -> &[u8] {
            &self.bytes[self.offset..]
        }
    }

    impl From<String> for RemainingBytes {
        fn from(string: String) -> Self {
            Self {
                bytes: string.into_bytes(),
                offset: 0,
            }
        }
    }
}

// ========

use ch::*;
mod ch {
    use std::io::Write;

    use crate::{def_wr_and_disp_by_ref, Readable, Reader, Writable};

    // TODO: macro to impl display and debug
    #[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct Ch(pub u8);

    #[allow(single_use_lifetimes)]
    impl<'a> Readable<'a> for Ch {
        #[track_caller]
        fn read<R: Reader>(reader: &'a mut R) -> Self {
            reader.goto_word().unwrap();
            let ch: [u8; 1] = reader.read_chars().unwrap();
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

// ========

use chs::*;
mod chs {
    use core::str::from_utf8;
    use std::io::Write;

    use crate::{def_wr_and_disp_by_ref, Readable, Reader, Writable};

    #[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct Chs<const N: usize>(pub [u8; N]);

    #[allow(single_use_lifetimes)]
    impl<'a, const N: usize> Readable<'a> for Chs<N> {
        #[track_caller]
        fn read<R: Reader>(reader: &'a mut R) -> Self {
            reader.goto_word().unwrap();
            let chs = reader
                .read_chars()
                .expect("target word length exceeds source line length");
            Chs(chs)
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

// ========

use word::*;
mod word {
    use core::borrow::Borrow;
    use core::str::from_utf8;
    use std::io::Write;

    use crate::{def_wr_and_disp_by_ref, Readable, Reader, StrExt, Writable};

    #[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct Word<T>(pub T);

    pub type VecWord = Word<Vec<u8>>;
    pub type SliceWord<'a> = Word<&'a [u8]>;
    pub type ArrayWord<const N: usize> = Word<[u8; N]>;

    impl<T: Borrow<[u8]>> Word<T> {
        pub fn len(&self) -> usize {
            self.0.borrow().len()
        }
    }

    #[allow(single_use_lifetimes)]
    impl<'a> Readable<'a> for Word<&'a [u8]> {
        #[track_caller]
        fn read<R: Reader>(reader: &'a mut R) -> Self {
            reader.goto_word().unwrap();
            let word = reader.read_until(|line| line.word_end_offset()).unwrap();
            Word(word)
        }
    }

    #[allow(single_use_lifetimes)]
    impl<'a, const N: usize> Readable<'a> for Word<[u8; N]> {
        #[track_caller]
        fn read<R: Reader>(reader: &'a mut R) -> Self {
            let word = SliceWord::read(reader)
                .0
                .try_into()
                .expect("target word length is not equal to the source word length");
            Word(word)
        }
    }

    #[allow(single_use_lifetimes)]
    impl<'a> Readable<'a> for Word<Vec<u8>> {
        #[track_caller]
        fn read<R: Reader>(reader: &'a mut R) -> Self {
            let word = SliceWord::read(reader);
            Word(word.0.to_vec())
        }
    }

    impl<T: Borrow<[u8]>> Writable for &Word<T> {
        #[track_caller]
        fn write<W: Write>(self, writer: &mut W) {
            write!(writer, "{}", from_utf8(self.0.borrow()).unwrap()).unwrap();
        }
    }

    def_wr_and_disp_by_ref!((for T: Borrow<[u8]>) Word<T>);
}

// ========

use partial_word::*;
mod partial_word {
    use core::borrow::Borrow;
    use core::str::from_utf8;
    use std::io::Write;

    use crate::{def_wr_and_disp_by_ref, Readable, Reader, SliceWord, StrExt, Word, Writable};

    #[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct PartialWord<const N: usize>([u8; N]);

    impl<const N: usize> PartialWord<N> {
        pub fn as_word(&self) -> Word<&[u8]> {
            Word(&self.0[0..self.len()])
        }

        pub fn len(&self) -> usize {
            self.0
                .iter()
                .take_while(|ch| !ch.is_ascii_whitespace())
                .count()
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

// ========

use dec::*;
mod dec {
    use core::borrow::{Borrow, BorrowMut};
    use std::io::Write;

    use crate::{def_wr_and_disp_by_ref, Readable, Reader, SliceWord, Writable};

    #[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct DecBe<T = Vec<u8>>(pub T);

    #[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct DecLe<T = Vec<u8>>(pub T);

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
            dec[0..word.len()].copy_from_slice(word.0);
            for value in &mut dec {
                assert!(*value >= b'0' && *value <= b'9');
                *value -= b'0';
            }
            DecBe(dec)
        }
    }

    #[allow(single_use_lifetimes)]
    impl<'a> Readable<'a> for DecBe<Vec<u8>> {
        #[track_caller]
        fn read<R: Reader>(reader: &'a mut R) -> Self {
            let word = SliceWord::read(reader);
            DecBe(
                word.0
                    .iter()
                    .map(|ch| {
                        assert!((b'0'..=b'9').contains(ch));
                        ch - b'0'
                    })
                    .collect(),
            )
        }
    }

    impl<T: Borrow<[u8]>> Writable for &DecBe<T> {
        #[track_caller]
        fn write<W: Write>(self, writer: &mut W) {
            for &ch in self.0.borrow() {
                assert!(ch <= 9);
                write!(writer, "{}", (ch + b'0') as char).unwrap();
            }
        }
    }

    def_wr_and_disp_by_ref!((for T: Borrow<[u8]>) DecBe<T>);
}

// ========

/*
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct JoinedTuple<T>(T);

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SeperatedTuple<T>(T, &'static str);

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct JoinedIterator<T>(T);

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SeperatedIterator<T>(T, &'static str);


macro_rules! def {
    ( $($field:tt: $type:ident),* ) => {
        def!(@impl ());
        def!(@impl ()( $($field: $type),* ));
    };
    ( @impl ( $($field:tt: $type:ident),* ) ) => {
        impl< $($type),* > Writable for ( $($type,)* )
        where
            $( $type: Writable, )*
        {
            #[track_caller]
            fn write<W: Writer>(self, writer: &mut W) {
                $(
                    writer.write(self.$field);
                )*
            }
        }

        impl< 'a, $($type),* > Writable for &'a ( $($type,)* )
        where
            $( &'a $type: Writable, )*
        {
            #[track_caller]
            fn write<W: Writer>(self, writer: &mut W) {
                $(
                    writer.write(&self.$field);
                )*
            }
        }

        impl< 'a, $($type),* > Writable for &'a mut ( $($type,)* )
        where
            $( &'a mut $type: Writable, )*
        {
            #[track_caller]
            fn write<W: Writer>(self, writer: &mut W) {
                $(
                    writer.write(&mut self.$field);
                )*
            }
        }
    };
    ( @impl ( $($field:tt: $type:ident),* )() ) => {};
    (
        @impl
        ( $($field:tt: $type:ident),* )
        ( $next_field:tt: $next_type:ident
            $(, $rest_fields:tt: $rest_types:ident)*
        )
    ) => {
        def!(@impl ( $($field: $type,)* $next_field: $next_type ));
        def!(
            @impl ( $($field: $type,)* $next_field: $next_type )
            ( $($rest_fields: $rest_types),* )
        );
    };
}
def!(0: T0, 1: T1, 2: T2, 3: T3, 4: T4, 5: T5, 6: T6, 7: T7);

macro_rules! def {
    ( for ( $($arg:tt)* ) $ty:ty ) => {
        impl< $($arg)* > Writable for $ty
        where
            <$ty as IntoIterator>::Item: Writable
        {
            fn write<W: Writer>(self, writer: &mut W) {
                for item in self.into_iter() {
                    writer.write(item);
                }
            }
        }
    };
}
*/
