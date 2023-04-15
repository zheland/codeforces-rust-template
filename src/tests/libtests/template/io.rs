use crate::tests::{re, wr};
use crate::{ArrayWord, IteratorFmt, ReaderExt, SliceWord, TupleFmt, VecWord, Word, WriterExt};

#[test]
fn test_vec_word() {
    let mut input = re(b" \n ab bc \n \n cd ");
    assert_eq!(input.re::<VecWord>(), Word((*b"ab").to_vec()));
    assert_eq!(input.re::<VecWord>(), Word((*b"bc").to_vec()));
    assert_eq!(input.re::<VecWord>(), Word((*b"cd").to_vec()));
    assert_eq!(format!("{}", Word((*b" abc ").to_vec())), " abc ");
    assert_eq!(format!("{:?}", Word((*b" abc ").to_vec())), " abc ");
}

#[test]
fn test_slice_word() {
    let mut input = re(b" \n ab bc \n \n cd ");
    assert_eq!(input.re::<SliceWord<'_>>(), Word(&b"ab"[..]));
    assert_eq!(input.re::<SliceWord<'_>>(), Word(&b"bc"[..]));
    assert_eq!(input.re::<SliceWord<'_>>(), Word(&b"cd"[..]));
    assert_eq!(format!("{}", Word(&b" abc "[..])), " abc ");
    assert_eq!(format!("{:?}", Word(&b" abc "[..])), " abc ");
}

#[test]
fn test_array_word() {
    let mut input = re(b" \n ab bc \n \n cd ");
    assert_eq!(input.re::<ArrayWord<2>>(), Word(*b"ab"));
    assert_eq!(input.re::<ArrayWord<2>>(), Word(*b"bc"));
    assert_eq!(input.re::<ArrayWord<2>>(), Word(*b"cd"));
    assert_eq!(format!("{}", Word(*b" abc ")), " abc ");
    assert_eq!(format!("{:?}", Word(*b" abc ")), " abc ");
}

#[test]
#[should_panic]
fn test_array_word_smaller() {
    let mut input = re(b" \n ab bc \n \n cd ");
    let _ = input.re::<ArrayWord<3>>();
}

#[test]
#[should_panic]
fn test_array_word_bigger() {
    let mut input = re(b" \n ab bc \n \n cd ");
    let _ = input.re::<ArrayWord<1>>();
}

#[test]
fn test_separated_tuple() {
    assert_eq!(wr().wo(().jo()).as_writer(), b"");
    assert_eq!(wr().wo((12,).jo()).as_writer(), b"12");
    assert_eq!(wr().wo((12, "ab").jo()).as_writer(), b"12ab");
    assert_eq!(wr().wo((12, "ab", 34).jo()).as_writer(), b"12ab34");
    assert_eq!(wr().wo(().wo()).as_writer(), b"");
    assert_eq!(wr().wo((12,).wo()).as_writer(), b"12");
    assert_eq!(wr().wo((12, "ab").wo()).as_writer(), b"12 ab");
    assert_eq!(wr().wo((12, "ab", 34).wo()).as_writer(), b"12 ab 34");
    assert_eq!(wr().wo(().li()).as_writer(), b"");
    assert_eq!(wr().wo((12,).li()).as_writer(), b"12");
    assert_eq!(wr().wo((12, "ab").li()).as_writer(), b"12\nab");
    assert_eq!(wr().wo((12, "ab", 34).li()).as_writer(), b"12\nab\n34");
    assert_eq!(wr().wo(().sep(", ")).as_writer(), b"");
    assert_eq!(wr().wo((12,).sep(", ")).as_writer(), b"12");
    assert_eq!(wr().wo((12, "ab").sep(", ")).as_writer(), b"12, ab");
    assert_eq!(wr().wo((12, "ab", 34).sep(", ")).as_writer(), b"12, ab, 34");
    assert_eq!(wr().wo(().fmt("[", ", ", "]")).as_writer(), b"[]");
    assert_eq!(wr().wo((12,).fmt("[", ", ", "]")).as_writer(), b"[12]");
    assert_eq!(
        wr().wo((12, "ab").fmt("[", ", ", "]")).as_writer(),
        b"[12, ab]"
    );
    assert_eq!(
        wr().wo((12, "ab", 34).fmt("[", ", ", "]")).as_writer(),
        b"[12, ab, 34]"
    );
    assert_eq!(
        wr().wo(((1, 2).wo(), (1, 2).wo()).jo()).as_writer(),
        b"1 21 2"
    );
}

#[test]
fn test_separated_iterator() {
    assert_eq!(wr().wo([0; 0].jo()).as_writer(), b"");
    assert_eq!(wr().wo([12].jo()).as_writer(), b"12");
    assert_eq!(wr().wo([12, 23].jo()).as_writer(), b"1223");
    assert_eq!(wr().wo([12, 23, 34].jo()).as_writer(), b"122334");
    assert_eq!(wr().wo([0; 0].wo()).as_writer(), b"");
    assert_eq!(wr().wo([12].wo()).as_writer(), b"12");
    assert_eq!(wr().wo([12, 23].wo()).as_writer(), b"12 23");
    assert_eq!(wr().wo([12, 23, 34].wo()).as_writer(), b"12 23 34");
    assert_eq!(wr().wo([0; 0].li()).as_writer(), b"");
    assert_eq!(wr().wo([12].li()).as_writer(), b"12");
    assert_eq!(wr().wo([12, 23].li()).as_writer(), b"12\n23");
    assert_eq!(wr().wo([12, 23, 34].li()).as_writer(), b"12\n23\n34");
    assert_eq!(wr().wo([0; 0].sep(", ")).as_writer(), b"");
    assert_eq!(wr().wo([12].sep(", ")).as_writer(), b"12");
    assert_eq!(wr().wo([12, 23].sep(", ")).as_writer(), b"12, 23");
    assert_eq!(wr().wo([12, 23, 34].sep(", ")).as_writer(), b"12, 23, 34");
    assert_eq!(wr().wo([0; 0].fmt("[", ", ", "]")).as_writer(), b"[]");
    assert_eq!(wr().wo([12].fmt("[", ", ", "]")).as_writer(), b"[12]");
    assert_eq!(
        wr().wo([12, 23].fmt("[", ", ", "]")).as_writer(),
        b"[12, 23]"
    );
    assert_eq!(
        wr().wo([12, 23, 34].fmt("[", ", ", "]")).as_writer(),
        b"[12, 23, 34]"
    );
    assert_eq!(
        wr().wo([[1, 2].wo(), [1, 2].wo()].jo()).as_writer(),
        b"1 21 2"
    );
}
