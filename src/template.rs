pub fn main() {
    let mut re = Reader::new(BufReader::new(stdin()));
    let mut wr = Writer::new(BufWriter::new(stdout()));
    solver(&mut re, &mut wr);
}

pub fn solver<R: BufRead, W: Write>(re: &mut Reader<R>, wr: &mut Writer<W>) {
    let t: usize = re.re();
    'main: for _ in 0..t {
        let n: usize = re.re();
        let a: Vec<i64> = vec![re.re(); n];
        let ans = a.iter().sum::<i64>();
        wr.li(ans);
    }
}

pub const EXAMPLES: &str = r####"
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
pub const EXAMPLE_TIMEOUT: Duration = Duration::from_secs(1);

pub const TESTS: &str = r####"
----
====
"####;
pub const TESTS_TIMEOUT: Duration = Duration::from_secs(1);

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Debug)]
pub struct Preset {
    len: usize,
}
pub const IS_INTERACTOR_ENABLED: bool = false;
pub const INTERACTIVE_TIMEOUT: Duration = Duration::from_secs(1);

pub fn presets_buidler() -> impl Iterator<Item = Preset> {
    [1, 999].into_iter().map(|len| Preset { len })
}

pub fn interactor<R: BufRead, W: Write>(preset: &Preset, re: &mut Reader<R>, wr: &mut Writer<W>) {
    wr.li(1);
    wr.li(preset.len);
    wr.li(repeat_n(999_999_999, preset.len).wo());
    wr.fl();
    let ans: u64 = re.re();
    let exp: u64 = 999_999_999 * preset.len as u64;
    assert_eq!(ans, exp);
}

use prelude::*;
mod prelude {
    pub use core::array;
    pub use core::borrow::{Borrow, BorrowMut};
    pub use core::cell::RefCell;
    pub use core::cmp::Ordering::{Equal, Greater, Less};
    pub use core::cmp::{max, min};
    pub use core::convert::Infallible;
    pub use core::fmt::{Debug, Display, Formatter, Result as FmtResult};
    pub use core::iter::{
        empty, from_fn as iter_from_fn, once, once_with, repeat, repeat_n, repeat_with, successors,
    };
    pub use core::marker::PhantomData;
    pub use core::mem::{replace, swap, take};
    pub use core::str::{from_utf8, FromStr};
    pub use core::time::Duration;
    pub use std::collections::btree_map::Entry as BTreeMapEntry;
    pub use std::collections::hash_map::Entry as HashMapEntry;
    pub use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque};
    pub use std::io::{
        sink, stderr, stdin, stdout, BufRead, BufReader, BufWriter, Error as IoError,
        ErrorKind as IoErrorKind, Result as IoResult, Stderr, Stdin, Stdout, Write,
    };
    pub use std::sync::mpsc::{Receiver, Sender};
    pub use std::sync::Arc;
}

use macros::*;
mod macros {
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

    #[macro_export]
    macro_rules! for_each {
        ( for $pat:pat in [ $( $expr:expr ),* $(,)? ] $body:block ) => {
            $( { let $pat = $expr; $body; } )*
        };
    }
}

// IO and formatting libraries.

use io::*;
mod io {
    use std::io::{stdin, stdout, BufReader, BufWriter, Stdin, Stdout};

    #[derive(Clone, Debug)]
    pub struct Io<R, W> {
        pub reader: R,
        pub writer: W,
        pub is_dirty: bool,
    }

    impl<R, W> Io<R, W> {
        #[inline]
        pub const fn new(reader: R, writer: W) -> Self {
            Self {
                reader,
                writer,
                is_dirty: false,
            }
        }
    }

    impl Io<BufReader<Stdin>, BufWriter<Stdout>> {
        #[inline]
        pub fn from_stdio() -> Self {
            Self::new(BufReader::new(stdin()), BufWriter::new(stdout()))
        }
    }
}

use reader::*;
mod reader {
    use core::ops::Not;
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

use writer::*;
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

use readable::*;
mod readable {
    use core::hash::{BuildHasher, Hash};
    use core::num::{
        NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
        NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
    };
    use core::str::FromStr;
    use core::{array, iter};
    use std::collections::{
        BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque,
    };
    use std::io::BufRead;

    use super::Reader;

    pub trait Readable<'a, C> {
        fn read_from<R: BufRead>(reader: &'a mut Reader<R>, context: &C) -> Self;
    }

