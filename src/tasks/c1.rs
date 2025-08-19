#[cfg(test)]
#[path = "../lib-tests/mod.rs"]
mod lib_tests;

// use crate::{arr, d, dl, for_each};
use core::format_args as fa;

fn solver(re: &mut Input, wr: &mut Output, er: &mut Error) {
    let _ = er;
    let t: usize = re.re();
    dl!(t);
    'main: for _ in 0..t {
        let n: usize = re.re();
        dl!(n);
        let a: Vec<i64> = re.rec(n);
        dl!(a);
        let ans = a.iter().sum::<i64>();
        wr.li(ans);
    }
    // let a: &str = re.re();
    // wr.li(fa!("{:.4}", 0.1));
}

const TIMEOUT_SECONDS: f64 = 1.0;
const TEST_GEN_TIMEOUT_SECONDS: f64 = 1.0;
const MEMORY_LIMIT_BYTES: usize = 256 * 1024 * 1024;
const EXAMPLES: &str = r####"
====
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
"####; // "; // (fix codeforces code highlighting)

#[cfg(feature = "dev-mode")]
fn tests(wr: &mut Output) {
    use rand::{Rng, SeedableRng};
    use rand_chacha::ChaCha8Rng;

    wr.li("===="); // Test separator.
    wr.li("----"); // IO separator.
    // let mut rng = ChaCha8Rng::seed_from_u64(123);
    // for _ in 0..4 {
    //     let n = rng.random_range(1..=100);
    //     let m = rng.random_range(1..=1000);
    //     wr.li("====");
    //     wr.li(1);
    //     wr.li(n);
    //     wr.li(repeat_n(m, n).wo());
    //     wr.li("----");
    //     wr.li(n * m);
    // }
}

#[cfg(feature = "dev-mode")]
fn interactive_presets(wr: &mut Output) {
    let _ = wr;
    // wr.li("===="); // Preset separator.
    // for n in [1, 10, 100] {
    //     wr.li(n);
    //     wr.li("===="); // Preset separator.
    // }
}

#[cfg(feature = "dev-mode")]
fn interactor(re: &mut Input, wr: &mut Output, er: &mut Error) {
    let _ = er;
    let n: usize = re.re();
    wr.li(100);
    wr.li(repeat_n(10, n).wo());
    wr.fl();
    let v: u32 = re.re();
    assert_eq!(v, 10 * n as u32);
}

#[cfg(not(feature = "dev-mode"))]
pub fn main() {
    let (mut re, mut wr, mut er) = std_io();
    solver(&mut re, &mut wr, &mut er);
}

#[cfg(feature = "dev-mode")]
pub fn main() {
    let (mut re, mut wr, mut er) = std_io();
    let operation = std::env::args_os().nth(1);
    bridge(&mut re, &mut wr, &mut er, operation.as_deref());
}

#[cfg(feature = "dev-mode")]
fn bridge(re: &mut Input, wr: &mut Output, er: &mut Error, operation: Option<&OsStr>) {
    match operation.map(OsStr::as_encoded_bytes) {
        Some(b"solver") | None => solver(re, wr, er),
        Some(b"timeout-seconds") => wr.li(TIMEOUT_SECONDS),
        Some(b"memory-limit-bytes") => wr.li(MEMORY_LIMIT_BYTES),
        Some(b"examples") => wr.li(EXAMPLES),
        Some(b"tests") => tests(wr),
        Some(b"test-gen-timeout-seconds") => wr.li(TEST_GEN_TIMEOUT_SECONDS),
        Some(b"interactive-presets") => interactive_presets(wr),
        Some(b"interactor") => interactor(re, wr, er),
        Some(op) => panic!("unexpected operation: {:?}", String::from_utf8_lossy(op)),
    }
}

use prelude::*;
mod prelude {
    pub use core::array;
    pub use core::borrow::{Borrow, BorrowMut};
    pub use core::cell::RefCell;
    pub use core::cmp::Ordering::{self, Equal, Greater, Less};
    pub use core::cmp::{max, min};
    pub use core::convert::Infallible;
    pub use core::fmt::{Debug, Display, Formatter, Result as FmtResult};
    pub use core::iter::{
        empty, from_fn as iter_from_fn, once, once_with, repeat, repeat_n, repeat_with, successors,
    };
    pub use core::marker::PhantomData;
    pub use core::mem::{replace, swap, take};
    pub use core::str::{FromStr, from_utf8};
    pub use core::time::Duration;
    pub use std::collections::btree_map::Entry as BTreeMapEntry;
    pub use std::collections::hash_map::Entry as HashMapEntry;
    pub use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque};
    pub use std::ffi::{OsStr, OsString};
    pub use std::io::{
        BufRead, BufReader, BufWriter, Error as IoError, ErrorKind as IoErrorKind,
        Result as IoResult, Stderr, Stdin, Stdout, Write, sink, stderr, stdin, stdout,
    };
    pub use std::sync::Arc;
    pub use std::sync::mpsc::{Receiver, Sender};
}

pub use macros::*;
mod macros {
    #[macro_export]
    macro_rules! for_each {
        ( for $pat:pat in [ $( $expr:expr ),* $(,)? ] $body:block ) => {
            $( { let $pat = $expr; $body; } )*
        };
    }

    #[macro_export]
    macro_rules! d {
        ( $($arg:tt)* ) => ( if cfg!(feature = "stderr") { $($arg)* } )
    }

