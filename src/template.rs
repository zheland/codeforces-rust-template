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
    clippy::missing_panics_doc,
    clippy::module_name_repetitions,
    clippy::wildcard_imports,
    dead_code,
    non_snake_case,
    unused_imports,
    unused_macros
)]

#[cfg(test)]
pub mod tests;

#[cfg(test)]
pub mod extensions;

use core::borrow::{Borrow, BorrowMut};
use core::cell::RefCell;
use core::cmp::Ordering::{Equal, Greater, Less};
use core::cmp::{max, min};
use core::convert::Infallible;
use core::fmt::{Debug, Display, Formatter, Result as FmtResult};
use core::iter::{empty, once, repeat, successors};
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
    test_with_interactor(problem, |io| interactor(io, Preset {}))
}

// ========

use debug::*;
mod debug {
    #[macro_export]
    macro_rules! d {
        ( $($arg:tt)* ) => ( if cfg!(debug_assertions) { $($arg)* } )
    }
}

// ========

use util::*;
mod util {
    #[macro_export]
    macro_rules! impl_for_value_and_refs {
        ( impl ($(($($a:tt)*))?) ($b:ty) for ($c:ty) ($($d:tt)*) { $($e:tt)* } ) => {
            impl $(<$($a)*>)? $b for $c $($d)* { $($e)* }
            impl $(<$($a)*>)? $b for &$c $($d)* { $($e)* }
            impl $(<$($a)*>)? $b for &mut $c $($d)* { $($e)* }
        };
    }
}

// ========

use io::*;
mod io {
    use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Stdin, Stdout};

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
            #[allow(clippy::redundant_closure_for_method_calls)]
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
        #[track_caller]
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
            self.writer.flush().unwrap();
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

    use crate::impl_for_value_and_refs;

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

            impl $( < $($args)* > )? core::fmt::Debug for $ty {
                fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) ->  Result<(), core::fmt::Error> {
                    let mut buffer = Vec::new();
                    Writable::write(self, &mut buffer);
                    fmt.write_str(::core::str::from_utf8(&buffer).unwrap())
                }
            }
        };
    }

    impl_for_value_and_refs! {
        impl((T: Display, E: Display)) (Writable) for (Result<T, E>) () {
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
            #[allow(clippy::redundant_closure_for_method_calls)]
            self.bytes().position(|ch| ch.is_ascii_whitespace())
        }
    }

    impl StrExt for &[u8] {
        fn word_start_offset(self) -> Option<usize> {
            self.iter().position(|ch| !ch.is_ascii_whitespace())
        }

        fn word_end_offset(self) -> Option<usize> {
            #[allow(clippy::redundant_closure_for_method_calls)]
            self.iter().position(|ch| ch.is_ascii_whitespace())
        }
    }
}

// ========

use array_ext::*;
mod array_ext {
    pub fn array_from_fn<T, F, const N: usize>(mut cb: F) -> [T; N]
    where
        F: FnMut(usize) -> T,
    {
        let mut idx = 0;
        [(); N].map(|_| {
            let res = cb(idx);
            idx += 1;
            res
        })
    }

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

use tuple_ext::*;
mod tuple_ext {
    pub trait Tuple {}

