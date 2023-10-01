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
    clippy::too_many_lines,
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
use std::collections::btree_map::Entry as BTreeMapEntry;
use std::collections::hash_map::Entry as HashMapEntry;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque};
use std::io::{
    sink, stderr, stdin, stdout, BufRead, BufReader, BufWriter, Error as IoError,
    ErrorKind as IoErrorKind, Result as IoResult, Stderr, Stdin, Stdout, Write,
};
use std::sync::Arc;

#[cfg(test)]
use crate::tests::{test_with_examples, test_with_interactor, ChannelReader, ChannelWriter};

#[allow(unused_labels)]
pub fn problem<I: ReaderExt + WriterExt>(io: &mut I) {
    #[rustfmt::skip]
    macro_rules! re {
        () => {{ io.re() }};
        ( $ty:ty ) => {{ io.re::<$ty>() }};
        [ $len:expr ] => {{ match $len { len => (0..len).map(|_| io.re()) } }};
        [ $ty:ty; $len:expr ] => {{ match $len { len => (0..len).map(|_| io.re::<$ty>()) } }};
    }

    let t: usize = re!();
    'main: for _ in 0..t {
        let n = re!(usize);
        let a = re![i64; n].into_vec();
        io.li(a.iter().sum::<i64>());
    }
}

const EXAMPLES: &str = r####"
----
====
1
2 3 4
----
7
====
2
3 4 5 6
7 8 9 10 11 12 13 14
----
15
77
"####;

#[derive(Clone, Debug)]
pub struct Preset {}

#[allow(unused_variables, clippy::needless_pass_by_value)]
pub fn interactor<I: ReaderExt + WriterExt>(io: &mut I, preset: &Preset) {
    #[rustfmt::skip]
    macro_rules! re {
        () => {{ io.re() }};
        ( $ty:ty ) => {{ io.re::<$ty>() }};
        [ $len:expr ] => {{ match $len { len => (0..len).map(|_| io.re()) } }};
        [ $ty:ty; $len:expr ] => {{ match $len { len => (0..len).map(|_| io.re::<$ty>()) } }};
    }

    io.li(2i32);
    io.li((3i32, [1i32, 2i32, 3i32].wo()).wo());
    io.li((4i32, [2i32, 3i32, 4i32, 5i32].wo()).wo());
    io.fl();
    let a: i32 = re!();
    let b: i32 = re!();
    assert_eq!(a, 6, "{preset:?}");
    assert_eq!(b, 14, "{preset:?}");
}

pub fn main() {
    let mut io = Io::from_stdio();
    problem(&mut io);
}

#[test]
fn test_examples() {
    test_with_examples(|io| problem(io), EXAMPLES, true);
}

#[test]
#[cfg(feature = "interactive")]
fn test_interactor() {
    let presets = [Preset {}];
    for preset in &presets {
        test_with_interactor(problem, |io| interactor(io, preset));
    }
}

use debug::*;
mod debug {
    #[macro_export]
    macro_rules! d {
        ( $($arg:tt)* ) => ( if cfg!(debug_assertions) { $($arg)* } )
    }

    #[macro_export] // shorter dbg without pretty flag
    macro_rules! dl {
        () => {
            if cfg!(debug_assertions) {
                ::std::eprintln!("[{}:{}]", ::core::file!(), ::core::line!());
            }
        };
        ($($val:expr),+ $(,)?) => {
            if cfg!(debug_assertions) {
                ::std::eprint!("[{}:{}]", ::core::file!(), ::core::line!());
                $(
                    ::std::eprint!(
                        " {} = {:?};", ::core::stringify!($val), &$val
                    );
                )+
                ::std::eprintln!();
            }
        };
    }
}

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

use writer_ext::*;
mod writer_ext {
    use std::io::Write;

    use crate::{Io, Writable};

    pub trait WriterExt {
        fn jo1<T: Writable>(&mut self, value: T);
        fn wo1<T: Writable>(&mut self, value: T);
        fn ln(&mut self);
        fn fl(&mut self);

        fn ok(&mut self) {}

        fn li<T: Writable>(&mut self, value: T) {
            self.wo1(value);
            self.ln();
        }

        fn jo<T: Writable>(&mut self, value: T) -> &mut Self {
            self.jo1(value);
            self
        }

        fn wo<T: Writable>(&mut self, value: T) -> &mut Self {
            self.wo1(value);
            self
        }

        fn ask<T: Writable>(&mut self) -> &mut Self {
            self.wo1('?');
            self
        }

        fn ans(&mut self) -> &mut Self {
            self.wo1('!');
            self
        }