    // Shorter dbg without pretty flag.
    #[macro_export]
    macro_rules! dl {
        () => {
            if cfg!(feature = "stderr") {
                ::std::eprintln!("[{}:{}]", ::core::file!(), ::core::line!());
            }
        };
        ($($val:expr),+ $(,)?) => {
            if cfg!(feature = "stderr") {
                ::std::eprint!("[{}:{}]", ::core::file!(), ::core::line!());
                $( ::std::eprint!(" {} = {:?};", ::core::stringify!($val), &$val); )+
                ::std::eprintln!();
            }
        };
    }

    #[macro_export]
    macro_rules! arr {
        [ |$arg:ident| $expr:expr; $len:literal ] => {
            ::core::array::from_fn::<_, $len, _>(|$arg| $expr)
        };
        [ $expr:expr; $len:literal ] => {
            ::core::array::from_fn::<_, $len, _>(|_| $expr)
        };
    }
}

// IO and formatting libraries.

pub use io::*;
mod io {
    use std::io::{BufReader, BufWriter};

    use super::{Reader, Writer};

    pub type Input = Reader<BufReader<std::io::Stdin>>;
    pub type Output = Writer<BufWriter<std::io::Stdout>>;
    #[cfg(feature = "stderr")]
    pub type Error = Writer<BufWriter<std::io::Stderr>>;
    #[cfg(not(feature = "stderr"))]
    pub type Error = Writer<BufWriter<std::io::Sink>>;

    #[inline]
    #[must_use]
    pub fn std_io() -> (Input, Output, Error) {
        let re = Reader::new(BufReader::new(std::io::stdin()));
        let wr = Writer::new(BufWriter::new(std::io::stdout()));
        #[cfg(feature = "stderr")]
        let er = Writer::new(BufWriter::new(std::io::stderr()));
        #[cfg(not(feature = "stderr"))]
        let er = Writer::new(BufWriter::new(std::io::sink()));
        (re, wr, er)
    }
}

pub use reader::*;
mod reader {
    use std::io::{BufRead, Error as IoError, ErrorKind as IoErrorKind, Result as IoResult};

    #[derive(Clone, Debug)]
    pub struct Reader<R> {
        pub reader: R,
        pub line: String,
        pub offset: usize,
        pub cached: bool,
    }

    impl<R> Reader<R> {
        #[inline]
        pub const fn new(reader: R) -> Self {
            Self {
                reader,
                offset: 0,
                line: String::new(),
                cached: false,
            }
        }
    }

    impl<R: BufRead> Reader<R> {
        #[inline]
        pub fn cache_next_line(&mut self) -> IoResult<()> {
            self.offset = 0;
            self.line.clear();
            let len = self.reader.read_line(&mut self.line)?;
            if len > 0 {
                self.cached = true;
                Ok(())
            } else {
                Err(IoError::from(IoErrorKind::UnexpectedEof))
            }
        }

        #[inline]
        pub fn drop_cached_line(&mut self) {
            self.offset = 0;
            self.line.clear();
            self.cached = false;
        }

        #[inline]
        pub fn remaining(&self) -> &str {
            &self.line[self.offset..]
        }

        #[inline]
        pub fn skip_line_whitespace(&mut self) {
            let remaining = self.line[self.offset..].bytes();
            let offset = remaining.take_while(u8::is_ascii_whitespace).count();
            self.offset += offset;
        }

        #[inline]
        pub fn skip_whitespace(&mut self) -> IoResult<()> {
            loop {
                self.skip_line_whitespace();
                if !self.remaining().is_empty() {
                    return Ok(());
                }
                self.cache_next_line()?;
            }
        }

        #[inline]
        pub fn read_line_word(&mut self) -> &str {
            self.skip_line_whitespace();
            let remaining = self.line[self.offset..].bytes();
            let len = remaining.take_while(|ch| !ch.is_ascii_whitespace()).count();
            let value = &self.line[self.offset..][..len];
            self.offset += len;
            value
        }

        #[inline]
        pub fn read_word(&mut self) -> IoResult<&str> {
            self.skip_whitespace()?;
            Ok(self.read_line_word())
        }
    }
}

pub use writer::*;
mod writer {
    #[derive(Clone, Debug)]
    pub struct Writer<W> {
        pub writer: W,
        pub is_dirty: bool,
    }

    impl<W> Writer<W> {
        #[inline]
        pub const fn new(writer: W) -> Self {
            Self {
                writer,
                is_dirty: false,
            }
        }
    }
}

pub use readable::*;
mod readable {
    use core::hash::{BuildHasher, Hash};
    use core::num::{
        NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize, NonZeroU8,
        NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize,
    };
    use core::str::FromStr;
    use core::{array, iter};
    use std::collections::{
        BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque,
    };
    use std::io::BufRead;

    use super::Reader;

    pub trait Readable<'a, C> {
        #[track_caller]
        fn read_from<R: BufRead>(reader: &'a mut Reader<R>, context: C) -> Self;
    }

    macro_rules! def_impl {
        ( $( $ty:ty ),* $(,)? ) => {
            $(
                impl<'a> Readable<'a, ()> for $ty {
                    #[inline]
                    fn read_from<R: BufRead>(reader: &'a mut Reader<R>, (): ()) -> Self {
                        let word = reader.read_word().unwrap();
                        FromStr::from_str(word).unwrap()
                    }
                }
            )*
        };
    }
    def_impl! {
        u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize,
        NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize,
        NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize,
        bool, f32, f64, char
    }

