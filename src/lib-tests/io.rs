use std::io::BufReader;

use crate::{DecoratableIterator, DecoratableTuple, Reader, ReaderExt, Word, Writer, WriterExt};

#[must_use]
pub fn re(value: &[u8]) -> Reader<BufReader<&[u8]>> {
    Reader::new(BufReader::new(value))
}

#[must_use]
pub const fn wr() -> Writer<Vec<u8>> {
    Writer::new(Vec::new())
}

#[test]
fn test_array_word() {
    let mut input = re(b" \n abcd bc HO \n  \t \n   \n     cd\nef ");
    assert_eq!(input.re::<Word<4>>(), Word(*b"abcd"));
    assert_eq!(input.re::<Word<4>>(), Word(*b"bc H"));
    assert_eq!(input.re::<Word<2>>(), Word(*b"O "));
    assert_eq!(input.re::<Word<2>>(), Word(*b"cd"));
    assert_eq!(format!("{}", Word(*b" abc ")), " abc ");
    assert_eq!(format!("{:?}", Word(*b" abc ")), "\" abc \"");
}

#[test]
#[should_panic(expected = "can not read string of length 5 from the line \"   abc \"")]
fn test_array_word_smaller() {
    let mut input = re(b" \n   abc \n abcdef");
    let v = input.re::<Word<5>>();
    assert_eq!(v.as_str(), " abc ");
}

#[test]
fn test_separated_tuple() {
    assert_eq!(wr().wo(().jo()).writer, b"");
    assert_eq!(wr().wo((12,).jo()).writer, b"12");
    assert_eq!(wr().wo((12, "ab").jo()).writer, b"12ab");
    assert_eq!(wr().wo((12, "ab", 34).jo()).writer, b"12ab34");
    assert_eq!(wr().wo(().wo()).writer, b"");
    assert_eq!(wr().wo((12,).wo()).writer, b"12");
    assert_eq!(wr().wo((12, "ab").wo()).writer, b"12 ab");
    assert_eq!(wr().wo((12, "ab", 34).wo()).writer, b"12 ab 34");
    assert_eq!(wr().wo(().li()).writer, b"");
    assert_eq!(wr().wo((12,).li()).writer, b"12");
    assert_eq!(wr().wo((12, "ab").li()).writer, b"12\nab");
    assert_eq!(wr().wo((12, "ab", 34).li()).writer, b"12\nab\n34");
    assert_eq!(wr().wo(().sep(", ")).writer, b"");
    assert_eq!(wr().wo((12,).sep(", ")).writer, b"12");
    assert_eq!(wr().wo((12, "ab").sep(", ")).writer, b"12, ab");
    assert_eq!(wr().wo((12, "ab", 34).sep(", ")).writer, b"12, ab, 34");
    assert_eq!(wr().wo(().decorate("[", ", ", "]")).writer, b"");
    assert_eq!(wr().wo((12,).decorate("[", ", ", "]")).writer, b"[12]");
    assert_eq!(
        wr().wo((12, "ab").decorate("[", ", ", "]")).writer,
        b"[12], [ab]"
    );
    assert_eq!(
        wr().wo((12, "ab", 34).decorate("[", ", ", "]")).writer,
        b"[12], [ab], [34]"
    );
    assert_eq!(wr().wo(((1, 2).wo(), (1, 2).wo()).jo()).writer, b"1 21 2");
}

#[test]
fn test_separated_iterator() {
    assert_eq!(wr().wo([0; 0].jo()).writer, b"");
    assert_eq!(wr().wo([12].jo()).writer, b"12");
    assert_eq!(wr().wo([12, 23].jo()).writer, b"1223");
    assert_eq!(wr().wo([12, 23, 34].jo()).writer, b"122334");
    assert_eq!(wr().wo([0; 0].wo()).writer, b"");
    assert_eq!(wr().wo([12].wo()).writer, b"12");
    assert_eq!(wr().wo([12, 23].wo()).writer, b"12 23");
    assert_eq!(wr().wo([12, 23, 34].wo()).writer, b"12 23 34");
    assert_eq!(wr().wo([0; 0].li()).writer, b"");
    assert_eq!(wr().wo([12].li()).writer, b"12");
    assert_eq!(wr().wo([12, 23].li()).writer, b"12\n23");
    assert_eq!(wr().wo([12, 23, 34].li()).writer, b"12\n23\n34");
    assert_eq!(wr().wo([0; 0].sep(", ")).writer, b"");
    assert_eq!(wr().wo([12].sep(", ")).writer, b"12");
    assert_eq!(wr().wo([12, 23].sep(", ")).writer, b"12, 23");
    assert_eq!(wr().wo([12, 23, 34].sep(", ")).writer, b"12, 23, 34");
    assert_eq!(wr().wo([0; 0].decorate(", ")).writer, b"");
    assert_eq!(wr().wo([12].decorate(", ")).writer, b"12");
    assert_eq!(wr().wo([12, 23].decorate(", ")).writer, b"12, 23");
    assert_eq!(wr().wo([12, 23, 34].decorate(", ")).writer, b"12, 23, 34");
    assert_eq!(wr().wo([[1, 2].wo(), [1, 2].wo()].jo()).writer, b"1 21 2");
}
