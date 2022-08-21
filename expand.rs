#![feature(prelude_import)]
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
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
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
pub fn problem<I: ReaderExt + WriterExt>(io: &mut I) {
    let a: i32 = io.re();
    let b: i32 = io.re();
    io.li(a);
    io.li(b);
    io.li(a + b);
    [1, 2].sort()
}
const EXAMPLES: &'static str = r####"
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
pub struct Preset {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for Preset {
    #[inline]
    fn clone(&self) -> Preset {
        Preset {}
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for Preset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(f, "Preset")
    }
}
#[allow(unused_variables)]
pub fn interactor<I: ReaderExt + WriterExt>(io: &mut I, preset: Preset) {
    io.li(1i32);
    io.li(2i32);
    io.fl();
    let a: i32 = io.re();
    let b: i32 = io.re();
    let c: i32 = io.re();
    match (&a, &1) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::Some(::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_debug(&preset)],
                    )),
                );
            }
        }
    };
    match (&b, &2) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::Some(::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_debug(&preset)],
                    )),
                );
            }
        }
    };
    match (&c, &3) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::Some(::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_debug(&preset)],
                    )),
                );
            }
        }
    };
}
pub fn main() {
    let mut io = Io::from_stdio();
    problem(&mut io);
}
use io::*;
mod io {
    use std::io::{stdin, stdout, BufReader, BufWriter, Stdin, Stdout};
    use crate::{LineReader, WordWriter};
    pub struct Io<R, W> {
        pub reader: LineReader<R>,
        pub writer: WordWriter<W>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<R: ::core::clone::Clone, W: ::core::clone::Clone> ::core::clone::Clone for Io<R, W> {
        #[inline]
        fn clone(&self) -> Io<R, W> {
            Io {
                reader: ::core::clone::Clone::clone(&self.reader),
                writer: ::core::clone::Clone::clone(&self.writer),
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<R: ::core::fmt::Debug, W: ::core::fmt::Debug> ::core::fmt::Debug for Io<R, W> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "Io",
                "reader",
                &&self.reader,
                "writer",
                &&self.writer,
            )
        }
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
use line_reader::*;
mod line_reader {
    use std::io::{BufRead, Error as IoError, ErrorKind as IoErrorKind, Result as IoResult};
    use crate::{Readable, Reader, ReaderExt, RemainingBytes};
    pub struct LineReader<R> {
        reader: R,
        line: RemainingBytes,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<R: ::core::clone::Clone> ::core::clone::Clone for LineReader<R> {
        #[inline]
        fn clone(&self) -> LineReader<R> {
            LineReader {
                reader: ::core::clone::Clone::clone(&self.reader),
                line: ::core::clone::Clone::clone(&self.line),
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<R: ::core::fmt::Debug> ::core::fmt::Debug for LineReader<R> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "LineReader",
                "reader",
                &&self.reader,
                "line",
                &&self.line,
            )
        }
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
use word_writer::*;
mod word_writer {
    use std::io::Write;
    use crate::{Writable, WriterExt};
    pub struct WordWriter<W> {
        writer: W,
        is_seperator_needed: bool,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<W: ::core::clone::Clone> ::core::clone::Clone for WordWriter<W> {
        #[inline]
        fn clone(&self) -> WordWriter<W> {
            WordWriter {
                writer: ::core::clone::Clone::clone(&self.writer),
                is_seperator_needed: ::core::clone::Clone::clone(&self.is_seperator_needed),
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<W: ::core::fmt::Debug> ::core::fmt::Debug for WordWriter<W> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "WordWriter",
                "writer",
                &&self.writer,
                "is_seperator_needed",
                &&self.is_seperator_needed,
            )
        }
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
                {
                    let result = self
                        .writer
                        .write_fmt(::core::fmt::Arguments::new_v1(&[" "], &[]));
                    result
                }
                .unwrap();
            }
            value.write(&mut self.writer);
            self.is_seperator_needed = true;
        }
        fn ln(&mut self) {
            {
                let result = self
                    .writer
                    .write_fmt(::core::fmt::Arguments::new_v1(&["\n"], &[]));
                result
            }
            .unwrap();
            self.is_seperator_needed = false;
        }
        fn fl(&mut self) {
            self.writer.flush().unwrap()
        }
    }
}
use readable::*;
mod readable {
    use core::str::{from_utf8, FromStr};
    use crate::{Reader, SliceWord, Word};
    #[allow(single_use_lifetimes)]
    pub trait Readable<'a>: Sized {
        fn read<R: Reader>(reader: &'a mut R) -> Self;
    }
    #[allow(single_use_lifetimes)]
    impl<'a> Readable<'a> for u8 {
        #[track_caller]
        fn read<R: Reader>(reader: &'a mut R) -> Self {
            let word = SliceWord::read(reader);
            FromStr::from_str(from_utf8(word.0).unwrap()).unwrap()
        }
    }
    #[allow(single_use_lifetimes)]
    impl<'a> Readable<'a> for u16 {
        #[track_caller]
        fn read<R: Reader>(reader: &'a mut R) -> Self {
            let word = SliceWord::read(reader);
            FromStr::from_str(from_utf8(word.0).unwrap()).unwrap()
        }
    }
    #[allow(single_use_lifetimes)]
    impl<'a> Readable<'a> for u32 {
        #[track_caller]
        fn read<R: Reader>(reader: &'a mut R) -> Self {
            let word = SliceWord::read(reader);
            FromStr::from_str(from_utf8(word.0).unwrap()).unwrap()
        }
    }
    #[allow(single_use_lifetimes)]
    impl<'a> Readable<'a> for u64 {
        #[track_caller]
        fn read<R: Reader>(reader: &'a mut R) -> Self {
            let word = SliceWord::read(reader);
            FromStr::from_str(from_utf8(word.0).unwrap()).unwrap()
        }
    }
    #[allow(single_use_lifetimes)]
    impl<'a> Readable<'a> for u128 {
        #[track_caller]
        fn read<R: Reader>(reader: &'a mut R) -> Self {
            let word = SliceWord::read(reader);
            FromStr::from_str(from_utf8(word.0).unwrap()).unwrap()
        }
    }
    #[allow(single_use_lifetimes)]
    impl<'a> Readable<'a> for usize {
        #[track_caller]
        fn read<R: Reader>(reader: &'a mut R) -> Self {
            let word = SliceWord::read(reader);
            FromStr::from_str(from_utf8(word.0).unwrap()).unwrap()
        }
    }
    #[allow(single_use_lifetimes)]
    impl<'a> Readable<'a> for i8 {
        #[track_caller]
        fn read<R: Reader>(reader: &'a mut R) -> Self {
            let word = SliceWord::read(reader);
            FromStr::from_str(from_utf8(word.0).unwrap()).unwrap()
        }
    }
    #[allow(single_use_lifetimes)]
    impl<'a> Readable<'a> for i16 {
        #[track_caller]
        fn read<R: Reader>(reader: &'a mut R) -> Self {
            let word = SliceWord::read(reader);
            FromStr::from_str(from_utf8(word.0).unwrap()).unwrap()
        }
    }
    #[allow(single_use_lifetimes)]
    impl<'a> Readable<'a> for i32 {
        #[track_caller]
        fn read<R: Reader>(reader: &'a mut R) -> Self {
            let word = SliceWord::read(reader);
            FromStr::from_str(from_utf8(word.0).unwrap()).unwrap()
        }
    }
    #[allow(single_use_lifetimes)]
    impl<'a> Readable<'a> for i64 {
        #[track_caller]
        fn read<R: Reader>(reader: &'a mut R) -> Self {
            let word = SliceWord::read(reader);
            FromStr::from_str(from_utf8(word.0).unwrap()).unwrap()
        }
    }
    #[allow(single_use_lifetimes)]
    impl<'a> Readable<'a> for i128 {
        #[track_caller]
        fn read<R: Reader>(reader: &'a mut R) -> Self {
            let word = SliceWord::read(reader);
            FromStr::from_str(from_utf8(word.0).unwrap()).unwrap()
        }
    }
    #[allow(single_use_lifetimes)]
    impl<'a> Readable<'a> for isize {
        #[track_caller]
        fn read<R: Reader>(reader: &'a mut R) -> Self {
            let word = SliceWord::read(reader);
            FromStr::from_str(from_utf8(word.0).unwrap()).unwrap()
        }
    }
    #[allow(single_use_lifetimes)]
    impl<'a> Readable<'a> for bool {
        #[track_caller]
        fn read<R: Reader>(reader: &'a mut R) -> Self {
            let word = SliceWord::read(reader);
            FromStr::from_str(from_utf8(word.0).unwrap()).unwrap()
        }
    }
    #[allow(single_use_lifetimes)]
    impl<'a> Readable<'a> for f32 {
        #[track_caller]
        fn read<R: Reader>(reader: &'a mut R) -> Self {
            let word = SliceWord::read(reader);
            FromStr::from_str(from_utf8(word.0).unwrap()).unwrap()
        }
    }
    #[allow(single_use_lifetimes)]
    impl<'a> Readable<'a> for f64 {
        #[track_caller]
        fn read<R: Reader>(reader: &'a mut R) -> Self {
            let word = SliceWord::read(reader);
            FromStr::from_str(from_utf8(word.0).unwrap()).unwrap()
        }
    }
    #[allow(single_use_lifetimes)]
    impl<'a> Readable<'a> for () {
        #[track_caller]
        #[allow(unused_variables)]
        fn read<R: Reader>(reader: &'a mut R) -> Self {
            ()
        }
    }
    #[allow(single_use_lifetimes)]
    impl<'a, T0> Readable<'a> for (T0,)
    where
        T0: 'static + for<'b> Readable<'b>,
    {
        #[track_caller]
        #[allow(unused_variables)]
        fn read<R: Reader>(reader: &'a mut R) -> Self {
            (T0::read(reader),)
        }
    }
    #[allow(single_use_lifetimes)]
    impl<'a, T0, T1> Readable<'a> for (T0, T1)
    where
        T0: 'static + for<'b> Readable<'b>,
        T1: 'static + for<'b> Readable<'b>,
    {
        #[track_caller]
        #[allow(unused_variables)]
        fn read<R: Reader>(reader: &'a mut R) -> Self {
            (T0::read(reader), T1::read(reader))
        }
    }
    #[allow(single_use_lifetimes)]
    impl<'a, T0, T1, T2> Readable<'a> for (T0, T1, T2)
    where
        T0: 'static + for<'b> Readable<'b>,
        T1: 'static + for<'b> Readable<'b>,
        T2: 'static + for<'b> Readable<'b>,
    {
        #[track_caller]
        #[allow(unused_variables)]
        fn read<R: Reader>(reader: &'a mut R) -> Self {
            (T0::read(reader), T1::read(reader), T2::read(reader))
        }
    }
    #[allow(single_use_lifetimes)]
    impl<'a, T0, T1, T2, T3> Readable<'a> for (T0, T1, T2, T3)
    where
        T0: 'static + for<'b> Readable<'b>,
        T1: 'static + for<'b> Readable<'b>,
        T2: 'static + for<'b> Readable<'b>,
        T3: 'static + for<'b> Readable<'b>,
    {
        #[track_caller]
        #[allow(unused_variables)]
        fn read<R: Reader>(reader: &'a mut R) -> Self {
            (
                T0::read(reader),
                T1::read(reader),
                T2::read(reader),
                T3::read(reader),
            )
        }
    }
    #[allow(single_use_lifetimes)]
    impl<'a, T0, T1, T2, T3, T4> Readable<'a> for (T0, T1, T2, T3, T4)
    where
        T0: 'static + for<'b> Readable<'b>,
        T1: 'static + for<'b> Readable<'b>,
        T2: 'static + for<'b> Readable<'b>,
        T3: 'static + for<'b> Readable<'b>,
        T4: 'static + for<'b> Readable<'b>,
    {
        #[track_caller]
        #[allow(unused_variables)]
        fn read<R: Reader>(reader: &'a mut R) -> Self {
            (
                T0::read(reader),
                T1::read(reader),
                T2::read(reader),
                T3::read(reader),
                T4::read(reader),
            )
        }
    }
    #[allow(single_use_lifetimes)]
    impl<'a, T0, T1, T2, T3, T4, T5> Readable<'a> for (T0, T1, T2, T3, T4, T5)
    where
        T0: 'static + for<'b> Readable<'b>,
        T1: 'static + for<'b> Readable<'b>,
        T2: 'static + for<'b> Readable<'b>,
        T3: 'static + for<'b> Readable<'b>,
        T4: 'static + for<'b> Readable<'b>,
        T5: 'static + for<'b> Readable<'b>,
    {
        #[track_caller]
        #[allow(unused_variables)]
        fn read<R: Reader>(reader: &'a mut R) -> Self {
            (
                T0::read(reader),
                T1::read(reader),
                T2::read(reader),
                T3::read(reader),
                T4::read(reader),
                T5::read(reader),
            )
        }
    }
    #[allow(single_use_lifetimes)]
    impl<'a, T0, T1, T2, T3, T4, T5, T6> Readable<'a> for (T0, T1, T2, T3, T4, T5, T6)
    where
        T0: 'static + for<'b> Readable<'b>,
        T1: 'static + for<'b> Readable<'b>,
        T2: 'static + for<'b> Readable<'b>,
        T3: 'static + for<'b> Readable<'b>,
        T4: 'static + for<'b> Readable<'b>,
        T5: 'static + for<'b> Readable<'b>,
        T6: 'static + for<'b> Readable<'b>,
    {
        #[track_caller]
        #[allow(unused_variables)]
        fn read<R: Reader>(reader: &'a mut R) -> Self {
            (
                T0::read(reader),
                T1::read(reader),
                T2::read(reader),
                T3::read(reader),
                T4::read(reader),
                T5::read(reader),
                T6::read(reader),
            )
        }
    }
    #[allow(single_use_lifetimes)]
    impl<'a, T0, T1, T2, T3, T4, T5, T6, T7> Readable<'a> for (T0, T1, T2, T3, T4, T5, T6, T7)
    where
        T0: 'static + for<'b> Readable<'b>,
        T1: 'static + for<'b> Readable<'b>,
        T2: 'static + for<'b> Readable<'b>,
        T3: 'static + for<'b> Readable<'b>,
        T4: 'static + for<'b> Readable<'b>,
        T5: 'static + for<'b> Readable<'b>,
        T6: 'static + for<'b> Readable<'b>,
        T7: 'static + for<'b> Readable<'b>,
    {
        #[track_caller]
        #[allow(unused_variables)]
        fn read<R: Reader>(reader: &'a mut R) -> Self {
            (
                T0::read(reader),
                T1::read(reader),
                T2::read(reader),
                T3::read(reader),
                T4::read(reader),
                T5::read(reader),
                T6::read(reader),
                T7::read(reader),
            )
        }
    }
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
use writable::*;
mod writable {
    use std::fmt::Display;
    use std::io::Write;
    use std::num::{
        NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
        NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
    };
    pub trait Writable {
        fn write<W: Write>(self, writer: &mut W);
    }
    impl Writable for u8 {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for &u8 {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for &mut u8 {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for u16 {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for &u16 {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for &mut u16 {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for u32 {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for &u32 {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for &mut u32 {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for u64 {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for &u64 {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for &mut u64 {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for u128 {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for &u128 {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for &mut u128 {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for usize {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for &usize {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for &mut usize {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for i8 {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for &i8 {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for &mut i8 {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for i16 {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for &i16 {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for &mut i16 {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for i32 {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for &i32 {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for &mut i32 {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for i64 {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for &i64 {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for &mut i64 {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for i128 {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for &i128 {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for &mut i128 {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for isize {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for &isize {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for &mut isize {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for NonZeroU8 {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for &NonZeroU8 {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for &mut NonZeroU8 {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for NonZeroU16 {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for &NonZeroU16 {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for &mut NonZeroU16 {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for NonZeroU32 {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for &NonZeroU32 {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for &mut NonZeroU32 {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for NonZeroU64 {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for &NonZeroU64 {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for &mut NonZeroU64 {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for NonZeroU128 {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for &NonZeroU128 {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for &mut NonZeroU128 {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for NonZeroUsize {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for &NonZeroUsize {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for &mut NonZeroUsize {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for NonZeroI8 {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for &NonZeroI8 {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for &mut NonZeroI8 {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for NonZeroI16 {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for &NonZeroI16 {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for &mut NonZeroI16 {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for NonZeroI32 {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for &NonZeroI32 {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for &mut NonZeroI32 {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for NonZeroI64 {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for &NonZeroI64 {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for &mut NonZeroI64 {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for NonZeroI128 {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for &NonZeroI128 {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for &mut NonZeroI128 {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for NonZeroIsize {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for &NonZeroIsize {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for &mut NonZeroIsize {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for bool {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for &bool {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for &mut bool {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for f32 {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for &f32 {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for &mut f32 {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for f64 {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for &f64 {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for &mut f64 {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for char {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for &char {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for &mut char {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for &str {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for &&str {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for &mut &str {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for String {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for &String {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl Writable for &mut String {
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&&self)],
                ));
                result
            }
            .unwrap();
        }
    }
    impl<T: Display, E: Display> Writable for Result<T, E> {
        fn write<W: Write>(self, writer: &mut W) {
            match self {
                Ok(ok) => {
                    let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_display(&ok)],
                    ));
                    result
                }
                .unwrap(),
                Err(err) => {
                    let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_display(&err)],
                    ));
                    result
                }
                .unwrap(),
            }
        }
    }
    impl<T: Display, E: Display> Writable for &Result<T, E> {
        fn write<W: Write>(self, writer: &mut W) {
            match self {
                Ok(ok) => {
                    let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_display(&ok)],
                    ));
                    result
                }
                .unwrap(),
                Err(err) => {
                    let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_display(&err)],
                    ));
                    result
                }
                .unwrap(),
            }
        }
    }
    impl<T: Display, E: Display> Writable for &mut Result<T, E> {
        fn write<W: Write>(self, writer: &mut W) {
            match self {
                Ok(ok) => {
                    let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_display(&ok)],
                    ));
                    result
                }
                .unwrap(),
                Err(err) => {
                    let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_display(&err)],
                    ));
                    result
                }
                .unwrap(),
            }
        }
    }
}
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
use remaining_bytes::*;
mod remaining_bytes {
    pub struct RemainingBytes {
        bytes: Vec<u8>,
        offset: usize,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for RemainingBytes {
        #[inline]
        fn clone(&self) -> RemainingBytes {
            RemainingBytes {
                bytes: ::core::clone::Clone::clone(&self.bytes),
                offset: ::core::clone::Clone::clone(&self.offset),
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for RemainingBytes {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "RemainingBytes",
                "bytes",
                &&self.bytes,
                "offset",
                &&self.offset,
            )
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for RemainingBytes {
        #[inline]
        fn default() -> RemainingBytes {
            RemainingBytes {
                bytes: ::core::default::Default::default(),
                offset: ::core::default::Default::default(),
            }
        }
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
use ch::*;
mod ch {
    use std::io::Write;
    use crate::{Readable, Reader, Writable};
    pub struct Ch(pub u8);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Ch {
        #[inline]
        fn clone(&self) -> Ch {
            let _: ::core::clone::AssertParamIsClone<u8>;
            *self
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for Ch {}
    impl ::core::marker::StructuralEq for Ch {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Eq for Ch {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<u8>;
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::hash::Hash for Ch {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.0, state)
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Ord for Ch {
        #[inline]
        fn cmp(&self, other: &Ch) -> ::core::cmp::Ordering {
            ::core::cmp::Ord::cmp(&self.0, &other.0)
        }
    }
    impl ::core::marker::StructuralPartialEq for Ch {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for Ch {
        #[inline]
        fn eq(&self, other: &Ch) -> bool {
            self.0 == other.0
        }
        #[inline]
        fn ne(&self, other: &Ch) -> bool {
            self.0 != other.0
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialOrd for Ch {
        #[inline]
        fn partial_cmp(&self, other: &Ch) -> ::core::option::Option<::core::cmp::Ordering> {
            ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
        }
    }
    #[allow(single_use_lifetimes)]
    impl<'a> Readable<'a> for Ch {
        #[track_caller]
        fn read<R: Reader>(reader: &'a mut R) -> Self {
            reader.goto_word().unwrap();
            let ch: [u8; 1] = reader.read_chars().unwrap();
            Ch(ch[0])
        }
    }
    impl Writable for Ch {
        #[track_caller]
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&(self.0 as char))],
                ));
                result
            }
            .unwrap();
        }
    }
}
use chs::*;
mod chs {
    use core::str::from_utf8;
    use std::io::Write;
    use crate::{Readable, Reader, Writable};
    pub struct Chs<const N: usize>(pub [u8; N]);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<const N: usize> ::core::clone::Clone for Chs<N> {
        #[inline]
        fn clone(&self) -> Chs<N> {
            let _: ::core::clone::AssertParamIsClone<[u8; N]>;
            *self
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<const N: usize> ::core::marker::Copy for Chs<N> {}
    impl<const N: usize> ::core::marker::StructuralEq for Chs<N> {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<const N: usize> ::core::cmp::Eq for Chs<N> {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<[u8; N]>;
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<const N: usize> ::core::hash::Hash for Chs<N> {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.0, state)
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<const N: usize> ::core::cmp::Ord for Chs<N> {
        #[inline]
        fn cmp(&self, other: &Chs<N>) -> ::core::cmp::Ordering {
            ::core::cmp::Ord::cmp(&self.0, &other.0)
        }
    }
    impl<const N: usize> ::core::marker::StructuralPartialEq for Chs<N> {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<const N: usize> ::core::cmp::PartialEq for Chs<N> {
        #[inline]
        fn eq(&self, other: &Chs<N>) -> bool {
            self.0 == other.0
        }
        #[inline]
        fn ne(&self, other: &Chs<N>) -> bool {
            self.0 != other.0
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<const N: usize> ::core::cmp::PartialOrd for Chs<N> {
        #[inline]
        fn partial_cmp(&self, other: &Chs<N>) -> ::core::option::Option<::core::cmp::Ordering> {
            ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
        }
    }
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
    impl<const N: usize> Writable for Chs<N> {
        #[track_caller]
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(
                        &from_utf8(&self.0).unwrap(),
                    )],
                ));
                result
            }
            .unwrap();
        }
    }
}
use word::*;
mod word {
    use core::borrow::Borrow;
    use core::str::from_utf8;
    use std::io::Write;
    use crate::{Readable, Reader, StrExt, Writable};
    pub struct Word<T>(pub T);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<T: ::core::clone::Clone> ::core::clone::Clone for Word<T> {
        #[inline]
        fn clone(&self) -> Word<T> {
            Word(::core::clone::Clone::clone(&self.0))
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<T: ::core::marker::Copy> ::core::marker::Copy for Word<T> {}
    impl<T> ::core::marker::StructuralEq for Word<T> {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<T: ::core::cmp::Eq> ::core::cmp::Eq for Word<T> {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<T>;
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<T: ::core::hash::Hash> ::core::hash::Hash for Word<T> {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.0, state)
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<T: ::core::cmp::Ord> ::core::cmp::Ord for Word<T> {
        #[inline]
        fn cmp(&self, other: &Word<T>) -> ::core::cmp::Ordering {
            ::core::cmp::Ord::cmp(&self.0, &other.0)
        }
    }
    impl<T> ::core::marker::StructuralPartialEq for Word<T> {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<T: ::core::cmp::PartialEq> ::core::cmp::PartialEq for Word<T> {
        #[inline]
        fn eq(&self, other: &Word<T>) -> bool {
            self.0 == other.0
        }
        #[inline]
        fn ne(&self, other: &Word<T>) -> bool {
            self.0 != other.0
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<T: ::core::cmp::PartialOrd> ::core::cmp::PartialOrd for Word<T> {
        #[inline]
        fn partial_cmp(&self, other: &Word<T>) -> ::core::option::Option<::core::cmp::Ordering> {
            ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
        }
    }
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
    impl<T: Borrow<[u8]>> Writable for Word<T> {
        #[track_caller]
        fn write<W: Write>(self, writer: &mut W) {
            {
                let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(
                        &from_utf8(self.0.borrow()).unwrap(),
                    )],
                ));
                result
            }
            .unwrap();
        }
    }
}
use partial_word::*;
mod partial_word {
    use core::borrow::Borrow;
    use core::str::from_utf8;
    use std::io::Write;
    use crate::{Readable, Reader, SliceWord, StrExt, Word, Writable};
    pub struct PartialWord<const N: usize>([u8; N]);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<const N: usize> ::core::clone::Clone for PartialWord<N> {
        #[inline]
        fn clone(&self) -> PartialWord<N> {
            let _: ::core::clone::AssertParamIsClone<[u8; N]>;
            *self
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<const N: usize> ::core::marker::Copy for PartialWord<N> {}
    impl<const N: usize> ::core::marker::StructuralEq for PartialWord<N> {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<const N: usize> ::core::cmp::Eq for PartialWord<N> {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<[u8; N]>;
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<const N: usize> ::core::hash::Hash for PartialWord<N> {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.0, state)
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<const N: usize> ::core::cmp::Ord for PartialWord<N> {
        #[inline]
        fn cmp(&self, other: &PartialWord<N>) -> ::core::cmp::Ordering {
            ::core::cmp::Ord::cmp(&self.0, &other.0)
        }
    }
    impl<const N: usize> ::core::marker::StructuralPartialEq for PartialWord<N> {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<const N: usize> ::core::cmp::PartialEq for PartialWord<N> {
        #[inline]
        fn eq(&self, other: &PartialWord<N>) -> bool {
            self.0 == other.0
        }
        #[inline]
        fn ne(&self, other: &PartialWord<N>) -> bool {
            self.0 != other.0
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<const N: usize> ::core::cmp::PartialOrd for PartialWord<N> {
        #[inline]
        fn partial_cmp(
            &self,
            other: &PartialWord<N>,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
        }
    }
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
            if !(slice.len() <= N) {
                ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                    &["source word length exceeds target word length"],
                    &[],
                ))
            };
            let mut word = [0; N];
            word[0..slice.len()].copy_from_slice(slice.0);
            PartialWord(word)
        }
    }
    impl<const N: usize> Writable for PartialWord<N> {
        #[track_caller]
        fn write<W: Write>(self, writer: &mut W) {
            self.as_word().write(writer)
        }
    }
}
use dec::*;
mod dec {
    use std::borrow::Borrow;
    use std::io::Write;
    use crate::{Readable, Reader, SliceWord, Writable};
    pub struct Dec<T = Vec<u8>>(pub T);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<T: ::core::clone::Clone> ::core::clone::Clone for Dec<T> {
        #[inline]
        fn clone(&self) -> Dec<T> {
            Dec(::core::clone::Clone::clone(&self.0))
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<T: ::core::marker::Copy> ::core::marker::Copy for Dec<T> {}
    impl<T> ::core::marker::StructuralEq for Dec<T> {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<T: ::core::cmp::Eq> ::core::cmp::Eq for Dec<T> {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<T>;
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<T: ::core::hash::Hash> ::core::hash::Hash for Dec<T> {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.0, state)
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<T: ::core::cmp::Ord> ::core::cmp::Ord for Dec<T> {
        #[inline]
        fn cmp(&self, other: &Dec<T>) -> ::core::cmp::Ordering {
            ::core::cmp::Ord::cmp(&self.0, &other.0)
        }
    }
    impl<T> ::core::marker::StructuralPartialEq for Dec<T> {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<T: ::core::cmp::PartialEq> ::core::cmp::PartialEq for Dec<T> {
        #[inline]
        fn eq(&self, other: &Dec<T>) -> bool {
            self.0 == other.0
        }
        #[inline]
        fn ne(&self, other: &Dec<T>) -> bool {
            self.0 != other.0
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<T: ::core::cmp::PartialOrd> ::core::cmp::PartialOrd for Dec<T> {
        #[inline]
        fn partial_cmp(&self, other: &Dec<T>) -> ::core::option::Option<::core::cmp::Ordering> {
            ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
        }
    }
    #[allow(single_use_lifetimes)]
    impl<'a, const N: usize> Readable<'a> for Dec<[u8; N]> {
        #[track_caller]
        fn read<R: Reader>(reader: &'a mut R) -> Self {
            let word = SliceWord::read(reader);
            if word.len() > N {
                ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                    &["source word length exceeds target word length"],
                    &[],
                ));
            }
            let mut dec = [0; N];
            dec[0..word.len()].copy_from_slice(word.0);
            for value in &mut dec {
                if !(*value >= b'0' && *value <= b'9') {
                    ::core::panicking::panic(
                        "assertion failed: *value >= b\\\'0\\\' && *value <= b\\\'9\\\'",
                    )
                };
                *value -= b'0';
            }
            Dec(dec)
        }
    }
    #[allow(single_use_lifetimes)]
    impl<'a> Readable<'a> for Dec<Vec<u8>> {
        #[track_caller]
        fn read<R: Reader>(reader: &'a mut R) -> Self {
            let word = SliceWord::read(reader);
            Dec(word
                .0
                .iter()
                .map(|ch| {
                    if !(b'0'..=b'9').contains(&ch) {
                        ::core::panicking::panic(
                            "assertion failed: (b\\\'0\\\'..=b\\\'9\\\').contains(&ch)",
                        )
                    };
                    ch - b'0'
                })
                .collect())
        }
    }
    impl<T: Borrow<[u8]>> Writable for Dec<T> {
        #[track_caller]
        fn write<W: Write>(self, writer: &mut W) {
            for &ch in self.0.borrow() {
                if !(ch <= 9) {
                    ::core::panicking::panic("assertion failed: ch <= 9")
                };
                {
                    let result = writer.write_fmt(::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_display(&((ch + b'0') as char))],
                    ));
                    result
                }
                .unwrap();
            }
        }
    }
}