    impl<'a> Readable<'a, ()> for &'a str {
        #[inline]
        fn read_from<R: BufRead>(reader: &'a mut Reader<R>, (): ()) -> Self {
            reader.read_word().unwrap()
        }
    }

    impl<'a> Readable<'a, ()> for String {
        #[inline]
        fn read_from<R: BufRead>(reader: &'a mut Reader<R>, (): ()) -> Self {
            <&'a str>::read_from(reader, ()).to_owned()
        }
    }

    impl<'a> Readable<'a, usize> for &'a str {
        #[inline]
        fn read_from<R: BufRead>(reader: &'a mut Reader<R>, len: usize) -> Self {
            while reader.remaining().trim().is_empty() {
                reader.cache_next_line().unwrap();
            }
            let line = &reader.line[reader.offset..].trim_end_matches('\n');
            let trimmed = line.trim_start();
            let removed_len = line.len() - trimmed.len();
            assert!(
                trimmed.len() >= len,
                "can not read string of length {len} from the line {line:?}"
            );
            reader.offset += len + removed_len;
            &trimmed[0..len]
        }
    }

    impl<'a> Readable<'a, usize> for String {
        #[inline]
        fn read_from<R: BufRead>(reader: &'a mut Reader<R>, len: usize) -> Self {
            <&'a str>::read_from(reader, len).to_owned()
        }
    }

    impl<'a, T, const N: usize, C> Readable<'a, C> for [T; N]
    where
        T: for<'b> Readable<'b, C>,
        C: Copy,
    {
        #[inline]
        fn read_from<R: BufRead>(reader: &'a mut Reader<R>, context: C) -> Self {
            array::from_fn(|_| T::read_from(reader, context))
        }
    }

    macro_rules! def_impl {
        ( ( $lt:lifetime, $ctx:tt $( $args:tt )* ) $in:ty => $out:ty $(, where $( $bounds:tt )* )? ) => {
            impl<$lt $( $args )*> Readable<$lt, usize> for $out
            where
                $in: for<'b> Readable<'b, ()>,
                $( $( $bounds )* )?
            {
                #[inline]
                fn read_from<R: BufRead>(reader: &$lt mut Reader<R>, len: usize) -> Self {
                    iter::repeat_with(|| <$in>::read_from(reader, ())).take(len).collect()
                }
            }

            impl<$lt, $ctx $( $args )*> Readable<$lt, (usize, $ctx)> for $out
            where
                $in: for<'b> Readable<'b, C>,
                $ctx: Copy,
                $( $( $bounds )* )?
            {
                #[inline]
                fn read_from<R: BufRead>(reader: &$lt mut Reader<R>, context: (usize, $ctx)) -> Self {
                    iter::repeat_with(|| <$in>::read_from(reader, context.1)).take(context.0).collect()
                }
            }
        };
    }

    def_impl!(('a, C, T,) T => Vec<T>);
    def_impl!(('a, C, T,) T => VecDeque<T>);
    def_impl!(('a, C, T,) T => LinkedList<T>);
    def_impl!(('a, C, T,) T => BinaryHeap<T>, where T: Ord);
    def_impl!(('a, C, T,) T => BTreeSet<T>, where T: Ord);
    def_impl!(('a, C, K, V,) (K, V) => BTreeMap<K, V>, where K: Ord);
    def_impl!(('a, C, T, S,) T => HashSet<T, S>,
        where T: Eq + Hash, S: Default + BuildHasher);
    def_impl!(('a, C, K, V, S,) (K, V) => HashMap<K, V, S>,
        where K: Eq + Hash, S: Default + BuildHasher);

    macro_rules! impl_variadic {
        ( [ $( $args:tt )* ] [ $next:tt $( $rest:tt )* ] ) => {
            impl_variadic!( [ $( $args )* ] [] );
            impl_variadic!( [ $( $args )* $next ] [ $( $rest )* ] );
        };
        ( [] [] ) => {};
        ( [ $( ( $field:tt $type:ident $context:ident ) )* ] [] ) => {
            impl<'a $(, $type )*> Readable<'a, ()> for ( $( $type, )* )
            where
                $( $type: for<'b> Readable<'b, ()>, )*
            {
                #[inline]
                fn read_from<R: BufRead>(reader: &'a mut Reader<R>, (): ()) -> Self {
                    ( $( <$type>::read_from(reader, ()), )* )
                }
            }

            impl<'a $(, $type )* $(, $context )*> Readable<'a, ( $( $context, )* )>
                for ( $( $type, )* )
            where
                $( $type: for<'b> Readable<'b, $context>, )*
            {
                #[inline]
                fn read_from<R: BufRead>(
                    reader: &'a mut Reader<R>,
                    context: ( $( $context, )* )
                ) -> Self {
                    ( $( <$type>::read_from(reader, context.$field), )* )
                }
            }
        };
    }

    impl_variadic! {
        [] [
            (0 T1 C1) (1 T2 C2) (2 T3 C3) (3 T4 C4)
            (4 T5 C5) (5 T6 C6) (6 T7 C7) (7 T8 C8)
            (8 T9 C9) (9 T10 C10) (10 T11 C11) (11 T12 C12)
        ]
    }
}

pub use writable::*;
mod writable {
    use core::fmt::Arguments;
    use core::num::{
        NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize, NonZeroU8,
        NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize,
    };
    use std::io::Write;

    pub trait Writable {
        fn write_into<W: Write>(self, writer: &mut W);
    }

    macro_rules! def_impl {
        ( $ty1:ty => $ty2:ty, $self:ident => $expr:expr ) => {
            #[allow(clippy::mut_mut)]
            impl<'a, T> Writable for $ty1
            where
                $ty2: Writable,
                T: ?Sized,
            {
                #[inline]
                fn write_into<W: Write>($self, writer: &mut W) {
                    <$ty2>::write_into($expr, writer);
                }
            }
        };
    }

    def_impl!(&'a &'a T => &'a T, self => &**self);
    def_impl!(&'a mut &'a T => &'a T, self => &**self);
    def_impl!(&'a &'a mut T => &'a T, self => &**self);
    def_impl!(&'a mut &'a mut T => &'a mut T, self => &mut **self);

    macro_rules! def_impl {
        ( $( $ty:ty ),* $(,)? ) => {
            $(
                impl Writable for $ty {
                    #[inline]
                    fn write_into<W: Write>(self, writer: &mut W) {
                        write!(writer, "{self}").unwrap();
                    }
                }

                impl Writable for &$ty {
                    #[inline]
                    fn write_into<W: Write>(self, writer: &mut W) {
                        write!(writer, "{self}").unwrap();
                    }
                }

                impl Writable for &mut $ty {
                    #[inline]
                    fn write_into<W: Write>(self, writer: &mut W) {
                        write!(writer, "{self}").unwrap();
                    }
                }
            )*
        };
    }
    def_impl! {
        u8, u16, u32, u64, u128, usize,
        i8, i16, i32, i64, i128, isize,
        NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize,
        NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize,
        bool, f32, f64,
        char, String, Arguments<'_>
    }

    impl Writable for &str {
        #[inline]
        fn write_into<W: Write>(self, writer: &mut W) {
            write!(writer, "{self}").unwrap();
        }
    }

    impl Writable for &mut str {
        #[inline]
        fn write_into<W: Write>(self, writer: &mut W) {
            write!(writer, "{self}").unwrap();
        }
    }
}

pub use reader_ext::*;
mod reader_ext {
    use core::iter::repeat_with;
    use std::io::BufRead;

    use super::{Readable, Reader};

    pub trait ReaderExt {
        fn re<'a, T>(&'a mut self) -> T
        where
            T: Readable<'a, ()>;

        fn rec<'a, T, C>(&'a mut self, context: C) -> T
        where
            T: Readable<'a, C>;

        fn rei<T>(&mut self, len: usize) -> impl Iterator<Item = T>
        where
            T: for<'b> Readable<'b, ()>;
    }

    impl<R: BufRead> ReaderExt for Reader<R> {
        #[inline]
        #[track_caller]
        fn re<'a, T>(&'a mut self) -> T
        where
            T: Readable<'a, ()>,
        {
            T::read_from(self, ())
        }

        #[inline]
        #[track_caller]
        fn rec<'a, T, C>(&'a mut self, context: C) -> T
        where
            T: Readable<'a, C>,
        {
            T::read_from(self, context)
        }

        #[inline]
        #[track_caller]
        fn rei<T>(&mut self, len: usize) -> impl Iterator<Item = T>
        where
            T: for<'b> Readable<'b, ()>,
        {
            repeat_with(|| T::read_from(self, ())).take(len)
        }
    }
}

pub use writer_ext::*;
mod writer_ext {
    use std::io::Write;

    use super::{Writable, Writer};

    pub trait WriterExt {
        fn jo<T: Writable>(&mut self, value: T) -> &mut Self;
        fn wo<T: Writable>(&mut self, value: T) -> &mut Self;
        fn ln(&mut self);
        fn fl(&mut self);

        #[inline]
        fn li<T: Writable>(&mut self, value: T) {
            self.wo(value).ln();
        }

        #[inline]
        fn lifl<T: Writable>(&mut self, value: T) {
            self.wo(value).ln();
            self.fl();
        }

        #[inline]
        fn lnfl<T: Writable>(&mut self) {
            self.ln();
            self.fl();
        }

        #[inline]
        fn ask(&mut self) -> &mut Self {
            self.wo("?")
        }

        #[inline]
        fn ans(&mut self) -> &mut Self {
            self.wo("!")
        }

        #[inline]
        fn yes(&mut self) -> &mut Self {
            self.wo("YES")
        }

        #[inline]
        fn no(&mut self) -> &mut Self {
            self.wo("NO")
        }

        #[inline]
        fn yes_no(&mut self, value: bool) -> &mut Self {
            if value { self.yes() } else { self.no() }
        }

        #[inline]
        fn ok(&mut self) {
            _ = self;
        }
    }

    impl<W: Write> WriterExt for Writer<W> {
        #[inline]
        fn fl(&mut self) {
            self.writer.flush().unwrap();
        }

        #[inline]
        fn ln(&mut self) {
            self.writer.write_all(b"\n").unwrap();
            self.is_dirty = false;
        }

        #[inline]
        fn jo<T: Writable>(&mut self, value: T) -> &mut Self {
            value.write_into(&mut self.writer);
            self.is_dirty = true;
            self
        }

        #[inline]
        fn wo<T: Writable>(&mut self, value: T) -> &mut Self {
            if self.is_dirty {
                self.writer.write_all(b" ").unwrap();
            }
            value.write_into(&mut self.writer);
            self.is_dirty = true;
            self
        }
    }
}

pub use tuple_ext::*;
mod tuple_ext {
    pub trait Tuple {}

    macro_rules! impl_variadic {
        ( [ $( $args:tt )* ] [ $next:tt $( $rest:tt )* ] ) => {
            impl_variadic!( [ $( $args )* ] [] );
            impl_variadic!( [ $( $args )* $next ] [ $( $rest )* ] );
        };
        ( [ $( ( $type:ident ) )* ] [] ) => {
            impl<$( $type ),*> Tuple for ( $( $type, )* ) {}
        };
    }

    impl_variadic! {
        [] [ (T1) (T2) (T3) (T4) (T5) (T6) (T7) (T8) (T9) (T10) (T11) (T12) ]
    }
}

pub use decorated_tuple::*;
mod decorated_tuple {
    use std::io::Write;

    use super::{Tuple, Writable};

    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Debug)]
    pub struct DecoratedTuple<'a, Tuple> {
        tuple: Tuple,
        prefix: &'a str,
        separator: &'a str,
        suffix: &'a str,
    }

    impl<'a, Tuple> DecoratedTuple<'a, Tuple> {
        #[inline]
        pub const fn new(
            tuple: Tuple,
            prefix: &'a str,
            separator: &'a str,
            suffix: &'a str,
        ) -> Self {
            Self {
                tuple,
                prefix,
                separator,
                suffix,
            }
        }
    }

    impl Writable for DecoratedTuple<'_, ()> {
        #[inline]
        fn write_into<W: Write>(self, _: &mut W) {}
    }

    macro_rules! impl_variadic {
        ( [ $( $args:tt )* ] [ $next:tt $( $rest:tt )* ] ) => {
            impl_variadic!( [ $( $args )* ] [] );
            impl_variadic!( [ $( $args )* $next ] [ $( $rest )* ] );
        };
        ( [] [] ) => {};
        ( [ ( $field:tt $type:ident ) $( ( $fields:tt $types:ident ) )* ] [] ) => {
            impl<$type $(, $types )*> Writable for DecoratedTuple<'_, ( $type, $( $types, )* )>
            where
                $type: Writable,
                $( $types: Writable, )*
            {
                #[inline]
                fn write_into<W: Write>(self, writer: &mut W) {
                    self.prefix.write_into(writer);
                    self.tuple.$field.write_into(writer);
                    self.suffix.write_into(writer);
                    $(
                        self.separator.write_into(writer);
                        self.prefix.write_into(writer);
                        self.tuple.$fields.write_into(writer);
                        self.suffix.write_into(writer);
                    )*
                }
            }
        };
    }

    impl_variadic! {
        [] [
            (0 T1) (1 T2) (2 T3) (3 T4)(4 T5) (5 T6) (6 T7) (7 T8)
            (8 T9) (9 T10) (10 T11) (11 T12)
        ]
    }

    pub trait DecoratableTuple: Sized {
        #[inline]
        fn decorate<'a>(
            self,
            prefix: &'a str,
            separator: &'a str,
            suffix: &'a str,
        ) -> DecoratedTuple<'a, Self> {
            DecoratedTuple::new(self, prefix, separator, suffix)
        }

        #[inline]
        fn sep(self, separator: &str) -> DecoratedTuple<'_, Self> {
            DecoratedTuple::new(self, "", separator, "")
        }

        #[inline]
        fn jo(self) -> DecoratedTuple<'static, Self> {
            DecoratedTuple::new(self, "", "", "")
        }

        #[inline]
        fn wo(self) -> DecoratedTuple<'static, Self> {
            DecoratedTuple::new(self, "", " ", "")
        }

        #[inline]
        fn li(self) -> DecoratedTuple<'static, Self> {
            DecoratedTuple::new(self, "", "\n", "")
        }
    }

    impl<T> DecoratableTuple for T where T: Tuple {}
}

pub use decorated_iterator::*;
mod decorated_iterator {
    use std::io::Write;

    use super::Writable;

    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Debug)]
    pub struct DecoratedIterator<'a, I> {
        iter: I,
        separator: &'a str,
    }

    impl<'a, I> DecoratedIterator<'a, I> {
        #[inline]
        pub const fn new(iter: I, separator: &'a str) -> Self {
            Self { iter, separator }
        }
    }

    impl<I> Writable for DecoratedIterator<'_, I>
    where
        I: IntoIterator,
        I::Item: Writable,
    {
        #[inline]
        fn write_into<W: Write>(self, writer: &mut W) {
            let mut iter = self.iter.into_iter();
            if let Some(first) = iter.next() {
                first.write_into(writer);
                for item in iter {
                    self.separator.write_into(writer);
                    item.write_into(writer);
                }
            }
        }
    }

    pub trait DecoratableIterator: Sized {
        #[inline]
        fn decorate(self, separator: &str) -> DecoratedIterator<'_, Self> {
            DecoratedIterator::new(self, separator)
        }

        #[inline]
        fn sep(self, separator: &str) -> DecoratedIterator<'_, Self> {
            DecoratedIterator::new(self, separator)
        }

        #[inline]
        fn jo(self) -> DecoratedIterator<'static, Self> {
            DecoratedIterator::new(self, "")
        }

        #[inline]
        fn wo(self) -> DecoratedIterator<'static, Self> {
            DecoratedIterator::new(self, " ")
        }

        #[inline]
        fn li(self) -> DecoratedIterator<'static, Self> {
            DecoratedIterator::new(self, "\n")
        }
    }

    impl<I: IntoIterator> DecoratableIterator for I {}
}