        fn yes(&mut self) -> &mut Self {
            self.wo1("YES");
            self
        }

        fn no(&mut self) -> &mut Self {
            self.wo1("NO");
            self
        }

        fn yes_no(&mut self, value: bool) -> &mut Self {
            if value {
                self.yes()
            } else {
                self.no()
            }
        }
    }

    impl<R, W: Write> WriterExt for Io<R, W> {
        fn jo1<T: Writable>(&mut self, value: T) {
            self.writer.jo1(value);
        }

        fn wo1<T: Writable>(&mut self, value: T) {
            self.writer.wo1(value);
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
        fn jo1<T: Writable>(&mut self, value: T) {
            value.write(&mut self.writer);
            self.is_seperator_needed = true;
        }

        fn wo1<T: Writable>(&mut self, value: T) {
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
            impl $( < $($args)* > )? $crate::Writable for $ty {
                fn write<W: ::std::io::Write>(self, writer: &mut W) {
                    $crate::Writable::write(&self, writer)
                }
            }

            #[allow(unused_qualifications)]
            impl $( < $($args)* > )? $crate::Writable for &mut $ty {
                fn write<W: ::std::io::Write>(self, writer: &mut W) {
                    $crate::Writable::write(&*self, writer)
                }
            }
        };
    }

    #[macro_export]
    macro_rules! def_wr_and_disp_by_ref {
        ( ( $( for $($args:tt)* )? ) $ty:ty ) => {
            $crate::def_wr_by_ref!( ( $( for $($args)* )? ) $ty );

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
                    Ok(ok) => write!(writer, "{ok}").unwrap(),
                    Err(err) => write!(writer, "{err}").unwrap(),
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

use word_start::*;
mod word_start {
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
                write!(writer, "{prefix}").unwrap();
            }
            if let Some(suffix) = self.suffix.into_option() {
                write!(writer, "{suffix}").unwrap();
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
                write!(writer, "{prefix}").unwrap();
            }
            let mut iter = self.value.into_iter();
            if let Some(first) = iter.next() {
                first.write(writer);
                for item in iter {
                    if let Some(separator) = self.separator.into_option() {
                        write!(writer, "{separator}").unwrap();
                    }
                    item.write(writer);
                }
            }
            if let Some(suffix) = self.suffix.into_option() {
                write!(writer, "{suffix}").unwrap();
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
        () => { std::collections::BTreeSet::new() };
        ($($x:expr),+ $(,)?) => {{
            let mut collection = std::collections::BTreeSet::new();
            $( let _ = collection.insert($x); )+
            collection
        }};
    }

    #[macro_export]
    macro_rules! btm {
        () => { std::collections::BTreeMap::new() };
        ($(($k:expr, $v:expr)),+ $(,)?) => {{
            let mut collection = std::collections::BTreeMap::new();
            $( let _ = collection.insert($k, $v); )+
            collection
        }};
    }

    #[macro_export]
    macro_rules! hs {
        () => { std::collections::HashSet::new() };
        ($($x:expr),+ $(,)?) => {{
            let mut collection = std::collections::HashSet::with_capacity(
                [$($crate::replace_expr!($x ())),+].len()
            );
            $( let _ = collection.insert($x); )+
            collection
        }};
    }

    #[macro_export]
    macro_rules! hm {
        () => { std::collections::HashMap::new() };
        ($(($k:expr, $v:expr)),+ $(,)?) => {{
            let mut collection = std::collections::HashMap::with_capacity(
                [$($crate::replace_expr!($v ())),+].len()
            );
            $( let _ = collection.insert($k, $v); )+
            collection
        }};
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

use bool_ext::*;
mod bool_ext {
    pub trait BoolExt {
        fn select<T>(self, t: T, f: T) -> T;
        fn select_with<T, F1: FnOnce() -> T, F2: FnOnce() -> T>(self, t: F1, f: F2) -> T;
        fn then_some<T>(self, some: T) -> Option<T>;
        fn into_result<T, E>(self, ok: T, err: E) -> Result<T, E>;
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

        fn into_result<T, E>(self, ok: T, err: E) -> Result<T, E> {
            self.select(Ok(ok), Err(err))
        }
    }
}

use vec_ext::*;
mod vec_ext {
    use std::cmp::Ordering;
    use std::mem::replace;

    pub trait VecExt<T> {
        fn wc(capacity: usize) -> Self;
        fn set_or_push(&mut self, j: usize, value: T);
        fn swap_insert(&mut self, j: usize, value: T);
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

        fn swap_insert(&mut self, j: usize, value: T) {
            match j.cmp(&self.len()) {
                Ordering::Less => {
                    let last = replace(&mut self[j], value);
                    self.push(last);
                }
                Ordering::Equal => self.push(value),
                Ordering::Greater => {
                    panic!(
                        "index out of bounds: the len is {} but the index is {}",
                        self.len(),
                        j,
                    );
                }
            }
        }
    }
}

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

pub use sortable::*;
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
            self.insertion_sort_by(|lhs, rhs| rhs.cmp(lhs));
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
            self.sort_by(|lhs, rhs| rhs.cmp(lhs));
        }

        fn sort_unstable_rev(&mut self)
        where
            Self::Item: Ord,
        {
            self.sort_unstable_by(|lhs, rhs| rhs.cmp(lhs));
        }

        // works faster on an array with a length of less than 8 elements
        fn insertion_sort(&mut self)
        where
            Self::Item: Ord,
        {
            self.insertion_sort_by(Ord::cmp);
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
            self.insertion_sort_by(|lhs, rhs| f(lhs).cmp(&f(rhs)));
        }
    }
}

use unsigned::*;
mod unsigned {
    pub trait Unsigned {}
    macro_rules! def { ( $($ty:ty),* ) => { $( impl Unsigned for $ty {} )* }; }
    def!(u8, u16, u32, u64, u128, usize);
}

use signed::*;
mod signed {
    pub trait Signed {}
    macro_rules! def { ( $($ty:ty),* ) => { $( impl Signed for $ty {} )* }; }
    def!(i8, i16, i32, i64, i128, isize);
}

use wrap::*;
mod wrap {
    use core::num::Wrapping;
    pub fn wrap<T>(value: T) -> Wrapping<T> {
        Wrapping(value)
    }
}

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

use gcd::*;
mod gcd {
    use core::ops::Rem;

