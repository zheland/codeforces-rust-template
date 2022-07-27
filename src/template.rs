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
use core::fmt::{Debug, Display};
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
    io.wrln(a);
    io.wrln(b);
    io.wrln(a + b);
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

#[derive(Clone, Debug)]
pub struct Preset {}

#[allow(unused_variables)]
pub fn interactor<I: ReaderExt + WriterExt>(io: &mut I, preset: Preset) {
    io.wrln(1i32);
    io.wrln(2i32);
    io.fl();
    let a: i32 = io.re();
    let b: i32 = io.re();
    let c: i32 = io.re();
    assert_eq!(a, 1, "{preset:?}");
    assert_eq!(b, 2, "{preset:?}");
    assert_eq!(c, 3, "{preset:?}");
}

/*
    d! { exec-on-debug }
    Ascii<T = Vec<u8>>
    Dec<T = Vec<u8>>
    Io::{re}
    Io::{wr, ln, fl, wrln, wrlnfl, wr2, ask, ans}
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

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Ascii<T = Vec<u8>>(pub T);

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AsciiWord<T = Vec<u8>>(pub T);

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Dec<T = Vec<u8>>(pub T);

#[derive(Clone, Debug)]
pub struct Io<R, W> {
    pub reader: WordReader<R>,
    pub writer: WordWriter<W>,
}

impl<R, W> Io<R, W> {
    pub fn new(reader: R, writer: W) -> Self {
        Self {
            reader: WordReader::new(reader),
            writer: WordWriter::new(writer),
        }
    }
}

impl Io<BufReader<Stdin>, BufWriter<Stdout>> {
    pub fn from_stdio() -> Self {
        let reader = WordReader::new(BufReader::new(stdin()));
        let writer = WordWriter::new(BufWriter::new(stdout()));
        Self { reader, writer }
    }
}

pub trait ReaderExt {
    fn re<'a, T: Readable<'a>>(&'a mut self) -> T;
}

pub trait WriterExt {
    fn wr<T: Writable>(&mut self, value: T);

    fn jo<T: Writable>(&mut self, value: T);

    fn ln(&mut self);

    fn fl(&mut self);

    fn wrln<T: Writable>(&mut self, value: T) {
        self.wr(value);
        self.ln();
    }

    fn joln<T: Writable>(&mut self, value: T) {
        self.jo(value);
        self.ln();
    }

    fn lnfl(&mut self) {
        self.ln();
        self.fl();
    }

    fn wrlnfl<T: Writable>(&mut self, value: T) {
        self.wr(value);
        self.ln();
        self.fl();
    }

    fn wr2<T: Writable>(&mut self, value: T) -> &mut Self {
        self.wr(value);
        self
    }

    fn jo2<T: Writable>(&mut self, value: T) -> &mut Self {
        self.jo(value);
        self
    }

    fn ln2(&mut self) -> &mut Self {
        self.ln();
        self
    }

    fn fl2(&mut self) -> &mut Self {
        self.fl();
        self
    }

    fn wrln2<T: Writable>(&mut self, value: T) -> &mut Self {
        self.wrln(value);
        self
    }

    fn joln2<T: Writable>(&mut self, value: T) -> &mut Self {
        self.joln(value);
        self
    }

    fn lnfl2(&mut self) -> &mut Self {
        self.lnfl();
        self
    }

    fn wrlnfl2<T: Writable>(&mut self, value: T) -> &mut Self {
        self.wrlnfl(value);
        self
    }

    fn ask(&mut self) {
        self.wr('?');
    }

    fn ans(&mut self) {
        self.wr('!');
    }
}

impl<R: BufRead> ReaderExt for WordReader<R> {
    fn re<'a, T: Readable<'a>>(&'a mut self) -> T {
        T::read(self)
    }
}

impl<W: Write> WriterExt for WordWriter<W> {
    fn wr<T: Writable>(&mut self, value: T) {
        value.write(self);
    }

    fn jo<T: Writable>(&mut self, value: T) {
        let mut writer = CharWriter::new(&mut self.writer);
        value.write(&mut writer);
        self.is_seperator_needed = true;
    }

    fn ln(&mut self) {
        self.write_ln().unwrap();
    }

    fn fl(&mut self) {
        self.flush().unwrap();
    }
}

impl<R: BufRead, W> ReaderExt for Io<R, W> {
    fn re<'a, T: Readable<'a>>(&'a mut self) -> T {
        self.reader.re()
    }
}

impl<R, W: Write> WriterExt for Io<R, W> {
    fn wr<T: Writable>(&mut self, value: T) {
        self.writer.wr(value);
    }

    fn jo<T: Writable>(&mut self, value: T) {
        self.writer.jo(value);
    }

    fn ln(&mut self) {
        self.writer.ln();
    }

    fn fl(&mut self) {
        self.writer.fl();
    }
}

#[allow(single_use_lifetimes)]
pub trait Readable<'a>: Sized {
    fn read<R: Reader>(reader: &'a mut R) -> Self;
}

pub trait Writable {
    fn write<W: Writer>(&self, writer: &mut W);
}

pub trait Reader {
    fn goto_word(&mut self) -> IoResult<()>;
    fn read_word(&mut self) -> IoResult<&str>;
    fn read_char(&mut self) -> IoResult<char>;
    fn read_ascii_char(&mut self) -> IoResult<u8>;
    fn read_ascii_chars<const N: usize>(&mut self) -> IoResult<[u8; N]>;
}

pub trait Writer {
    fn write_word<T: Display>(&mut self, value: T) -> IoResult<()>;
    fn start_word_chars(&mut self) -> IoResult<()>;
    fn write_word_chars<T: Display>(&mut self, value: T) -> IoResult<()>;
}

#[derive(Clone, Debug)]
pub struct WordReader<R> {
    reader: R,
    line: String,
    offset: usize,
}

#[derive(Clone, Debug)]
pub struct WordWriter<W> {
    writer: W,
    is_seperator_needed: bool,
}

#[derive(Clone, Debug)]
pub struct CharWriter<W> {
    writer: W,
}

impl<R> WordReader<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            line: String::new(),
            offset: 0,
        }
    }

    pub fn as_reader(&self) -> &R {
        &self.reader
    }

    pub fn into_reader(self) -> R {
        self.reader
    }
}

impl<R: BufRead> Reader for WordReader<R> {
    fn goto_word(&mut self) -> IoResult<()> {
        loop {
            if self.offset >= self.line.len() {
                self.line.clear();
                let len = self.reader.read_line(&mut self.line)?;
                if len == 0 {
                    return Err(IoError::from(IoErrorKind::UnexpectedEof));
                }
                self.offset = 0;
            } else if self.line.as_bytes()[self.offset].is_ascii_whitespace() {
                self.offset += 1;
            } else {
                return Ok(());
            }
        }
    }

    #[track_caller]
    fn read_word(&mut self) -> IoResult<&str> {
        self.goto_word()?;
        let line = self.line.as_bytes();
        let start = self.offset;
        let mut end = start + 1;
        while end < self.line.len() && !line[end].is_ascii_whitespace() {
            end += 1;
        }
        self.offset = end + 1;
        let word = unsafe { std::str::from_utf8_unchecked(&line[start..end]) };
        Ok(word)
    }

    #[track_caller]
    fn read_char(&mut self) -> IoResult<char> {
        self.goto_word()?;
        let line = self.line.as_bytes();
        let ch = line[self.offset] as char;
        self.offset += 1;
        Ok(ch)
    }

    #[track_caller]
    fn read_ascii_char(&mut self) -> IoResult<u8> {
        self.goto_word()?;
        let line = self.line.as_bytes();
        let ch = line[self.offset];
        self.offset += 1;
        Ok(ch)
    }

    #[track_caller]
    fn read_ascii_chars<const N: usize>(&mut self) -> IoResult<[u8; N]> {
        self.goto_word()?;
        let line = self.line.as_bytes();
        let chars = line[self.offset..self.offset + N].try_into().unwrap();
        self.offset += N;
        Ok(chars)
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

impl<W> CharWriter<W> {
    pub fn new(writer: W) -> Self {
        Self { writer }
    }

    pub fn as_writer(&self) -> &W {
        &self.writer
    }

    pub fn into_writer(self) -> W {
        self.writer
    }
}

impl<R: Write> Writer for WordWriter<R> {
    fn write_word<T: Display>(&mut self, value: T) -> IoResult<()> {
        if self.is_seperator_needed {
            write!(self.writer, " {}", value)?;
        } else {
            write!(self.writer, "{}", value)?;
            self.is_seperator_needed = true;
        }
        Ok(())
    }

    fn start_word_chars(&mut self) -> IoResult<()> {
        if self.is_seperator_needed {
            write!(self.writer, " ")?;
        } else {
            self.is_seperator_needed = true;
        }
        Ok(())
    }

    fn write_word_chars<T: Display>(&mut self, value: T) -> IoResult<()> {
        write!(self.writer, "{}", value)?;
        Ok(())
    }
}

impl<R: Write> Writer for CharWriter<R> {
    fn write_word<T: Display>(&mut self, value: T) -> IoResult<()> {
        write!(self.writer, "{}", value)?;
        Ok(())
    }

    fn start_word_chars(&mut self) -> IoResult<()> {
        Ok(())
    }

    fn write_word_chars<T: Display>(&mut self, value: T) -> IoResult<()> {
        write!(self.writer, "{}", value)?;
        Ok(())
    }
}

impl<R: Write> WordWriter<R> {
    pub fn write_ln(&mut self) -> IoResult<()> {
        writeln!(self.writer)?;
        self.is_seperator_needed = false;
        Ok(())
    }

    pub fn flush(&mut self) -> IoResult<()> {
        self.writer.flush()
    }
}

// ========

impl<'a> Readable<'a> for &'a str {
    fn read<R: Reader>(reader: &'a mut R) -> Self {
        reader.read_word().unwrap()
    }
}

impl Writable for &str {
    fn write<W: Writer>(&self, writer: &mut W) {
        writer.write_word(self).unwrap();
    }
}

impl<'a> Readable<'a> for String {
    fn read<R: Reader>(reader: &'a mut R) -> Self {
        reader.read_word().unwrap().to_owned()
    }
}

impl Writable for String {
    fn write<W: Writer>(&self, writer: &mut W) {
        writer.write_word(self).unwrap();
    }
}

impl<'a> Readable<'a> for char {
    fn read<R: Reader>(reader: &'a mut R) -> Self {
        reader.read_char().unwrap().to_owned()
    }
}

impl Writable for char {
    fn write<W: Writer>(&self, writer: &mut W) {
        writer.write_word_chars(self).unwrap();
    }
}

macro_rules! def {
    ( $( $ty:ty ),* $(,)? ) => {
        $(
            #[allow(single_use_lifetimes)]
            impl<'a> Readable<'a> for $ty {
                #[track_caller]
                fn read<R: Reader>(reader: &'a mut R) -> Self {
                    FromStr::from_str(reader.read_word().unwrap()).unwrap()
                }
            }

            impl Writable for $ty {
                #[track_caller]
                fn write<W: Writer>(&self, writer: &mut W) {
                    writer.write_word(self).unwrap();
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
        def!(@impl ()( $($field: $type),* ));
    };
    ( @impl ( $($field:tt: $type:ident),* ) ) => {
        #[allow(single_use_lifetimes)]
        impl< 'a, $($type),* > Readable<'a> for ( $($type,)* )
        where
            $($type: 'static + for<'b> Readable<'b>,)*
        {
            #[track_caller]
            fn read<R: Reader>(reader: &'a mut R) -> Self {
                ( $($type::read(reader),)* )
            }
        }

        impl< $($type),* > Writable for ( $($type,)* )
        where
            $($type: Writable,)*
        {
            #[track_caller]
            fn write<W: Writer>(&self, writer: &mut W) {
                $(
                    self.$field.write(writer);
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

impl<T: Writable, const N: usize> Writable for [T; N] {
    #[track_caller]
    fn write<W: Writer>(&self, writer: &mut W) {
        for value in self {
            value.write(writer);
        }
    }
}

#[allow(single_use_lifetimes)]
impl<'a> Readable<'a> for Ascii<&'a [u8]> {
    #[track_caller]
    fn read<R: Reader>(reader: &'a mut R) -> Self {
        let value = reader.read_word().unwrap();
        Ascii(value.as_bytes())
    }
}

#[allow(single_use_lifetimes)]
impl<'a, const N: usize> Readable<'a> for Ascii<[u8; N]> {
    #[track_caller]
    fn read<R: Reader>(reader: &'a mut R) -> Self {
        let ascii = reader.read_ascii_chars::<N>().unwrap();
        Ascii(ascii)
    }
}

#[allow(single_use_lifetimes)]
impl<'a, const N: usize> Readable<'a> for AsciiWord<[u8; N]> {
    #[track_caller]
    fn read<R: Reader>(reader: &'a mut R) -> Self {
        let value = reader.read_word().unwrap();
        assert!(value.len() <= N, "the word length exceeds the ascii length");
        let mut ascii = [0; N];
        ascii[0..value.len()].copy_from_slice(value.as_bytes());
        AsciiWord(ascii)
    }
}

#[allow(single_use_lifetimes)]
impl<'a> Readable<'a> for Ascii<Vec<u8>> {
    #[track_caller]
    fn read<R: Reader>(reader: &'a mut R) -> Self {
        let value = reader.read_word().unwrap();
        Ascii(value.as_bytes().to_owned())
    }
}

impl<T: Borrow<[u8]>> Writable for Ascii<T> {
    #[track_caller]
    fn write<W: Writer>(&self, writer: &mut W) {
        let value = from_utf8(self.0.borrow()).unwrap();
        writer.write_word(value).unwrap();
    }
}

#[allow(single_use_lifetimes)]
impl<'a, const N: usize> Readable<'a> for Dec<[u8; N]> {
    #[track_caller]
    fn read<R: Reader>(reader: &'a mut R) -> Self {
        let value = reader.read_word().unwrap();
        assert_eq!(
            value.len(),
            N,
            "the length of the word does not match the length of dec"
        );
        let mut dec = [0; N];
        dec.copy_from_slice(value.as_bytes());
        for value in &mut dec {
            assert!(*value >= b'0' && *value <= b'9');
            *value -= b'0';
        }
        Dec(dec)
    }
}

#[allow(single_use_lifetimes)]
impl<'a> Readable<'a> for Dec<Vec<u8>> {
    #[track_caller]
    fn read<R: Reader>(reader: &'a mut R) -> Self {
        let value = reader.read_word().unwrap();
        Dec(value
            .bytes()
            .map(|value| {
                assert!((b'0'..=b'9').contains(&value));
                value - b'0'
            })
            .collect())
    }
}

impl<T: Borrow<[u8]>> Writable for Dec<T> {
    #[track_caller]
    fn write<W: Writer>(&self, writer: &mut W) {
        writer.start_word_chars().unwrap();
        for &ch in self.0.borrow() {
            assert!(ch <= 9);
            writer.write_word_chars((ch + b'0') as char).unwrap();
        }
    }
}