pub use word::*;
mod word {
    use core::fmt::{Debug, Display, Formatter, Result as FmtResult};
    use std::io::{BufRead, Write};

    use super::{Readable, Reader, Writable};

    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
    pub struct Word<const LEN: usize>(pub [u8; LEN]);

    impl<const LEN: usize> Word<LEN> {
        #[inline]
        #[must_use]
        pub fn as_str(&self) -> &str {
            str::from_utf8(&self.0).expect("non-utf8 Word contents")
        }
    }

    impl<'a, const LEN: usize> Readable<'a, ()> for Word<LEN> {
        #[inline]
        #[track_caller]
        fn read_from<R: BufRead>(reader: &'a mut Reader<R>, (): ()) -> Self {
            let string = <&'a str>::read_from(reader, LEN);
            Self(<[u8; LEN]>::try_from(string.as_bytes()).unwrap())
        }
    }

    impl<'a, const LEN: usize> Readable<'a, usize> for Word<LEN> {
        #[inline]
        fn read_from<R: BufRead>(reader: &'a mut Reader<R>, len: usize) -> Self {
            assert!(len <= LEN);
            let string = <&'a str>::read_from(reader, len);
            let mut word = [0; LEN];
            word[0..len].copy_from_slice(string.as_bytes());
            Self(word)
        }
    }