    use crate::{Abs, Zero};

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
            #[allow(
                clippy::cast_possible_truncation,
                clippy::cast_sign_loss,
                clippy::cast_precision_loss
            )]
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

        pub fn factorize(&self, value: usize) -> PrimesFactorizeIter<'_> {
            assert!(value < self.len);
            PrimesFactorizeIter::new(self, value)
        }

        pub fn num_divisors(&self, value: usize) -> usize {
            let factorization = self.factorize(value);
            factorization
                .dedup_count()
                .map(|(_, count)| count + 1)
                .product()
        }

        pub fn eulers_phi(&self, value: usize) -> usize {
            if value == 1 {
                return 0;
            }
            let factorization = self.factorize(value);
            #[allow(clippy::cast_possible_truncation)]
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
                    if self.0.sieve[self.1] == 0 {
                        let item = self.1;
                        self.1 += 1;
                        return Some(item * 2 + 1);
                    }
                    self.1 += 1;
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
                    if divisor == 0 {
                        let item = self.1;
                        self.1 = 1;
                        Some(item)
                    } else {
                        self.1 /= divisor;
                        Some(divisor)
                    }
                }
            } else {
                None
            }
        }
    }

    impl FusedIterator for PrimesFactorizeIter<'_> {}
}

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

use modular_ops::*;
mod modular_ops {
    use core::ops::{Add, BitAnd, Mul, Rem, ShrAssign, Sub};

    use crate::{MulRem, One, Two, Zero};

    pub trait ModularMul {
        fn modular_mul(self, other: Self, modulus: Self) -> Self;
    }

    pub trait ModularPow<U> {
        fn modular_pow(self, exp: U, modulus: Self) -> Self;
    }

    pub trait ModularInv {
        fn modular_inv_prime(self, modulo: Self) -> Self;
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
        fn modular_inv_prime(self, modulo: T) -> Self {
            assert!(self < modulo);
            self.modular_pow(modulo - T::two(), modulo)
        }
    }
}

use math_ext::*;
pub mod math_ext {
    #[inline]
    #[must_use]
    pub fn div_rem_u128(divident: u128, divisor: u64) -> (u64, u64) {
        #[cfg(target_arch = "x86_64")]
        {
            let hi: u64 = (divident >> 64) as u64;
            debug_assert!(hi < divisor);
            #[allow(clippy::cast_possible_truncation)]
            let lo: u64 = divident as u64;
            let mut quot = lo;
            let mut rem = hi;
            unsafe {
                std::arch::asm!(
                    "div {divisor}",
                    divisor = in(reg) divisor,
                    inout("rax") quot,
                    inout("rdx") rem,
                    options(pure, nomem, nostack)
                );
                (quot, rem)
            }
        }
        #[cfg(not(target_arch = "x86_64"))]
        {
            let (quot, rem) = num::integer::div_rem(divident, divisor as u128);
            (quot as u64, rem as u64)
        }
    }
}