    macro_rules! def_impl {
        ( $( $ty:ty ),* $(,)? ) => {
            $(
                impl<'a> Readable<'a, ()> for $ty {
                    #[inline]
                    fn read_from<R: BufRead>(reader: &'a mut Reader<R>, &(): &()) -> Self {
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
        bool, f32, f64, char, String
    }

    impl<'a> Readable<'a, ()> for &'a str {
        #[inline]
        fn read_from<R: BufRead>(reader: &'a mut Reader<R>, &(): &()) -> Self {
            reader.read_word().unwrap()
        }
    }

    impl<'a> Readable<'a, usize> for &'a str {
        #[inline]
        fn read_from<R: BufRead>(reader: &'a mut Reader<R>, &len: &usize) -> Self {
            while reader.remaining().is_empty() {
                reader.cache_next_line().unwrap();
            }
            let line = &reader.line[reader.offset..];
            assert!(
                line.len() >= len,
                "can not read string of length {len} from the line {line:?}"
            );
            reader.offset += len;
            &line[0..len]
        }
    }

    impl<'a, T, const N: usize, C> Readable<'a, C> for [T; N]
    where
        T: for<'b> Readable<'b, C>,
    {
        #[inline]
        fn read_from<R: BufRead>(reader: &'a mut Reader<R>, context: &C) -> Self {
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
                fn read_from<R: BufRead>(reader: &$lt mut Reader<R>, &len: &usize) -> Self {
                    iter::repeat_with(|| <$in>::read_from(reader, &())).take(len).collect()
                }
            }

            impl<$lt, $ctx $( $args )*> Readable<$lt, (usize, $ctx)> for $out
            where
                $in: for<'b> Readable<'b, C>,
                $( $( $bounds )* )?
            {
                #[inline]
                fn read_from<R: BufRead>(reader: &$lt mut Reader<R>, context: &(usize, $ctx)) -> Self {
                    iter::repeat_with(|| <$in>::read_from(reader, &context.1)).take(context.0).collect()
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
                fn read_from<R: BufRead>(reader: &'a mut Reader<R>, (): &()) -> Self {
                    ( $( <$type>::read_from(reader, &()), )* )
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
                    context: &( $( $context, )* )
                ) -> Self {
                    ( $( <$type>::read_from(reader, &context.$field), )* )
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

use writable::*;
mod writable {
    use core::num::{
        NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
        NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
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
        char, String
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
    use std::io::BufRead;

    use super::{Readable, Reader};

    pub trait ReaderExt {
        fn re<'a, T>(&'a mut self) -> T
        where
            T: Readable<'a, ()>;

        fn rec<'a, T, C>(&'a mut self, context: &C) -> T
        where
            T: Readable<'a, C>;
    }

    impl<R: BufRead> ReaderExt for Reader<R> {
        #[inline]
        fn re<'a, T>(&'a mut self) -> T
        where
            T: Readable<'a, ()>,
        {
            T::read_from(self, &())
        }

        #[inline]
        fn rec<'a, T, C>(&'a mut self, context: &C) -> T
        where
            T: Readable<'a, C>,
        {
            T::read_from(self, context)
        }
    }
}

pub use writer_ext::*;
mod writer_ext {
    use std::io::Write;

    use super::{Writable, Writer};

    pub trait WriterExt {
        fn ok(&mut self);
        fn fl(&mut self);
        fn ln(&mut self);
        fn li<T: Writable>(&mut self, value: T);
        fn jo<T: Writable>(&mut self, value: T) -> &mut Self;
        fn wo<T: Writable>(&mut self, value: T) -> &mut Self;
        fn ask(&mut self) -> &mut Self;
        fn ans(&mut self) -> &mut Self;
        fn yes(&mut self) -> &mut Self;
        fn no(&mut self) -> &mut Self;
        fn yes_no(&mut self, value: bool) -> &mut Self;
    }

    impl<W: Write> WriterExt for Writer<W> {
        #[inline]
        fn ok(&mut self) {
            _ = self;
        }

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
        fn li<T: Writable>(&mut self, value: T) {
            self.wo(value).ln();
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
            if value {
                self.yes()
            } else {
                self.no()
            }
        }
    }
}

use tuple_ext::*;
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

use decorated_tuple::*;
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

use decorated_iterator::*;
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

// Auxiliary libraries.

use array_ext::*;
mod array_ext {
    pub trait ArrayExt {
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

use dedup_count::*;
mod dedup_count {
    use core::iter::FusedIterator;

    #[derive(Clone, Debug)]
    pub struct DedupCountIter<T, I> {
        iter: I,
        prev: Option<(T, usize)>,
    }

    impl<T, I> DedupCountIter<T, I> {
        #[inline]
        pub const fn new(iter: I) -> Self {
            Self { iter, prev: None }
        }
    }

    impl<T, I> Iterator for DedupCountIter<T, I>
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

        #[inline]
        fn dedup_count(self) -> Self::Output {
            DedupCountIter::new(self.into_iter())
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
        I: IntoIterator<Item = T>,
    {
        #[inline]
        fn into_vec(self) -> Vec<T> {
            self.into_iter().collect()
        }
    }
}

use math_ext::*;
mod math_ext {
    pub trait IntegerExt {
        type Widen;
    }

    pub trait UnsignedExt: IntegerExt {
        fn mul_div(self, mul: Self, div: Self) -> Self::Widen;
        fn mul_rem(self, mul: Self, rem: Self) -> Self::Widen;

        fn gcd(self, other: Self) -> Self;
        fn lcm(self, other: Self) -> Self::Widen;
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
                impl UnsignedExt for $ty {
                    #[allow(trivial_numeric_casts, clippy::cast_lossless)]
                    #[inline]
                    fn mul_div(self, mul: Self, div: Self) -> Self::Widen {
                        self as Self::Widen * mul as Self::Widen / div as Self::Widen
                    }

                    #[allow(trivial_numeric_casts, clippy::cast_lossless)]
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
                }
            )*
        };
    }

    def!(u8 u16 u32 u64 u128 usize);
}

use factorial::*;
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

use primes::*;
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
        pub fn new(max: usize) -> Self {
            let len = max + 1;
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

        /// 2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, ...
        pub const fn iter(&self) -> PrimesIter<'_> {
            PrimesIter::new(self, 0)
        }

        pub const fn iter_from(&self, from: usize) -> PrimesIter<'_> {
            PrimesIter::new(self, from)
        }

        /// `factorize(2 * 3 * 11 * 103) = [2, 3, 11, 103]`
        pub fn factorize(&self, value: usize) -> PrimesFactorizeIter<'_> {
            assert!(value < self.len);
            PrimesFactorizeIter::new(self, value)
        }

        /// `factorize(2 * 3 * 11 * 103) = 4`
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
        pub const fn new(sieve: &'a Primes, from: usize) -> Self {
            Self(sieve, if from < 3 { 0 } else { from / 2 })
        }
    }

    impl Iterator for PrimesIter<'_> {
        type Item = usize;

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
        pub const fn new(sieve: &'a Primes, value: usize) -> Self {
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

use modular_ops::*;
mod modular_ops {
    use super::{IntegerExt, UnsignedExt};

    pub trait ModularExt: IntegerExt {
        fn modular_mul(self, other: Self, modulus: Self) -> Self;
        fn modular_pow(self, exp: Self, modulus: Self) -> Self;
        fn modular_inv_prime(self, modulus: Self) -> Self;
    }

    macro_rules! def {
        ( $( $ty:tt )* ) => {
            $(
                impl ModularExt for $ty {
                    #[allow(
                        trivial_numeric_casts,
                        clippy::cast_lossless,
                        clippy::cast_possible_truncation
                    )]
                    #[inline]
                    fn modular_mul(self, other: Self, modulus: Self) -> Self {
                        (self as Self::Widen * other as Self::Widen)
                            .rem_euclid(modulus as Self::Widen) as Self
                    }

                    #[allow(trivial_numeric_casts, clippy::cast_possible_truncation)]
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

                    #[allow(trivial_numeric_casts, clippy::cast_possible_truncation)]
                    #[inline]
                    fn modular_inv_prime(self, modulus: Self) -> Self {
                        assert!(self < modulus);
                        self.modular_pow(modulus - 2, modulus)
                    }
                }
            )*
        };
    }

    def!(u8 u16 u32 u64 u128 usize);
}

use div_rem_u128::*;
pub mod div_rem_u128 {
    #[cfg(target_arch = "x86_64")]
    #[inline]
    #[must_use]
    pub fn div_rem_u128(divident: u128, divisor: u64) -> (u64, u64) {
        let hi: u64 = (divident >> 64) as u64;
        debug_assert!(hi < divisor);
        #[allow(clippy::cast_possible_truncation)]
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

    pub fn hash<T>(value: &T) -> u64
    where
        T: Hash,
    {
        let mut hash = DefaultHasher::new();
        value.hash(&mut hash);
        hash.finish()
    }
}