    macro_rules! def_impl {
        ( $ty:ty ) => {
            impl<const LEN: usize> Writable for $ty {
                #[inline]
                fn write_into<W: Write>(self, writer: &mut W) {
                    write!(writer, "{}", self.as_str()).unwrap();
                }
            }
        };
    }
    def_impl!(Word<LEN>);
    def_impl!(&Word<LEN>);
    def_impl!(&mut Word<LEN>);

    impl<const LEN: usize> Debug for Word<LEN> {
        #[inline]
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            Debug::fmt(self.as_str(), f)
        }
    }

    impl<const LEN: usize> Display for Word<LEN> {
        #[inline]
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            Display::fmt(self.as_str(), f)
        }
    }
}

// Auxiliary libraries.

pub use array_ext::*;
mod array_ext {
    pub trait ArrayExt {
        #[must_use]
        fn rev(self) -> Self;
    }

    impl<T, const N: usize> ArrayExt for [T; N] {
        #[inline]
        fn rev(mut self) -> Self {
            self.reverse();
            self
        }
    }
}

pub use into_vec::*;
mod into_vec {
    pub trait IntoVec<T> {
        fn into_vec(self) -> Vec<T>;
    }

    impl<I, T> IntoVec<T> for I
    where
        I: IntoIterator<Item = T>,
    {
        #[inline]
        fn into_vec(self) -> Vec<T> {
            self.into_iter().collect()
        }
    }
}

