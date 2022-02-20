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
    self, sink, stderr, stdin, stdout, BufRead, BufReader, BufWriter, Stderr, Stdout, Write,
};
use std::sync::Arc;

macro_rules! de {
    ( $($arg:tt)* ) => ( if cfg!(debug_assertions) { $($arg)* } )
}

pub trait Problem {
    fn solve<R: BufRead, W: Write>(read: R, write: W);
}

#[derive(Clone, Copy, Debug)]
pub struct Main;

impl Problem for Main {
    fn solve<R: BufRead, W: Write>(read: R, write: W) {
        let mut re = Reader::new(read);
        let mut wr = Writer::new(write);

        let t: usize = re.re();
        'main: for _ in 0..t {
            let n: i64 = re.re();
            let m: i64 = re.re();
            // let n = re!(usize);
            // let a = re!([i64; n]);
            wr.wr(n + m);
        }
    }
}

pub fn main() {
    let reader = stdin();
    let mut writer = stdout();
    Main::solve(BufReader::new(reader), &mut writer);
    writer.flush().unwrap();
}

#[derive(Clone, Debug)]
pub struct Reader<R> {
    read: R,
    line: String,
    offset: usize,
}

#[derive(Clone, Debug)]
pub struct Writer<W> {
    write: W,
    is_seperator_needed: bool,
}

pub trait ReaderExt {
    fn re<'a, T>(&'a mut self) -> T
    where
        T: Readable<'a>;
}

pub trait WriterExt {
    fn wr<T>(&mut self, value: T)
    where
        T: Writable;

    fn ln(&mut self);

    fn wr2<T>(&mut self, value: T) -> &mut Self
    where
        T: Writable,
    {
        self.wr(value);
        self
    }

    fn ln2(&mut self) -> &mut Self {
        self.ln();
        self
    }
}

#[allow(single_use_lifetimes)]
pub trait Readable<'a>: Sized {
    fn read<R: BufRead>(reader: &'a mut Reader<R>) -> Self;
}

pub trait Writable {
    fn write<W: Write>(&self, writer: &mut Writer<W>);
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Ascii<T = Vec<u8>>(pub T);

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Dec<T = Vec<u8>>(pub T);

impl<R> Reader<R> {
    pub fn new(read: R) -> Self {
        Self {
            read,
            line: String::new(),
            offset: 0,
        }
    }

    pub fn into_read(self) -> R {
        self.read
    }