    macro_rules! def {
        ( $($field:tt: $type:ident),* ) => {
            def!(@impl ());
            def!(@impl ()( $($field: $type),* ));
        };
        ( @impl ( $($field:tt: $type:ident),* ) ) => {
            impl< $($type),* > Tuple for ( $($type,)* ) {}
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

    pub trait First {
        type First;
        fn first(self) -> Self::First;
        fn first_ref(&self) -> &Self::First;
        fn first_mut(&mut self) -> &mut Self::First;
    }

    pub trait Second {
        type Second;
        fn second(self) -> Self::Second;
        fn second_ref(&self) -> &Self::Second;
        fn second_mut(&mut self) -> &mut Self::Second;
    }

    pub trait Swap {
        type Swapped;
        fn swap(self) -> Self::Swapped;
    }

    impl<T, U> First for (T, U) {
        type First = T;

        fn first(self) -> Self::First {
            self.0
        }

        fn first_ref(&self) -> &Self::First {
            &self.0
        }

        fn first_mut(&mut self) -> &mut Self::First {
            &mut self.0
        }
    }

    impl<T, U> Second for (T, U) {
        type Second = U;

        fn second(self) -> Self::Second {
            self.1
        }

        fn second_ref(&self) -> &Self::Second {
            &self.1
        }

        fn second_mut(&mut self) -> &mut Self::Second {
            &mut self.1
        }
    }

    impl<T, U> Swap for (T, U) {
        type Swapped = (U, T);

        fn swap(self) -> Self::Swapped {
            (self.1, self.0)
        }
    }
}

// ========

use bool_ext::*;
mod bool_ext {
    pub trait BoolExt {
        fn select<T>(self, t: T, f: T) -> T;
        fn select_with<T, F1: FnOnce() -> T, F2: FnOnce() -> T>(self, t: F1, f: F2) -> T;

        fn then_some<T>(self, some: T) -> Option<T>;

        fn as_result<T, E>(self, ok: T, err: E) -> Result<T, E>;
    }

    impl BoolExt for bool {
        fn select<T>(self, t: T, f: T) -> T {
            self.select_with(|| t, || f)
        }

        fn select_with<T, F1: FnOnce() -> T, F2: FnOnce() -> T>(self, t: F1, f: F2) -> T {
            if self {
                t()
            } else {
                f()
            }
        }

        fn then_some<T>(self, some: T) -> Option<T> {
            self.select(Some(some), None)
        }

        fn as_result<T, E>(self, ok: T, err: E) -> Result<T, E> {
            self.select(Ok(ok), Err(err))
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

use wort_start::*;
mod wort_start {
    use crate::{Readable, Reader};

    #[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct WordStart;

    #[allow(single_use_lifetimes)]
    impl<'a> Readable<'a> for WordStart {
        #[track_caller]
        fn read<R: Reader>(reader: &'a mut R) -> Self {
            reader.goto_word().unwrap();
            WordStart
        }
    }
}

// ========

use line_start::*;
mod line_start {
    use crate::{Readable, Reader};

    #[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct LineStart;

    #[allow(single_use_lifetimes)]
    impl<'a> Readable<'a> for LineStart {
        #[track_caller]
        fn read<R: Reader>(reader: &'a mut R) -> Self {
            reader.skip_line().unwrap();
            LineStart
        }
    }
}

// ========

use ch::*;
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

// ========

use word_ch::*;
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
            if reader.get_line().is_empty() {
                reader.skip_line().unwrap();
            }
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

use word_chs::*;
mod word_chs {
    use core::str::from_utf8;
    use std::io::Write;

    use crate::{def_wr_and_disp_by_ref, Readable, Reader, Writable};

    #[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct WordChs<const N: usize>(pub [u8; N]);

    #[allow(single_use_lifetimes)]
    impl<'a, const N: usize> Readable<'a> for WordChs<N> {
        #[track_caller]
        fn read<R: Reader>(reader: &'a mut R) -> Self {
            reader.goto_word().unwrap();
            let chs = reader
                .read_chars()
                .expect("target word length exceeds source line length");
            WordChs(chs)
        }
    }

    impl<const N: usize> Writable for &WordChs<N> {
        #[track_caller]
        fn write<W: Write>(self, writer: &mut W) {
            write!(writer, "{}", from_utf8(&self.0).unwrap()).unwrap();
        }
    }

    def_wr_and_disp_by_ref!((for const N: usize) WordChs<N>);
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
    impl<'a> Readable<'a> for Word<Vec<u8>> {
        #[track_caller]
        fn read<R: Reader>(reader: &'a mut R) -> Self {
            let word = SliceWord::read(reader);
            Word(word.0.to_vec())
        }
    }

    impl<'a> Readable<'a> for Word<&'a [u8]> {
        #[track_caller]
        fn read<R: Reader>(reader: &'a mut R) -> Self {
            reader.goto_word().unwrap();
            #[allow(clippy::redundant_closure_for_method_calls)]
            let word = reader.read_until(|line| line.word_end_offset()).unwrap();
            Word(word)
        }
    }

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

// ========

use gen_option::*;
mod gen_option {
    pub trait GenOption<T> {
        fn into_option(self) -> Option<T>;
    }

    impl<T> GenOption<T> for () {
        fn into_option(self) -> Option<T> {
            None
        }
    }

    impl<T> GenOption<T> for (T,) {
        fn into_option(self) -> Option<T> {
            Some(self.0)
        }
    }
}

// ========

use separated_tuple::*;
mod separated_tuple {
    use std::io::Write;

    use crate::{GenOption, Tuple, Writable};

    #[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct SeparatedTuple<Value, Pref, Sep, Suff> {
        value: Value,
        prefix: Pref,
        separator: Sep,
        suffix: Suff,
    }

    impl<'a, Pref, Sep, Suff> Writable for SeparatedTuple<(), Pref, Sep, Suff>
    where
        Pref: GenOption<&'a str>,
        Sep: GenOption<&'a str>,
        Suff: GenOption<&'a str>,
    {
        fn write<W: Write>(self, writer: &mut W) {
            if let Some(prefix) = self.prefix.into_option() {
                write!(writer, "{}", prefix).unwrap();
            }
            if let Some(suffix) = self.suffix.into_option() {
                write!(writer, "{}", suffix).unwrap();
            }
        }
    }

    impl<Value, Pref, Sep, Suff> SeparatedTuple<Value, Pref, Sep, Suff> {
        fn new(value: Value, prefix: Pref, separator: Sep, suffix: Suff) -> Self {
            Self {
                value,
                prefix,
                separator,
                suffix,
            }
        }
    }

    macro_rules! def {
        ( $($field:tt: $type:ident),* ) => {
            def!(@impl ()( $($field: $type),* ));
        };
        ( @impl ( $first_field:tt: $first_type:ident $(, $field:tt: $type:ident)* ) ) => {
            impl< 'a, $first_type, $( $type, )* Pref, Sep, Suff > Writable
                for SeparatedTuple< ( $first_type, $( $type, )* ), Pref, Sep, Suff >
            where
                $first_type: Writable,
                $( $type: Writable, )*
                Pref: GenOption<&'a str>,
                Sep: Copy + GenOption<&'a str>,
                Suff: GenOption<&'a str>,
            {
                fn write<W: Write>(self, writer: &mut W) {
                    if let Some(prefix) = self.prefix.into_option() {
                        write!(writer, "{}", prefix).unwrap();
                    }
                    self.value.$first_field.write(writer);
                    $(
                        if let Some(separator) = self.separator.into_option() {
                            write!(writer, "{}", separator).unwrap();
                        }
                        self.value.$field.write(writer);
                    )*
                    if let Some(suffix) = self.suffix.into_option() {
                        write!(writer, "{}", suffix).unwrap();
                    }
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

    pub trait TupleFmt: Sized {
        fn jo(self) -> SeparatedTuple<Self, (), (), ()>;
        fn sep(self, separator: &'static str) -> SeparatedTuple<Self, (), (&'static str,), ()>;
        fn fmt(
            self,
            prefix: &'static str,
            separator: &'static str,
            suffix: &'static str,
        ) -> SeparatedTuple<Self, (&'static str,), (&'static str,), (&'static str,)>;

        fn wo(self) -> SeparatedTuple<Self, (), (&'static str,), ()> {
            self.sep(" ")
        }
        fn li(self) -> SeparatedTuple<Self, (), (&'static str,), ()> {
            self.sep("\n")
        }
    }

    impl<T> TupleFmt for T
    where
        T: Tuple,
    {
        fn jo(self) -> SeparatedTuple<Self, (), (), ()> {
            SeparatedTuple::new(self, (), (), ())
        }

        fn sep(self, separator: &'static str) -> SeparatedTuple<Self, (), (&'static str,), ()> {
            SeparatedTuple::new(self, (), (separator,), ())
        }

        fn fmt(
            self,
            prefix: &'static str,
            separator: &'static str,
            suffix: &'static str,
        ) -> SeparatedTuple<Self, (&'static str,), (&'static str,), (&'static str,)> {
            SeparatedTuple::new(self, (prefix,), (separator,), (suffix,))
        }
    }
}

// ========

use separated_iterator::*;
mod separated_iterator {
    use std::io::Write;

    use crate::{GenOption, Writable};

    #[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct SeparatedIterator<Value, Pref, Sep, Suff> {
        value: Value,
        prefix: Pref,
        separator: Sep,
        suffix: Suff,
    }

    impl<Value, Pref, Sep, Suff> SeparatedIterator<Value, Pref, Sep, Suff> {
        fn new(value: Value, prefix: Pref, separator: Sep, suffix: Suff) -> Self {
            Self {
                value,
                prefix,
                separator,
                suffix,
            }
        }
    }

    impl<'a, I, Pref, Sep, Suff> Writable for SeparatedIterator<I, Pref, Sep, Suff>
    where
        I: IntoIterator,
        I::Item: Writable,
        Pref: GenOption<&'a str>,
        Sep: Copy + GenOption<&'a str>,
        Suff: GenOption<&'a str>,
    {
        fn write<W: Write>(self, writer: &mut W) {
            if let Some(prefix) = self.prefix.into_option() {
                write!(writer, "{}", prefix).unwrap();
            }
            let mut iter = self.value.into_iter();
            if let Some(first) = iter.next() {
                first.write(writer);
                for item in iter {
                    if let Some(separator) = self.separator.into_option() {
                        write!(writer, "{}", separator).unwrap();
                    }
                    item.write(writer);
                }
            }
            if let Some(suffix) = self.suffix.into_option() {
                write!(writer, "{}", suffix).unwrap();
            }
        }
    }

    pub trait IteratorFmt: Sized {
        fn jo(self) -> SeparatedIterator<Self, (), (), ()>;
        fn sep(self, separator: &'static str) -> SeparatedIterator<Self, (), (&'static str,), ()>;
        fn fmt(
            self,
            prefix: &'static str,
            separator: &'static str,
            suffix: &'static str,
        ) -> SeparatedIterator<Self, (&'static str,), (&'static str,), (&'static str,)>;

        fn wo(self) -> SeparatedIterator<Self, (), (&'static str,), ()> {
            self.sep(" ")
        }
        fn li(self) -> SeparatedIterator<Self, (), (&'static str,), ()> {
            self.sep("\n")
        }
    }

    impl<I> IteratorFmt for I
    where
        I: IntoIterator,
    {
        fn jo(self) -> SeparatedIterator<Self, (), (), ()> {
            SeparatedIterator::new(self, (), (), ())
        }

        fn sep(self, separator: &'static str) -> SeparatedIterator<Self, (), (&'static str,), ()> {
            SeparatedIterator::new(self, (), (separator,), ())
        }

        fn fmt(
            self,
            prefix: &'static str,
            separator: &'static str,
            suffix: &'static str,
        ) -> SeparatedIterator<Self, (&'static str,), (&'static str,), (&'static str,)> {
            SeparatedIterator::new(self, (prefix,), (separator,), (suffix,))
        }
    }
}

// ========

use values::*;
mod values {
    macro_rules! def {
        ( $name:ident, $fn:ident, $is:ident, $value:literal ) => {
            pub trait $name {
                fn $fn() -> Self;
                fn $is(&self) -> bool;
            }
            def!(
                @impl $name, $fn, $is, $value,
                u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
            );
        };
        ( @impl $name:ident, $fn:ident, $is:ident, $value:literal, $ty:ty $(, $($ts:ty),*)? ) => {
            impl $name for $ty {
                fn $fn() -> Self {
                    $value
                }

                fn $is(&self) -> bool {
                    *self == $value
                }
            }
            $( def!(@impl $name, $fn, $is, $value, $($ts),*); )?
        };
    }

    def!(Zero, zero, is_zero, 0);
    def!(One, one, is_one, 1);
    def!(Two, two, is_two, 2);
    def!(Three, three, is_three, 3);
    def!(Five, five, is_five, 5);
    def!(Ten, ten, is_ten, 10);
}

// ========

use ops::*;
mod ops {
    macro_rules! def {
        ((
            $trait:ident $( ($($bounds:tt)*) )?,
            fn $fn:ident($($self:ident $(, $arg:ident: $arg_ty:ty)*)?) -> $ret:ty {
                u: $uexpr:expr, i: $iexpr:expr
            }
        )) => {
            pub trait $trait $( where $($bounds)* )? {
                fn $fn($($self $(, $arg: $arg_ty)*)?) -> $ret;
            }
            def!(@impl $trait, fn $fn($($self $(, $arg: $arg_ty)*)?) -> $ret {
                u8: $uexpr, u16: $uexpr, u32: $uexpr, u64: $uexpr, u128: $uexpr, usize: $uexpr,
                i8: $iexpr, i16: $iexpr, i32: $iexpr, i64: $iexpr, i128: $iexpr, isize: $iexpr
            });
        };
        ((
            $trait:ident $( ($($bounds:tt)*) )?,
            fn $fn:ident($($self:ident $(, $arg:ident: $arg_ty:ty)*)?) -> $ret:ty { $expr:expr }
        )) => {
            pub trait $trait $( where $($bounds)* )? {
                fn $fn($($self $(, $arg: $arg_ty)*)?) -> $ret;
            }
            def!(@impl $trait, fn $fn($($self $(, $arg: $arg_ty)*)?) -> $ret {
                u8: $expr, u16: $expr, u32: $expr, u64: $expr, u128: $expr, usize: $expr,
                i8: $expr, i16: $expr, i32: $expr, i64: $expr, i128: $expr, isize: $expr
            });
        };
        (@impl $trait:ident, fn $fn:ident($($tt:tt)*) -> $ret:ty {}) => {};
        (
            @impl $trait:ident,
            fn $fn:ident($($self:ident $(, $arg: ident: $arg_ty: ty)*)?) -> $ret:ty {
                $ty:tt: $expr:expr $(, $ts:tt: $es:expr)*
            }
        ) => {
            impl $trait for $ty {
                fn $fn($($self $(, $arg: $arg_ty)*)?) -> $ret {
                    $expr
                }
            }
            def!(@impl $trait, fn $fn($($self $(, $arg: $arg_ty)*)?) -> $ret { $($ts: $es),* });
        };
    }

    def!((Min, fn min() -> Self { Self::MIN } ));
    def!((Max, fn max() -> Self { Self::MAX } ));
    def!((Abs, fn abs(self) -> Self { u: self, i: self.abs() } ));
    def!((TrailingZeros, fn trailing_zeros(self) -> u32 { self.trailing_zeros() } ));
    def!((DivEuclid, fn div_euclid(self, rhs: Self) -> Self { self.div_euclid(rhs) } ));
    def!((RemEuclid, fn rem_euclid(self, rhs: Self) -> Self { self.rem_euclid(rhs) } ));
    def!((CheckedMul(Self: Sized), fn checked_mul(self, rhs: Self) -> Option<Self> {
        self.checked_mul(rhs)
    } ));
}

// ========

use mul_div::*;
mod mul_div {
    pub trait MulDiv {
        fn mul_div(self, mul: Self, div: Self) -> Self;
    }

    macro_rules! def {
        ( $low:ty, $hi:ty $(, $rest:ty)* ) => {
            impl MulDiv for $low {
                fn mul_div(self, mul: Self, div: Self) -> Self {
                    ((self as $hi) * (mul as $hi) / (div as $hi)) as $low
                }
            }
            def!($hi $(, $rest)*);
        };
        ( $last:ty ) => {};
    }

    def!(u8, u16, u32, u64, u128);
    def!(i8, i16, i32, i64, i128);
}

// ========

use mul_rem::*;
mod mul_rem {
    pub trait MulRem {
        fn mul_rem(self, mul: Self, rem: Self) -> Self;
    }

    macro_rules! def {
        ( $low:ty, $hi:ty $(, $rest:ty)* ) => {
            impl MulRem for $low {
                fn mul_rem(self, mul: Self, rem: Self) -> Self {
                    ((self as $hi) * (mul as $hi) % (rem as $hi)) as $low
                }
            }
            def!($hi $(, $rest)*);
        };
        ( $last:ty ) => {};
    }

    def!(u8, u16, u32, u64, u128);
    def!(i8, i16, i32, i64, i128);
}

// ========

use gcd::*;
mod gcd {
    use core::ops::Rem;

    use crate::{Abs, Zero};

    // https://en.wikipedia.org/wiki/Euclidean_algorithm
    // gcd(a, b) = gcd(a - b, a): gcd(14, 10) = gcd(4, 10)
    pub fn gcd<T>(lhs: T, rhs: T) -> T
    where
        T: Abs + Clone + Rem<Output = T> + Zero,
    {
        if rhs.is_zero() {
            lhs.abs()
        } else {
            gcd(rhs.clone(), lhs % rhs)
        }
    }
}

// ========

use lcm::*;
mod lcm {
    use core::ops::{Div, Mul, Rem};

    use crate::{gcd, Abs, Zero};

    pub fn lcm<T>(lhs: T, rhs: T) -> T
    where
        T: Abs + Clone + Div<Output = T> + Mul<Output = T> + Rem<Output = T> + Zero,
    {
        lhs.clone() * rhs.clone() / gcd(lhs, rhs)
    }
}

// ========

use into_range::*;
mod into_range {
    use core::ops::RangeBounds;

    pub trait IsInRange: Sized {
        fn into_range<R: RangeBounds<Self>>(self, range: R) -> Option<Self>;
    }

    impl<T> IsInRange for T
    where
        T: Copy + Ord + PartialOrd,
    {
        fn into_range<R: RangeBounds<Self>>(self, range: R) -> Option<Self> {
            if range.contains(&self) {
                Some(self)
            } else {
                None
            }
        }
    }
}

// ========

use log2::*;
mod log2 {
    pub trait Log2
    where
        Self: Sized,
    {
        fn log2_floor(self) -> u32;
        fn log2_ceil(self) -> u32;
        fn round_log2_floor(self) -> Self;
        fn round_log2_ceil(self) -> Self;
    }

    macro_rules! def {
        ( $($type:ty),* ) => {
            $(
                impl Log2 for $type {
                    fn log2_floor(self) -> u32 {
                        use core::mem::size_of;
                        assert!(self > 0);
                        (size_of::<Self>() * 8) as u32 - self.leading_zeros() - 1
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

// ========

use log10::*;
mod log10 {
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

// ========

use min_max::*;
mod min_max {
    use core::cmp::{max, min, Ordering};

    pub fn min_by<T, F: FnOnce(&T, &T) -> Ordering>(lhs: T, rhs: T, compare: F) -> T {
        match compare(&lhs, &rhs) {
            Ordering::Less | Ordering::Equal => lhs,
            Ordering::Greater => rhs,
        }
    }

    pub fn max_by<T, F: FnOnce(&T, &T) -> Ordering>(lhs: T, rhs: T, compare: F) -> T {
        match compare(&lhs, &rhs) {
            Ordering::Less | Ordering::Equal => rhs,
            Ordering::Greater => lhs,
        }
    }

    pub fn min_max_by<T, F: FnOnce(&T, &T) -> Ordering>(lhs: T, rhs: T, compare: F) -> (T, T) {
        match compare(&lhs, &rhs) {
            Ordering::Less | Ordering::Equal => (lhs, rhs),
            Ordering::Greater => (rhs, lhs),
        }
    }

    pub fn min_by_key<T, F: FnMut(&T) -> K, K: Ord>(v1: T, v2: T, mut f: F) -> T {
        min_by(v1, v2, |v1, v2| f(v1).cmp(&f(v2)))
    }

    pub fn max_by_key<T, F: FnMut(&T) -> K, K: Ord>(v1: T, v2: T, mut f: F) -> T {
        max_by(v1, v2, |v1, v2| f(v1).cmp(&f(v2)))
    }

    pub fn min_max_by_key<T, F: FnMut(&T) -> K, K: Ord>(v1: T, v2: T, mut f: F) -> (T, T) {
        min_max_by(v1, v2, |v1, v2| f(v1).cmp(&f(v2)))
    }

    pub trait IteratorMinMax {
        type Item;

        fn min_max(self) -> Option<(Self::Item, Self::Item)>
        where
            Self::Item: Ord;

        fn min_max_by<F>(self, compare: F) -> Option<(Self::Item, Self::Item)>
        where
            F: Clone + FnMut(&Self::Item, &Self::Item) -> Ordering;

        fn min_max_by_key<B, F>(self, f: F) -> Option<(Self::Item, Self::Item)>
        where
            B: Ord,
            F: Clone + FnMut(&Self::Item) -> B;
    }

    impl<I> IteratorMinMax for I
    where
        I: IntoIterator,
        I::Item: Clone,
    {
        type Item = I::Item;

        fn min_max(self) -> Option<(Self::Item, Self::Item)>
        where
            Self::Item: Ord,
        {
            self.into_iter().fold(None, |acc, value| match acc {
                Some(acc) => Some((min(acc.0, value.clone()), max(acc.1, value))),
                None => Some((value.clone(), value)),
            })
        }

        fn min_max_by<F>(self, compare: F) -> Option<(Self::Item, Self::Item)>
        where
            F: Clone + FnMut(&Self::Item, &Self::Item) -> Ordering,
        {
            self.into_iter().fold(None, |acc, value| match acc {
                Some(acc) => Some((
                    min_by(acc.0, value.clone(), compare.clone()),
                    max_by(acc.1, value, compare.clone()),
                )),
                None => Some((value.clone(), value)),
            })
        }

        fn min_max_by_key<B, F>(self, f: F) -> Option<(Self::Item, Self::Item)>
        where
            B: Ord,
            F: Clone + FnMut(&Self::Item) -> B,
        {
            self.into_iter().fold(None, |acc, value| match acc {
                Some(acc) => Some((
                    min_by_key(acc.0, value.clone(), f.clone()),
                    max_by_key(acc.1, value, f.clone()),
                )),
                None => Some((value.clone(), value)),
            })
        }
    }
}

// ========

use unsigned::*;
mod unsigned {
    pub trait Unsigned {}

    macro_rules! def {
        ( $($ty:ty),* ) => {
            $( impl Unsigned for $ty {} )*
        };
    }
    def!(u8, u16, u32, u64, u128, usize);
}

// ========

use signed::*;
mod signed {
    pub trait Signed {}

    macro_rules! def {
        ( $($ty:ty),* ) => {
            $( impl Signed for $ty {} )*
        };
    }
    def!(i8, i16, i32, i64, i128, isize);
}

// ========

use wrap::*;
mod wrap {
    use core::num::Wrapping;

    pub fn wrap<T>(value: T) -> Wrapping<T> {
        Wrapping(value)
    }
}

// ========

use option_ext::*;
mod option_ext {
    pub trait OptionExt<T> {
        fn omin(self, other: T) -> T;
        fn omax(self, other: T) -> T;
    }

    impl<T: Ord> OptionExt<T> for Option<T> {
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

    impl<T: Ord> OptionExt<Option<T>> for Option<T> {
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

// ========

use vec_ext::*;
mod vec_ext {
    pub trait VecExt<T> {
        fn wc(capacity: usize) -> Self;
        fn set_or_push(&mut self, j: usize, value: T);
    }

    impl<T> VecExt<T> for Vec<T> {
        fn wc(capacity: usize) -> Self {
            Self::with_capacity(capacity)
        }

        fn set_or_push(&mut self, j: usize, value: T) {
            if j == self.len() {
                self.push(value);
            } else {
                self[j] = value;
            }
        }
    }
}

// ========

use into_vec::*;
mod into_vec {
    pub trait IntoVec<T> {
        fn into_vec(self) -> Vec<T>;
    }

    impl<I, T> IntoVec<T> for I
    where
        Self: Sized,
        I: IntoIterator<Item = T>,
    {
        fn into_vec(self) -> Vec<T> {
            self.into_iter().collect()
        }
    }
}

// ========

use to_iterator::*;
mod to_iterator {
    pub trait ToIterator: Clone + IntoIterator {
        fn to_iter(&self) -> Self::IntoIter;
    }

    impl<T> ToIterator for T
    where
        T: Clone + IntoIterator,
    {
        fn to_iter(&self) -> Self::IntoIter {
            self.clone().into_iter()
        }
    }
}

// ========

use slice_from_iterator::*;
mod slice_from_iterator {
    pub trait SliceFromIterator {
        type Item;
        fn from_iter<I: Iterator<Item = Self::Item>>(&mut self, iter: I) -> usize;
    }

    impl<T> SliceFromIterator for [T] {
        type Item = T;
        fn from_iter<I: Iterator<Item = Self::Item>>(&mut self, iter: I) -> usize {
            let mut len = 0;
            for value in iter {
                assert!(len < self.len());
                self[len] = value;
                len += 1;
            }
            len
        }
    }
}

// ========

use sortable::*;
mod sortable {
    use core::cmp::Ordering;

    pub trait Sortable {
        type Item;

        fn sort_rev(&mut self)
        where
            Self::Item: Ord;

        fn sort_unstable_rev(&mut self)
        where
            Self::Item: Ord;

        fn insertion_sort(&mut self)
        where
            Self::Item: Ord;

        fn insertion_sort_by<F>(&mut self, compare: F)
        where
            F: FnMut(&Self::Item, &Self::Item) -> Ordering;

        fn insertion_sort_by_key<K, F>(&mut self, f: F)
        where
            F: FnMut(&Self::Item) -> K,
            K: Ord;

        fn insertion_sort_rev(&mut self)
        where
            Self::Item: Ord,
        {
            self.insertion_sort_by(|lhs, rhs| rhs.cmp(lhs))
        }
    }

    impl<T> Sortable for [T]
    where
        T: Copy,
    {
        type Item = T;

        fn sort_rev(&mut self)
        where
            Self::Item: Ord,
        {
            self.sort_by(|lhs, rhs| rhs.cmp(lhs))
        }

        fn sort_unstable_rev(&mut self)
        where
            Self::Item: Ord,
        {
            self.sort_unstable_by(|lhs, rhs| rhs.cmp(lhs))
        }

        // works faster on an array with a length of less than 8 elements
        fn insertion_sort(&mut self)
        where
            Self::Item: Ord,
        {
            self.insertion_sort_by(|lhs, rhs| lhs.cmp(rhs));
        }

        fn insertion_sort_by<F>(&mut self, mut compare: F)
        where
            F: FnMut(&Self::Item, &Self::Item) -> Ordering,
        {
            for i in 1..self.len() {
                let key = self[i];
                let mut j = i - 1;
                if compare(&self[j], &key) == Ordering::Greater {
                    self[i] = self[j];
                    while j > 0 && compare(&self[j - 1], &key) == Ordering::Greater {
                        self[j] = self[j - 1];
                        j -= 1;
                    }
                    self[j] = key;
                }
            }
        }

        fn insertion_sort_by_key<K, F>(&mut self, mut f: F)
        where
            F: FnMut(&Self::Item) -> K,
            K: Ord,
        {
            self.insertion_sort_by(|lhs, rhs| f(lhs).cmp(&f(rhs)))
        }
    }
}

// ========

use sums::*;
mod sums {
    use core::iter::FusedIterator;
    use core::ops::AddAssign;

    use crate::Zero;

    pub trait Sums {
        type Output;
        fn sums(self) -> Self::Output;
    }

    #[derive(Clone, Debug)]
    pub struct SumsIter<T, I>(T, I);

    impl<T, I> SumsIter<T, I>
    where
        T: Zero,
        I: Iterator<Item = T>,
    {
        pub fn new(iter: I) -> Self {
            Self(T::zero(), iter)
        }
    }

    impl<T, I> Sums for I
    where
        T: Zero,
        I: IntoIterator<Item = T>,
    {
        type Output = SumsIter<T, I::IntoIter>;
        fn sums(self) -> Self::Output {
            SumsIter::new(self.into_iter())
        }
    }

    impl<T, I> Iterator for SumsIter<T, I>
    where
        T: AddAssign + Clone,
        I: Iterator<Item = T>,
    {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            if let Some(next) = self.1.next() {
                self.0 += next;
                Some(self.0.clone())
            } else {
                None
            }
        }
    }

    impl<T, I> FusedIterator for SumsIter<T, I>
    where
        T: AddAssign + Clone,
        I: Iterator<Item = T>,
    {
    }
}

// ========

use prods::*;
mod prods {
    use core::iter::FusedIterator;
    use core::ops::MulAssign;

    use crate::One;

    pub trait Prods {
        type Output;
        fn prods(self) -> Self::Output;
    }

    #[derive(Clone, Debug)]
    pub struct ProdsIter<T, I>(T, I);

    impl<T, I> ProdsIter<T, I>
    where
        T: One,
        I: Iterator<Item = T>,
    {
        pub fn new(iter: I) -> Self {
            Self(T::one(), iter)
        }
    }

    impl<T, I> Prods for I
    where
        T: One,
        I: IntoIterator<Item = T>,
    {
        type Output = ProdsIter<T, I::IntoIter>;
        fn prods(self) -> Self::Output {
            ProdsIter::new(self.into_iter())
        }
    }

    impl<T, I> Iterator for ProdsIter<T, I>
    where
        T: MulAssign + Clone,
        I: Iterator<Item = T>,
    {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            if let Some(next) = self.1.next() {
                self.0 *= next;
                Some(self.0.clone())
            } else {
                None
            }
        }
    }

    impl<T, I> FusedIterator for ProdsIter<T, I>
    where
        T: MulAssign + Clone,
        I: Iterator<Item = T>,
    {
    }
}

// ========

use collections::*;
mod collections {
    #[macro_export]
    macro_rules! replace_expr {
        ($_t:tt $sub:expr) => {
            $sub
        };
    }

    #[macro_export]
    macro_rules! bts {
        () => {
            std::collections::BTreeSet::new()
        };
        ($($x:expr),+ $(,)?) => {{
            let mut collection = std::collections::BTreeSet::new();
            $( let _ = collection.insert($x); )+
            collection
        }};
    }

    #[macro_export]
    macro_rules! btm {
        () => {
            std::collections::BTreeMap::new()
        };
        ($(($k:expr, $v:expr)),+ $(,)?) => {{
            let mut collection = std::collections::BTreeMap::new();
            $( let _ = collection.insert($k, $v); )+
            collection
        }};
    }

    #[macro_export]
    macro_rules! hs {
        () => {
            std::collections::HashSet::new()
        };
        ($($x:expr),+ $(,)?) => {{
            let mut collection = std::collections::HashSet::with_capacity([$(crate::replace_expr!($x ())),+].len());
            $( let _ = collection.insert($x); )+
            collection
        }};
    }

    #[macro_export]
    macro_rules! hm {
        () => {
            std::collections::HashMap::new()
        };
        ($(($k:expr, $v:expr)),+ $(,)?) => {{
            let mut collection = std::collections::HashMap::with_capacity([$(crate::replace_expr!($v ())),+].len());
            $( let _ = collection.insert($k, $v); )+
            collection
        }};
    }
}

// ========

use ceil::*;
mod ceil {
    use core::ops::{Add, Div, Sub};

    use crate::{DivEuclid, One};

    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct Ceil<T>(pub T);

    impl<T> Div<T> for Ceil<T>
    where
        T: Add<Output = T> + Clone + DivEuclid + One + Sub<Output = T>,
    {
        type Output = T;

        fn div(self, rhs: T) -> Self::Output {
            (self.0 + rhs.clone() - T::one()).div_euclid(rhs)
        }
    }

    pub fn ceil<T>(value: T) -> Ceil<T> {
        Ceil(value)
    }
}

// ========

use binomial::*;
mod binomial {
    use core::cmp::PartialOrd;
    use core::ops::{Add, AddAssign, Div, Mul, Sub};
    use std::cmp::min;
    use std::iter::successors;

    use crate::{Five, MulDiv, One, Ten, Three, Two, Zero};

    pub fn binomial<T>(n: T, k: T) -> T
    where
        T: Add<Output = T>
            + AddAssign
            + Copy
            + MulDiv
            + One
            + Ord
            + PartialOrd
            + Sub<Output = T>
            + Zero,
    {
        *Binomial::with(n, k).get()
    }

    // (n | k) = n! / (k! * (n - k)!)
    // (n | 0) = (n | n) = 1
    // (n - 1 | k) = (n | k) * (n - k) / n
    // (n + 1 | k) = (n | k) * (n + 1) / (n + 1 - k)
    // (n | k - 1) = (n | k) * k / (n + 1 - k)
    // (n | k + 1) = (n | k) * (n - k) / (k + 1)
    // n\k  0  1  2  3  4  5
    // 0    1
    // 1    1  1
    // 2    1  2  1
    // 3    1  3  3  1
    // 4    1  4  6  4  1
    // 5    1  5 10 10  5  1
    #[derive(Clone, Copy, Debug)]
    pub struct Binomial<T> {
        value: T,
        n: T,
        k: T,
    }

    impl<T> Binomial<T> {
        pub fn get(&self) -> &T {
            &self.value
        }

        pub fn n(&self) -> &T {
            &self.n
        }

        pub fn k(&self) -> &T {
            &self.k
        }
    }

    impl<T> Binomial<T>
    where
        T: One + Copy + Zero,
    {
        pub fn new() -> Self {
            Self {
                value: T::one(),
                n: T::zero(),
                k: T::zero(),
            }
        }

        pub fn with_n(n: T) -> Self {
            Self {
                value: T::one(),
                n,
                k: T::zero(),
            }
        }

        pub fn with_nk(nk: T) -> Self {
            Self {
                value: T::one(),
                n: nk,
                k: nk,
            }
        }
    }

    impl<T> Binomial<T>
    where
        T: Add<Output = T> + AddAssign + Copy + MulDiv + One + PartialOrd + Sub<Output = T> + Zero,
    {
        pub fn with(n: T, k: T) -> Self {
            if k > n {
                Self {
                    value: T::zero(),
                    n,
                    k,
                }
            } else if k <= n - k {
                let mut coeff = Self::with_n(n);
                let mut p = T::zero();
                while p < k {
                    coeff = coeff.inc_k();
                    p += T::one();
                }
                coeff
            } else {
                let mut coeff = Self::with_nk(k);
                let mut p = k;
                while p < n {
                    coeff = coeff.inc_n();
                    p += T::one();
                }
                coeff
            }
        }
    }

    impl<T> Binomial<T>
    where
        T: Copy + Sub<Output = T> + Ord,
    {
        pub fn left(&self) -> Self {
            Self {
                value: self.value,
                n: self.n,
                k: min(self.k, self.n - self.k),
            }
        }
    }

    impl<T> Binomial<T>
    where
        T: Add<Output = T> + Copy + MulDiv + One + Sub<Output = T> + Zero,
    {
        pub fn line(n: T) -> impl Iterator<Item = Binomial<T>> {
            successors(Some(Self::with_n(n)), |coeff| Some(coeff.inc_k()))
        }

        // 0:  1  1  1  1  1
        // 1:  1  2  3  4  5
        // 2:  1  3  6 10 15 triangle number
        // 2:  1  4 10 20 35 tetrahedral numbers
        pub fn column(nk: T) -> impl Iterator<Item = Binomial<T>> {
            successors(Some(Self::with_nk(nk)), |coeff| Some(coeff.inc_n()))
        }

        pub fn diag(n: T) -> impl Iterator<Item = Binomial<T>> {
            successors(Some(Self::with_n(n)), |coeff| Some(coeff.inc_nk()))
        }
    }

    impl<T> Binomial<T>
    where
        T: Add<Output = T> + Copy + MulDiv + One + Sub<Output = T>,
    {
        pub fn dec_n(&self) -> Binomial<T> {
            Self {
                value: self.value.mul_div(self.n - self.k, self.n),
                n: self.n - T::one(),
                k: self.k,
            }
        }

        pub fn inc_n(&self) -> Binomial<T> {
            Self {
                value: self
                    .value
                    .mul_div(self.n + T::one(), self.n + T::one() - self.k),
                n: self.n + T::one(),
                k: self.k,
            }
        }

        pub fn dec_nk(&self) -> Binomial<T> {
            Self {
                value: self.value.mul_div(self.k, self.n),
                n: self.n - T::one(),
                k: self.k - T::one(),
            }
        }

        pub fn inc_nk(&self) -> Binomial<T> {
            Self {
                value: self.value.mul_div(self.n + T::one(), self.k + T::one()),
                n: self.n + T::one(),
                k: self.k + T::one(),
            }
        }

        pub fn dec_k(&self) -> Binomial<T> {
            Self {
                value: self.value.mul_div(self.k, self.n + T::one() - self.k),
                n: self.n,
                k: self.k - T::one(),
            }
        }

        pub fn inc_k(&self) -> Binomial<T> {
            Self {
                value: self.value.mul_div(self.n - self.k, self.k + T::one()),
                n: self.n,
                k: self.k + T::one(),
            }
        }
    }
}

// ========

use factorial::*;
mod factorial {
    pub trait Factorial {
        fn factorial(value: usize) -> Self;
    }

    macro_rules! def {
        ( $ty:ty, [$($list:tt)*] ) => {
            def!(@impl $ty, 0, [$($list)*,], [], [
                1,
                1,
                2,
                6,
                24,
                120,
                720,
                5_040,
                40_320,
                362_880,
                3_628_800,
                39_916_800,
                479_001_600,
                6_227_020_800,
                87_178_291_200,
                1_307_674_368_000,
                20_922_789_888_000,
                355_687_428_096_000,
                6_402_373_705_728_000,
                121_645_100_408_832_000,
                2_432_902_008_176_640_000,
                51_090_942_171_709_440_000,
                1_124_000_727_777_607_680_000,
                25_852_016_738_884_976_640_000,
                620_448_401_733_239_439_360_000,
                15_511_210_043_330_985_984_000_000,
                403_291_461_126_605_635_584_000_000,
                10_888_869_450_418_352_160_768_000_000,
                304_888_344_611_713_860_501_504_000_000,
                8_841_761_993_739_701_954_543_616_000_000,
                265_252_859_812_191_058_636_308_480_000_000,
                8_222_838_654_177_922_817_725_562_880_000_000,
                263_130_836_933_693_530_167_218_012_160_000_000,
                8_683_317_618_811_886_495_518_194_401_280_000_000,
                295_232_799_039_604_140_847_618_609_643_520_000_000
            ]);
        };
        (
            @impl $ty:ty, $len:expr,
            [$item:tt $($list:tt)*],
            [$($result:literal),*],
            [$value:literal $(, $values:literal)*]
        ) => {
            def!(@impl $ty, $len + 1, [$($list)*], [$($result,)* $value], [$($values),*]);
        };
        (
            @impl $ty:ty, $len:expr,
            [],
            [$($result:literal),*],
            [$($values:literal),*]
        ) => {
            impl Factorial for $ty {
                fn factorial(value: usize) -> Self {
                    let result: [$ty; $len] = [$($result),*];
                    result[value]
                }
            }
        };
    }

    def!(i32, [,,,,,,,,,,,,]);
    def!(u32, [,,,,,,,,,,,,]);
    def!(i64, [,,,,,,,,,,,,,,,,,,,,]);
    def!(u64, [,,,,,,,,,,,,,,,,,,,,]);
    def!(i128, [,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,]);
    def!(u128, [,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,]);
}

// ========

use factorials::*;
mod factorials {
    use core::ops::{AddAssign, MulAssign};

    use crate::One;

    #[derive(Clone, Debug)]
    pub struct Factorials<T> {
        data: Vec<T>,
        mult1: T,
        mult2: T,
    }

    impl<T: One> Factorials<T> {
        pub fn new() -> Self {
            Self {
                data: vec![T::one()],
                mult1: T::one(),
                mult2: T::one(),
            }
        }
    }

    impl<T: One> Factorials<T> {
        pub fn get(&mut self, n: usize) -> T
        where
            T: Copy + MulAssign + AddAssign + One,
        {
            while n >= self.data.len() {
                self.mult2 *= self.mult1;
                self.mult1 += T::one();
                self.data.push(self.mult2);
            }
            self.data[n]
        }
    }
}

// ========

use factorize::*;
mod factorize {
    use core::ops::{AddAssign, BitAnd, DivAssign, Mul, Rem, ShrAssign};

    use crate::{Five, One, Three, TrailingZeros, Two, Zero};

    pub fn factorize_to_vec<T: Factorizable>(value: T) -> Vec<(T, usize)> {
        let mut factorization = Vec::new();
        factorize(value, |pair| factorization.push(pair));
        factorization
    }

    pub fn count_multipliers<T: Factorizable>(value: T) -> usize {
        let mut count = 0;
        factorize(value, |pair| count += pair.1);
        count
    }

    pub fn count_multiplier_primes<T: Factorizable>(value: T) -> usize {
        let mut count = 0;
        factorize(value, |_| count += 1);
        count
    }

    pub trait Factorizable:
        AddAssign
        + BitAnd<Output = Self>
        + Copy
        + DivAssign
        + Five
        + Mul<Output = Self>
        + One
        + PartialEq
        + PartialOrd
        + Rem<Output = Self>
        + ShrAssign<u32>
        + Three
        + TrailingZeros
        + Two
        + Zero
    {
    }

    impl<T> Factorizable for T where
        T: AddAssign
            + BitAnd<Output = T>
            + Copy
            + DivAssign
            + Five
            + Mul<Output = T>
            + One
            + PartialEq
            + PartialOrd
            + Rem<Output = T>
            + ShrAssign<u32>
            + Three
            + TrailingZeros
            + Two
            + Zero
    {
    }

    pub fn factorize<T: Factorizable, F: FnMut((T, usize))>(mut value: T, mut func: F) {
        if (value & T::one()).is_zero() {
            let pow2 = value.trailing_zeros();
            value >>= pow2;
            func((T::two(), pow2 as usize));
        }

        {
            let mut pow3 = 0;
            while (value % T::three()).is_zero() {
                pow3 += 1;
                value /= T::three();
            }
            if pow3 > 0 {
                func((T::three(), pow3));
            }
        }

        let mut divisor: T = T::five();
        while divisor * divisor <= value {
            for _ in 0..2 {
                let mut pow = 0;
                while (value % divisor).is_zero() {
                    pow += 1;
                    value /= divisor;
                }
                if pow > 0 {
                    func((divisor, pow));
                }
                divisor += T::two();
            }
            divisor += T::two();
        }

        if !value.is_one() {
            func((value, 1));
        }
    }
}

// ========

use primes::*;
mod primes {
    use core::iter::FusedIterator;

    use crate::DedupCount;

    #[derive(Clone, Debug)]
    pub struct Primes {
        sieve: Vec<usize>,
        len: usize,
    }

    impl Primes {
        pub fn new(len: usize) -> Self {
            let odds_len = len / 2;
            let mut sieve = vec![0; odds_len];
            let half = (len as f64).sqrt().ceil() as usize;
            for j in (3..half).step_by(2) {
                if sieve[j / 2] == 0 {
                    for k in (j * j..len).step_by(2 * j) {
                        if sieve[k / 2] == 0 {
                            sieve[k / 2] = j;
                        }
                    }
                }
            }
            Self { sieve, len }
        }

        pub fn odds_sieve(&self) -> &[usize] {
            &self.sieve
        }

        pub fn get_sieve_value(&self, value: usize) -> usize {
            if value < 4 {
                0
            } else if value & 1 == 0 {
                2
            } else {
                self.sieve[value / 2]
            }
        }

        pub fn is_prime(&self, value: usize) -> bool {
            if value < 3 {
                value == 2
            } else {
                value & 1 == 1 && self.sieve[value / 2] == 0
            }
        }

        pub fn iter(&self) -> PrimesIter<'_> {
            PrimesIter::new(self, 0)
        }

        pub fn iter_from(&self, from: usize) -> PrimesIter<'_> {
            PrimesIter::new(self, from)
        }

        // f(7) = [ 7 ]
        // f(8) = [ 2 2 2 ]
        // f(9) = [ 3 3 ]
        pub fn factorize(&self, value: usize) -> PrimesFactorizeIter<'_> {
            assert!(value < self.len);
            PrimesFactorizeIter::new(self, value)
        }

        // f( 7) = 2: ( N[7] + 1 )
        // f( 8) = 4: ( N[2 2 2] + 1 )
        // f(12) = 6: ( N[2 2] + 1 ) * ( N[3] + 1 )
        pub fn num_divisors(&self, value: usize) -> usize {
            let factorization = self.factorize(value);
            factorization
                .dedup_count()
                .map(|(_, count)| count + 1)
                .product()
        }

        // num of coprime with n before n
        // f(7) = 6 : 1 2 3 4 5 6
        // f(8) = 4 : 1 - 3 - 5 - 7
        // f(9) = 6 : 1 2 - 4 5 - 7 8
        pub fn eulers_phi(&self, value: usize) -> usize {
            if value == 1 {
                return 0;
            }
            let factorization = self.factorize(value);
            factorization.dedup_count().fold(value, |mult, (prime, _)| {
                ((mult as u64) * (prime - 1) as u64 / prime as u64) as usize
            })
        }
    }

    #[derive(Clone, Debug)]
    pub struct PrimesIter<'a>(&'a Primes, usize);

    impl<'a> PrimesIter<'a> {
        pub fn new(sieve: &'a Primes, from: usize) -> Self {
            Self(sieve, if from < 3 { 0 } else { from / 2 })
        }
    }

    impl Iterator for PrimesIter<'_> {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.1 == 0 {
                if self.0.len > 2 {
                    self.1 = 1;
                    Some(2)
                } else {
                    None
                }
            } else {
                while self.1 < self.0.sieve.len() {
                    if self.0.sieve[self.1] != 0 {
                        self.1 += 1;
                    } else {
                        let item = self.1;
                        self.1 += 1;
                        return Some(item * 2 + 1);
                    }
                }
                None
            }
        }
    }

    impl FusedIterator for PrimesIter<'_> {}

    #[derive(Clone, Debug)]
    pub struct PrimesFactorizeIter<'a>(&'a Primes, usize);

    impl<'a> PrimesFactorizeIter<'a> {
        pub fn new(sieve: &'a Primes, value: usize) -> Self {
            Self(sieve, value)
        }
    }

    impl Iterator for PrimesFactorizeIter<'_> {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.1 > 1 {
                if self.1 & 1 == 0 {
                    self.1 /= 2;
                    Some(2)
                } else {
                    let divisor = self.0.sieve[self.1 / 2];
                    if divisor != 0 {
                        self.1 /= divisor;
                        Some(divisor)
                    } else {
                        let item = self.1;
                        self.1 = 1;
                        Some(item)
                    }
                }
            } else {
                None
            }
        }
    }

    impl FusedIterator for PrimesFactorizeIter<'_> {}
}

// ========

use dedup_count::*;
mod dedup_count {
    use core::iter::FusedIterator;

    #[derive(Clone, Debug)]
    pub struct DedupCountIter<T, I> {
        iter: I,
        prev: Option<(T, usize)>,
    }

    impl<T, I> DedupCountIter<T, I> {
        pub fn new(iter: I) -> Self {
            Self { iter, prev: None }
        }
    }

    impl<T, I> Iterator for DedupCountIter<T, I>
    where
        T: Eq,
        I: Iterator<Item = T>,
    {
        type Item = (T, usize);

        fn next(&mut self) -> Option<Self::Item> {
            loop {
                match (self.prev.take(), self.iter.next()) {
                    (Some((prev, count)), Some(item)) => {
                        if prev == item {
                            self.prev = Some((prev, count + 1));
                        } else {
                            self.prev = Some((item, 1));
                            return Some((prev, count));
                        }
                    }
                    (Some((prev, count)), None) => {
                        return Some((prev, count));
                    }
                    (None, Some(item)) => self.prev = Some((item, 1)),
                    (None, None) => {
                        return None;
                    }
                }
            }
        }
    }

    impl<T, I> FusedIterator for DedupCountIter<T, I>
    where
        T: Eq,
        I: Iterator<Item = T>,
    {
    }

    pub trait DedupCount {
        type Output;
        fn dedup_count(self) -> Self::Output;
    }

    impl<T, I> DedupCount for I
    where
        I: IntoIterator<Item = T>,
    {
        type Output = DedupCountIter<T, I::IntoIter>;
        fn dedup_count(self) -> Self::Output {
            DedupCountIter::new(self.into_iter())
        }
    }
}

// ========

use combinations::*;
mod combinations {
    use std::ops::Div;

    use crate::Factorial;

    pub fn combinations<T>(k: usize, n: usize) -> T
    where
        T: Div<Output = T> + Factorial,
    {
        assert!(k <= n);
        // https://en.wikipedia.org/wiki/Combination
        // https://en.wikipedia.org/wiki/Binomial_coefficient
        // https://en.wikipedia.org/wiki/Pascal's_triangle
        // n=3, k=1:   1   2   3
        // n=3, k=2:  12  23  31
        // n=3, k=3: 123
        // C^k_n = n! / [k! (n - k)!] = n!
        T::factorial(n) / T::factorial(k) / T::factorial(n - k)
    }
}

// ========

use permutations::*;
mod permutations {
    use std::ops::Div;

    use crate::Factorial;

    pub fn permutations<T>(n: usize) -> T
    where
        T: Factorial,
    {
        // https://en.wikipedia.org/wiki/Permutation
        // n=3: 123 132 213 231 312 321
        // P_n = A^n_n = n! / (n - n)! = n!
        T::factorial(n)
    }

    pub fn k_permutations<T>(k: usize, n: usize) -> T
    where
        T: Div<Output = T> + Factorial,
    {
        assert!(k <= n);
        // https://en.wikipedia.org/wiki/Permutation#k-permutations_of_n
        // n=3, k=1:   1   2   3
        // n=3, k=2:  12  21  13  31  23  32
        // n=3, k=3: 123 132 213 231 312 321
        // A^k_n = n! / (n - k)!
        T::factorial(n) / T::factorial(n - k)
    }
}

// ========

use subsequences::*;
mod subsequences {
    use core::ops::{Add, Div, Mul};

    use crate::values::{One, Two};

    pub fn subsequences<T>(n: T) -> T
    where
        T: Add<Output = T> + Clone + Div<Output = T> + Mul<Output = T> + One + Two,
    {
        // n=1:  1
        // n=2:  1 2 12
        // n=3:  1 2 3 12 23 123
        // n=4:  1 2 3 4 12 23 34 123 234 1234
        n.clone() * (n + T::one()) / T::two()
    }
}

// ========

use solve_ax_cong_b_mod_p::*;
mod solve_ax_cong_b_mod_p {
    use core::ops::{Div, Mul, Rem};

    use crate::{gcd, Abs, ModularInv, Zero};

    pub fn solve_ax_cong_b_mod_p<T>(a: T, b: T, p: T) -> Option<T>
    where
        T: Abs
            + Clone
            + Div<Output = T>
            + Eq
            + Mul<Output = T>
            + PartialEq
            + Rem<Output = T>
            + ModularInv
            + Zero,
    {
        // https://ru.wikipedia.org/wiki/Сравнение_по_модулю
        let d = gcd(a.clone(), p.clone());
        if b.clone() % d.clone() != T::zero() {
            None
        } else {
            let a1 = a / d.clone();
            let b1 = b / d.clone();
            let p1 = p / d;
            Some(a1.modular_inv_prime(p1) * b1)
        }
    }
}

// ========

use modular_mul::*;
mod modular_mul {
    pub trait ModularMul {
        fn modular_mul(self, other: Self, modulus: Self) -> Self;
    }

    macro_rules! def {
        ( $low:ty, $hi:ty $(, $rest:ty)* ) => {
            impl ModularMul for $low {
                fn modular_mul(self, other: $low, modulus: Self) -> Self {
                    (((self as $hi) * (other as $hi)).rem_euclid(modulus as $hi)) as $low
                }
            }
            def!($hi $(, $rest)*);
        };
        ( $last:ty ) => {};
    }

    def!(u8, u16, u32, u64, u128);
}

// ========

use modular_pow::*;
mod modular_pow {
    use core::ops::{Add, BitAnd, Mul, Rem, ShrAssign};

    use crate::{MulRem, One, Zero};

    pub trait ModularPow<U> {
        fn modular_pow(self, exp: U, modulus: Self) -> Self;
    }

    impl<T, U> ModularPow<U> for T
    where
        T: Add<Output = T>
            + Clone
            + Copy
            + Mul<Output = T>
            + MulRem
            + One
            + PartialOrd
            + Rem<Output = T>
            + Zero,
        U: One + BitAnd<Output = U> + Clone + Copy + PartialEq<U> + PartialOrd + ShrAssign + Zero,
    {
        // https://doc.rust-lang.org/src/core/num/uint_macros.rs.html#1538-1558
        fn modular_pow(self, mut exp: U, modulus: Self) -> Self {
            if exp == U::zero() {
                return Self::one();
            }
            let mut base = self;
            let mut acc = Self::one();

            while exp > U::one() {
                if (exp & U::one()) == U::one() {
                    acc = acc.mul_rem(base, modulus);
                }
                exp >>= U::one();
                base = base.mul_rem(base, modulus);
            }

            acc.mul_rem(base, modulus)
        }
    }
}

// ========

use modular_inv::*;
mod modular_inv {
    use core::ops::{Add, BitAnd, Mul, Rem, ShrAssign, Sub};

    use crate::mul_rem::MulRem;
    use crate::{ModularPow, One, Two, Zero};

    pub trait ModularInv {
        fn modular_inv_prime(self, modulo: Self) -> Self;
    }

    impl<T> ModularInv for T
    where
        T: Add<Output = T>
            + BitAnd<Output = T>
            + Clone
            + Copy
            + Mul<T, Output = T>
            + MulRem
            + One
            + PartialOrd
            + Rem<Output = T>
            + ShrAssign
            + Sub<Output = T>
            + Two
            + Zero,
    {
        // https://en.wikipedia.org/wiki/Euler%27s_theorem
        // https://en.wikipedia.org/wiki/Fermat%27s_little_theorem
        // p: prime && a % p > 0
        // => a ** (p - 1) % p = 1
        // => a ** (p - 2) % p = a ** -1 % p
        fn modular_inv_prime(self, modulo: T) -> Self {
            assert!(self < modulo);
            self.modular_pow(modulo - T::two(), modulo)
        }
    }
}