pub use dedup_count::*;
mod dedup_count {
    use core::iter::FusedIterator;

    #[derive(Clone, Debug)]
    pub struct DedupCountIter<I, T> {
        iter: I,
        prev: Option<(T, usize)>,
    }

    impl<I, T> DedupCountIter<I, T> {
        #[inline]
        pub const fn new(iter: I) -> Self {
            Self { iter, prev: None }
        }
    }

    impl<I, T> Iterator for DedupCountIter<I, T>
    where
        T: Eq,
        I: Iterator<Item = T>,
    {
        type Item = (T, usize);

        #[inline]
        fn next(&mut self) -> Option<Self::Item> {
            loop {
                match (self.prev.take(), self.iter.next()) {
                    (None, Some(item)) => self.prev = Some((item, 1)),
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
                    (None, None) => {
                        return None;
                    }
                }
            }
        }
    }

    impl<I, T> FusedIterator for DedupCountIter<I, T>
    where
        T: Eq,
        I: Iterator<Item = T>,
    {
    }

    pub trait DedupCount {
        type Output;
        fn dedup_count(self) -> Self::Output;
    }

    impl<I, T> DedupCount for I
    where
        I: IntoIterator<Item = T>,
    {
        type Output = DedupCountIter<I::IntoIter, T>;

        #[inline]
        fn dedup_count(self) -> Self::Output {
            DedupCountIter::new(self.into_iter())
        }
    }
}

pub use slice_ext::*;
mod slice_ext {
    use core::cmp::Ordering;

    pub trait SliceExt {
        type Item;

        fn sort_rev(&mut self)
        where
            Self::Item: Ord;

        fn sort_unstable_rev(&mut self)
        where
            Self::Item: Ord;

        fn binary_search_first_or_less(&self, x: &Self::Item) -> usize
        where
            Self::Item: Ord;
    }

    impl<T> SliceExt for [T] {
        type Item = T;

        #[inline]
        fn sort_rev(&mut self)
        where
            Self::Item: Ord,
        {
            self.sort_by(|lhs, rhs| rhs.cmp(lhs));
        }

        #[inline]
        fn sort_unstable_rev(&mut self)
        where
            Self::Item: Ord,
        {
            self.sort_unstable_by(|lhs, rhs| rhs.cmp(lhs));
        }

        #[inline]
        fn binary_search_first_or_less(&self, x: &Self::Item) -> usize
        where
            Self::Item: Ord,
        {
            match self.binary_search_by(|p| p.cmp(x).then(Ordering::Greater)) {
                Ok(i) | Err(i) => i,
            }
        }
    }
}

pub use math_ext::*;
mod math_ext {
    pub trait IntegerExt {
        type Widen;
    }