    pub fn to_read(&self) -> &R {
        &self.read
    }
}

impl<R: BufRead> Reader<R> {
    #[track_caller]
    fn goto_word(&mut self) -> io::Result<()> {
        loop {
            if self.offset >= self.line.len() {
                self.line.clear();
                let len = self.read.read_line(&mut self.line)?;
                if len == 0 {
                    return Err(io::Error::from(io::ErrorKind::UnexpectedEof));
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
    pub fn read_word(&mut self) -> io::Result<&str> {
        self.goto_word()?;
        let line = self.line.as_bytes();
        let start = self.offset;
        let mut end = start + 1;
        while end < self.line.len() && !line[end].is_ascii_whitespace() {
            end += 1;
        }
        self.offset = end + 1;
        let word = unsafe { std::str::from_utf8_unchecked(&line[start..end]) };
        return Ok(word);
    }
}

impl<W> Writer<W> {
    pub fn new(write: W) -> Self {
        Self {
            write,
            is_seperator_needed: false,
        }
    }

    pub fn into_write(self) -> W {
        self.write
    }

    pub fn to_write(&self) -> &W {
        &self.write
    }
}

impl<R: Write> Writer<R> {
    #[track_caller]
    pub fn write_word<T: Display>(&mut self, value: T) -> io::Result<()> {
        if self.is_seperator_needed {
            write!(self.write, " {}", value)?;
        } else {
            write!(self.write, "{}", value)?;
            self.is_seperator_needed = true;
        }
        Ok(())
    }

    #[track_caller]
    pub fn start_word_chars(&mut self) -> io::Result<()> {
        if self.is_seperator_needed {
            write!(self.write, " ")?;
        } else {
            self.is_seperator_needed = true;
        }
        Ok(())
    }

    #[track_caller]
    pub fn write_word_chars<T: Display>(&mut self, value: T) -> io::Result<()> {
        write!(self.write, "{}", value)?;
        self.is_seperator_needed = true;
        Ok(())
    }

    #[track_caller]
    pub fn write_ln(&mut self) -> io::Result<()> {
        write!(self.write, "\n")?;
        self.is_seperator_needed = false;
        Ok(())
    }
}

impl<R: BufRead> ReaderExt for Reader<R> {
    #[track_caller]
    fn re<'a, T>(&'a mut self) -> T
    where
        T: Readable<'a>,
    {
        T::read(self)
    }
}

impl<W: Write> WriterExt for Writer<W> {
    #[track_caller]
    fn wr<T>(&mut self, value: T)
    where
        T: Writable,
    {
        value.write(self);
    }

    fn ln(&mut self) {
        self.write_ln().unwrap();
    }
}

impl<'a> Readable<'a> for &'a str {
    fn read<R: BufRead>(reader: &'a mut Reader<R>) -> Self {
        reader.read_word().unwrap()
    }
}

impl Writable for &str {
    fn write<W: Write>(&self, writer: &mut Writer<W>) {
        writer.write_word(self).unwrap()
    }
}

macro_rules! def {
    ( $( $ty:ty ),* $(,)? ) => {
        $(
            #[allow(single_use_lifetimes)]
            impl<'a> Readable<'a> for $ty {
                #[track_caller]
                fn read<R: BufRead>(reader: &'a mut Reader<R>) -> Self {
                    FromStr::from_str(reader.read_word().unwrap()).unwrap()
                }
            }

            impl Writable for $ty {
                #[track_caller]
                fn write<W: Write>(&self, writer: &mut Writer<W>) {
                    writer.write_word(self).unwrap()
                }
            }
        )*
    };
}
def! {
    u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize,
    bool, char, String, f32, f64,
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
            fn read<R: BufRead>(reader: &'a mut Reader<R>) -> Self {
                ( $($type::read(reader),)* )
            }
        }

        impl< $($type),* > Writable for ( $($type,)* )
        where
            $($type: Writable,)*
        {
            #[track_caller]
            fn write<W: Write>(&self, writer: &mut Writer<W>) {
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
    fn read<R: BufRead>(reader: &'a mut Reader<R>) -> Self {
        [(); N].map(|()| T::read(reader))
    }
}

impl<T: Writable, const N: usize> Writable for [T; N] {
    #[track_caller]
    fn write<W: Write>(&self, writer: &mut Writer<W>) {
        for value in self {
            value.write(writer);
        }
    }
}

#[allow(single_use_lifetimes)]
impl<'a> Readable<'a> for Ascii<&'a [u8]> {
    #[track_caller]
    fn read<R: BufRead>(reader: &'a mut Reader<R>) -> Self {
        let value = reader.read_word().unwrap();
        Ascii(value.as_bytes())
    }
}

#[allow(single_use_lifetimes)]
impl<'a, const N: usize> Readable<'a> for Ascii<[u8; N]> {
    #[track_caller]
    fn read<R: BufRead>(reader: &'a mut Reader<R>) -> Self {
        let value = reader.read_word().unwrap();
        assert!(value.len() <= N, "the word length exceeds the ascii length");
        let mut ascii = [0; N];
        ascii[0..value.len()].copy_from_slice(value.as_bytes());
        Ascii(ascii)
    }
}

#[allow(single_use_lifetimes)]
impl<'a> Readable<'a> for Ascii<Vec<u8>> {
    #[track_caller]
    fn read<R: BufRead>(reader: &'a mut Reader<R>) -> Self {
        let value = reader.read_word().unwrap();
        Ascii(value.as_bytes().to_owned())
    }
}

impl<T: Borrow<[u8]>> Writable for Ascii<T> {
    #[track_caller]
    fn write<W: Write>(&self, writer: &mut Writer<W>) {
        let value = from_utf8(self.0.borrow()).unwrap();
        writer.write_word(value).unwrap();
    }
}

#[allow(single_use_lifetimes)]
impl<'a, const N: usize> Readable<'a> for Dec<[u8; N]> {
    #[track_caller]
    fn read<R: BufRead>(reader: &'a mut Reader<R>) -> Self {
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
    fn read<R: BufRead>(reader: &'a mut Reader<R>) -> Self {
        let value = reader.read_word().unwrap();
        Dec(value
            .bytes()
            .map(|value| {
                assert!(value >= b'0' && value <= b'9');
                value - b'0'
            })
            .collect())
    }
}

impl<T: Borrow<[u8]>> Writable for Dec<T> {
    #[track_caller]
    fn write<W: Write>(&self, writer: &mut Writer<W>) {
        writer.start_word_chars().unwrap();
        for &ch in self.0.borrow() {
            assert!(ch <= 9);
            writer.write_word_chars((ch as u8 + b'0') as char).unwrap();
        }
    }
}