    pub trait UnsignedExt: Sized + IntegerExt {
        fn mul_div(self, mul: Self, div: Self) -> Self::Widen;
        fn mul_rem(self, mul: Self, rem: Self) -> Self::Widen;

        #[must_use]
        fn gcd(self, other: Self) -> Self;
        fn lcm(self, other: Self) -> Self::Widen;

        fn solve_ax_plus_by_eq_gcd_ab(self, other: Self) -> (Self, Self, Self);
    }

    macro_rules! def {
        ( $( ( $ty:tt $wide:tt ) )* ) => {
            $(
                impl IntegerExt for $ty {
                    type Widen = $wide;
                }
            )*
        };
    }

    def!((u8 u16) (u16 u32) (u32 u64) (u64 u128) (u128 u128) (usize u128));
    def!((i8 i16) (i16 i32) (i32 i64) (i64 i128) (i128 i128) (isize i128));

    macro_rules! def {
        ( $( $ty:tt )* ) => {
            $(
                #[allow(trivial_numeric_casts)]
                impl UnsignedExt for $ty {
                    #[inline]
                    fn mul_div(self, mul: Self, div: Self) -> Self::Widen {
                        self as Self::Widen * mul as Self::Widen / div as Self::Widen
                    }

                    #[inline]
                    fn mul_rem(self, mul: Self, rem: Self) -> Self::Widen {
                        self as Self::Widen * mul as Self::Widen % rem as Self::Widen
                    }


                    #[inline]
                    fn gcd(self, other: Self) -> Self {
                        if other == 0 {
                            self
                        } else {
                            other.gcd(self % other)
                        }
                    }

                    #[inline]
                    fn lcm(self, other: Self) -> Self::Widen {
                        self.mul_div(other, self.gcd(other))
                    }

                    #[inline]
                    fn solve_ax_plus_by_eq_gcd_ab(self, other: Self) -> (Self, Self, Self) {
                        if other == 0 {
                            (self, 1, 0)
                        } else {
                            let (gcd, x, y) = solve_ax_plus_by_eq_gcd_ab(other, self % other);
                            (gcd, y, x - (self / other) * y)
                        }
                    }
                }
            )*
        };
    }

    def!(u8 u16 u32 u64 u128 usize);

    #[inline]
    pub fn gcd<T>(lhs: T, rhs: T) -> T
    where
        T: UnsignedExt,
    {
        lhs.gcd(rhs)
    }

    #[inline]
    pub fn lcm<T>(lhs: T, rhs: T) -> T::Widen
    where
        T: UnsignedExt,
    {
        lhs.lcm(rhs)
    }

    /// Solves: ax + by = gcd(a, b).
    /// Extended euclidean algorithm.
    pub fn solve_ax_plus_by_eq_gcd_ab<T>(lhs: T, rhs: T) -> (T, T, T)
    where
        T: UnsignedExt,
    {
        lhs.solve_ax_plus_by_eq_gcd_ab(rhs)
    }
}

pub use factorial::*;
mod factorial {
    pub const FACTORIALS: [u128; 35] = [
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
        295_232_799_039_604_140_847_618_609_643_520_000_000,
    ];
}

pub use primes::*;
mod primes {
    use core::iter::FusedIterator;

    use super::DedupCount;

    #[derive(Clone, Debug)]
    pub struct Primes {
        // 1 3 5 7 9 11 13 15 17 19 21 23 25 27 29 ...
        // 0 0 0 0 3  0  0  3  0  0  3  0  5  3  0 ...
        sieve: Vec<usize>,
        len: usize,
    }

    impl Primes {
        #[inline]
        #[must_use]
        pub fn new(max: usize) -> Self {
            let len = max + 1;
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

        #[inline]
        #[must_use]
        pub fn odds_sieve(&self) -> &[usize] {
            &self.sieve
        }

        #[inline]
        #[must_use]
        pub fn get_sieve_value(&self, value: usize) -> usize {
            if value < 4 {
                0
            } else if value & 1 == 0 {
                2
            } else {
                self.sieve[value / 2]
            }
        }

        #[inline]
        #[must_use]
        pub fn is_prime(&self, value: usize) -> bool {
            if value < 3 {
                value == 2
            } else {
                value & 1 == 1 && self.sieve[value / 2] == 0
            }
        }

        /// 2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, ...
        #[inline]
        #[must_use]
        pub const fn iter(&self) -> PrimesIter<'_> {
            PrimesIter::new(self, 0)
        }

        #[inline]
        #[must_use]
        pub const fn iter_from(&self, from: usize) -> PrimesIter<'_> {
            PrimesIter::new(self, from)
        }

        /// `factorize(2 * 3 * 11 * 103) = [2, 3, 11, 103]`
        #[inline]
        #[must_use]
        pub fn factorize(&self, value: usize) -> PrimesFactorizeIter<'_> {
            assert!(value < self.len);
            PrimesFactorizeIter::new(self, value)
        }

        /// `factorize(2 * 3 * 11 * 103) = 4`
        #[inline]
        #[must_use]
        pub fn num_divisors(&self, value: usize) -> usize {
            let factorization = self.factorize(value);
            factorization
                .dedup_count()
                .map(|(_, count)| count + 1)
                .product()
        }

        /// Counts (GCD = 1) integers up to given that are coprime to it.
        ///
        /// - `eulers_phi(5) = 4`.
        /// - `eulers_phi(6) = 2`.
        /// - `eulers_phi(7) = 6`.
        /// - `eulers_phi(8) = 4`.
        #[inline]
        #[must_use]
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

    impl<'a> IntoIterator for &'a Primes {
        type Item = usize;
        type IntoIter = PrimesIter<'a>;

        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }

    #[derive(Clone, Debug)]
    pub struct PrimesIter<'a>(&'a Primes, usize);

    impl<'a> PrimesIter<'a> {
        #[inline]
        #[must_use]
        pub const fn new(sieve: &'a Primes, from: usize) -> Self {
            Self(sieve, if from < 3 { 0 } else { from / 2 })
        }
    }

    impl Iterator for PrimesIter<'_> {
        type Item = usize;

        #[inline]
        fn next(&mut self) -> Option<Self::Item> {
            if self.1 == 0 {
                (self.0.len > 2).then(|| {
                    self.1 = 1;
                    2
                })
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
        #[inline]
        #[must_use]
        pub const fn new(sieve: &'a Primes, value: usize) -> Self {
            Self(sieve, value)
        }
    }

    impl Iterator for PrimesFactorizeIter<'_> {
        type Item = usize;

        #[inline]
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

pub use modular_ops::*;
mod modular_ops {
    use super::{IntegerExt, UnsignedExt, gcd};

    pub trait ModularExt: Sized + IntegerExt {
        #[must_use]
        fn modular_mul(self, other: Self, modulus: Self) -> Self;

        #[must_use]
        fn modular_pow(self, exp: Self, modulus: Self) -> Self;

        #[must_use]
        fn modular_inv_prime(self, modulus: Self) -> Self;

        #[must_use]
        fn modular_inv_with_eulers_phi(self, modulus: Self, modulus_eulers_phi: Self) -> Self;

        #[must_use]
        fn solve_ax_cong_b_mod_p(a: Self, b: Self, p: Self) -> Option<Self>;
    }

    macro_rules! def {
        ( $( $ty:tt )* ) => {
            $(
                #[allow(trivial_numeric_casts)]
                impl ModularExt for $ty {
                    #[inline]
                    fn modular_mul(self, other: Self, modulus: Self) -> Self {
                        (self as Self::Widen * other as Self::Widen)
                            .rem_euclid(modulus as Self::Widen) as Self
                    }

                    #[inline]
                    fn modular_pow(self, mut exp: Self, modulus: Self) -> Self {
                        if exp == 0 {
                            return 1;
                        }
                        let mut base = self;
                        let mut acc: Self = 1;

                        while exp > 1 {
                            if exp & 1 == 1 {
                                acc = acc.mul_rem(base, modulus) as Self;
                            }
                            exp >>= 1;
                            base = base.mul_rem(base, modulus) as Self;
                        }

                        acc.mul_rem(base, modulus) as Self
                    }

                    // https://en.wikipedia.org/wiki/Fermat%27s_little_theorem
                    // if p: prime && a % p > 0,
                    // then a^(p - 1) % p = 1,
                    // then a^(p - 2) % p = a^-1 % p.
                    #[inline]
                    fn modular_inv_prime(self, modulus: Self) -> Self {
                        assert!(self < modulus);
                        debug_assert!(self % modulus > 0);
                        self.modular_pow(modulus - 2, modulus)
                    }

                    // https://en.wikipedia.org/wiki/Euler%27s_theorem
                    // if gcd(a, p) == 1,
                    // then a^phi(p) % p = 1.
                    #[inline]
                    fn modular_inv_with_eulers_phi(
                        self,
                        modulus: Self,
                        modulus_eulers_phi: Self
                    ) -> Self {
                        debug_assert_eq!(gcd(self, modulus), 1);
                        self.modular_pow(modulus_eulers_phi - 1, modulus)
                    }

                    #[inline]
                    fn solve_ax_cong_b_mod_p(a: Self, b: Self, p: Self) -> Option<Self> {
                        let d = gcd(a, p);
                        (b % d == 0).then(|| {
                            let a1 = a / d;
                            let b1 = b / d;
                            let p1 = p / d;
                            a1.modular_inv_prime(p1) * b1
                        })
                    }
                }
            )*
        };
    }

    def!(u8 u16 u32 u64 u128 usize);
}

pub use div_rem_u128::*;
pub mod div_rem_u128 {
    #[cfg(target_arch = "x86_64")]
    #[inline]
    #[must_use]
    pub fn div_rem_u128(divident: u128, divisor: u64) -> (u64, u64) {
        let hi: u64 = (divident >> 64) as u64;
        debug_assert!(hi < divisor);
        let lo: u64 = divident as u64;
        let (mut quot, mut rem);
        unsafe {
            std::arch::asm!(
                "div {divisor}",
                divisor = in(reg) divisor,
                inout("rax") lo => quot,
                inout("rdx") hi => rem,
                options(pure, nomem, nostack)
            );
            (quot, rem)
        }
    }

    #[cfg(not(target_arch = "x86_64"))]
    #[inline]
    #[must_use]
    pub fn div_rem_u128(divident: u128, divisor: u64) -> (u64, u64) {
        let (quot, rem) = (divident / divisor as u128, divident % divisor as u128);
        (quot as u64, rem as u64)
    }
}

pub use hash_ext::*;
mod hash_ext {
    use core::hash::{Hash, Hasher};
    use std::collections::hash_map::DefaultHasher;

    #[inline]
    pub fn hash<T>(value: &T) -> u64
    where
        T: ?Sized + Hash,
    {
        let mut hash = DefaultHasher::new();
        value.hash(&mut hash);
        hash.finish()
    }
}
