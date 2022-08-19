use crate::{
    ArrayDecBe, ArrayDecLe, ArrayWord, Ch, Chs, DecBe, DecLe, IteratorFmt, LineReader, PartialWord,
    ReaderExt, SliceWord, TupleFmt, VecDecBe, VecDecLe, VecWord, Word, WordWriter, WriterExt,
};

use std::io::BufReader;

fn re(value: &[u8]) -> LineReader<BufReader<&[u8]>> {
    LineReader::new(BufReader::new(value))
}

fn wr() -> WordWriter<Vec<u8>> {
    WordWriter::new(Vec::new())
}

#[test]
fn test_ch() {
    let mut input = re(b" \n ab bc \n \n cd ");
    assert_eq!(input.re::<Ch>(), Ch(b'a'));
    assert_eq!(input.re::<Ch>(), Ch(b'b'));
    assert_eq!(input.re::<Ch>(), Ch(b'b'));
    assert_eq!(input.re::<Ch>(), Ch(b'c'));
    assert_eq!(format!("{}", Ch(b'a')), "a");
    assert_eq!(format!("{:?}", Ch(b'a')), "a");
}

#[test]
fn test_chs() {
    let mut input = re(b" \n ab bc \n \n cd ");
    assert_eq!(input.re::<Chs<1>>(), Chs(*b"a"));
    assert_eq!(input.re::<Chs<3>>(), Chs(*b"b b"));
    assert_eq!(input.re::<Chs<1>>(), Chs(*b"c"));
    assert_eq!(format!("{}", Chs(*b" abc ")), " abc ");
    assert_eq!(format!("{:?}", Chs(*b" abc ")), " abc ");
}

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
fn test_partial_array_word() {
    let mut input = re(b" \n ab bc \n \n cd ");
    assert_eq!(input.re::<PartialWord<2>>(), PartialWord(*b"ab"));
    assert_eq!(input.re::<PartialWord<3>>(), PartialWord(*b"bc\0"));
    assert_eq!(input.re::<PartialWord<4>>(), PartialWord(*b"cd\0\0"));
    assert_eq!(format!("{}", PartialWord(*b" abc \0\0")), " abc ");
    assert_eq!(format!("{:?}", PartialWord(*b" abc \0\0")), " abc ");
}

#[test]
#[should_panic]
fn test_partial_array_word_bigger() {
    let mut input = re(b" \n ab bc cd ");
    let _ = input.re::<PartialWord<1>>();
}

#[test]
fn test_vec_dec_be() {
    let mut input = re(b" \n 0123 1234 \n \n 2345 ");
    assert_eq!(
        input.re::<VecDecBe>(),
        DecBe((*b"\x00\x01\x02\x03").to_vec())
    );
    assert_eq!(
        input.re::<VecDecBe>(),
        DecBe((*b"\x01\x02\x03\x04").to_vec())
    );
    assert_eq!(
        input.re::<VecDecBe>(),
        DecBe((*b"\x02\x03\x04\x05").to_vec())
    );
    assert_eq!(format!("{}", DecBe((*b"\x00\x01\x02\x03").to_vec())), "123");
    assert_eq!(
        format!("{:?}", DecBe((*b"\x01\x02\x03\x04").to_vec())),
        "1234"
    );
}

#[test]
fn test_array_dec_be() {
    let mut input = re(b" \n 0123 1234 \n \n 2345 ");
    assert_eq!(input.re::<ArrayDecBe<4>>(), DecBe(*b"\x00\x01\x02\x03"));
    assert_eq!(input.re::<ArrayDecBe<5>>(), DecBe(*b"\x00\x01\x02\x03\x04"));
    assert_eq!(
        input.re::<ArrayDecBe<6>>(),
        DecBe(*b"\x00\x00\x02\x03\x04\x05")
    );
    assert_eq!(format!("{}", DecBe(*b"\x00\x01\x02\x03")), "123");
    assert_eq!(format!("{:?}", DecBe(*b"\x01\x02\x03\x04")), "1234");
}

#[test]
#[should_panic]
fn test_array_dec_be_smaller() {
    let mut input = re(b" \n 0123 1234 \n \n 2345 ");
    let _ = input.re::<ArrayDecBe<3>>();
}

#[test]
#[should_panic]
fn test_array_dec_be_invalid_chars() {
    let mut input = re(b" \n ab ");
    let _ = input.re::<ArrayDecBe<4>>();
}

#[test]
fn test_vec_dec_le() {
    let mut input = re(b" \n 0123 1234 \n \n 2345 ");
    assert_eq!(
        input.re::<VecDecLe>(),
        DecLe((*b"\x03\x02\x01\x00").to_vec())
    );
    assert_eq!(
        input.re::<VecDecLe>(),
        DecLe((*b"\x04\x03\x02\x01").to_vec())
    );
    assert_eq!(
        input.re::<VecDecLe>(),
        DecLe((*b"\x05\x04\x03\x02").to_vec())
    );
    assert_eq!(
        format!("{}", DecLe((*b"\x00\x01\x02\x03\x00").to_vec())),
        "3210"
    );
    assert_eq!(
        format!("{:?}", DecLe((*b"\x01\x02\x03\x04").to_vec())),
        "4321"
    );
}

#[test]
fn test_array_dec_le() {
    let mut input = re(b" \n 0123 1234 \n \n 2345 ");
    assert_eq!(input.re::<ArrayDecLe<4>>(), DecLe(*b"\x03\x02\x01\x00"));
    assert_eq!(input.re::<ArrayDecLe<5>>(), DecLe(*b"\x04\x03\x02\x01\x00"));
    assert_eq!(
        input.re::<ArrayDecLe<6>>(),
        DecLe(*b"\x05\x04\x03\x02\x00\x00")
    );
    assert_eq!(format!("{}", DecLe(*b"\x00\x01\x02\x03\x00")), "3210");
    assert_eq!(format!("{:?}", DecLe(*b"\x01\x02\x03\x04")), "4321");
}

#[test]
#[should_panic]
fn test_array_dec_le_smaller() {
    let mut input = re(b" \n 0123 1234 \n \n 2345 ");
    let _ = input.re::<ArrayDecLe<3>>();
}

#[test]
#[should_panic]
fn test_array_dec_le_invalid_chars() {
    let mut input = re(b" \n ab ");
    let _ = input.re::<ArrayDecLe<4>>();
}

#[test]
#[should_panic]
fn test_separated_tuple() {
    assert_eq!(wr().wo2(().jo()).as_writer(), b"");
    assert_eq!(wr().wo2((12,).jo()).as_writer(), b"12");
    assert_eq!(wr().wo2((12, "ab").jo()).as_writer(), b"12ab");
    assert_eq!(wr().wo2((12, "ab", 34).jo()).as_writer(), b"12ab34");
    assert_eq!(wr().wo2(().wo()).as_writer(), b"");
    assert_eq!(wr().wo2((12,).wo()).as_writer(), b"12");
    assert_eq!(wr().wo2((12, "ab").wo()).as_writer(), b"12 ab");
    assert_eq!(wr().wo2((12, "ab", 34).wo()).as_writer(), b"12 ab 34");
    assert_eq!(wr().wo2(().li()).as_writer(), b"");
    assert_eq!(wr().wo2((12,).li()).as_writer(), b"12");
    assert_eq!(wr().wo2((12, "ab").li()).as_writer(), b"12\nab");
    assert_eq!(wr().wo2((12, "ab", 34).li()).as_writer(), b"12\nab\n34");
    assert_eq!(wr().wo2(().sep(", ")).as_writer(), b"");
    assert_eq!(wr().wo2((12,).sep(", ")).as_writer(), b"12");
    assert_eq!(wr().wo2((12, "ab").sep(", ")).as_writer(), b"12, ab");
    assert_eq!(
        wr().wo2((12, "ab", 34).sep(", ")).as_writer(),
        b"12, ab, 34"
    );
    assert_eq!(wr().wo2(().fmt("[", ", ", "]")).as_writer(), b"");
    assert_eq!(wr().wo2((12,).fmt("[", ", ", "]")).as_writer(), b"12");
    assert_eq!(
        wr().wo2((12, "ab").fmt("[", ", ", "]")).as_writer(),
        b"12, ab"
    );
    assert_eq!(
        wr().wo2((12, "ab", 34).fmt("[", ", ", "]")).as_writer(),
        b"12, ab, 34"
    );
}

#[test]
#[should_panic]
fn test_separated_iterator() {
    assert_eq!(wr().wo2([0; 0].jo()).as_writer(), b"");
    assert_eq!(wr().wo2([12].jo()).as_writer(), b"12");
    assert_eq!(wr().wo2([12, 23].jo()).as_writer(), b"1223");
    assert_eq!(wr().wo2([12, 23, 34].jo()).as_writer(), b"122334");
    assert_eq!(wr().wo2([0; 0].wo()).as_writer(), b"");
    assert_eq!(wr().wo2([12].wo()).as_writer(), b"12");
    assert_eq!(wr().wo2([12, 23].wo()).as_writer(), b"12 23");
    assert_eq!(wr().wo2([12, 23, 34].wo()).as_writer(), b"12 23 34");
    assert_eq!(wr().wo2([0; 0].li()).as_writer(), b"");
    assert_eq!(wr().wo2([12].li()).as_writer(), b"12");
    assert_eq!(wr().wo2([12, 23].li()).as_writer(), b"12\n23");
    assert_eq!(wr().wo2([12, 23, 34].li()).as_writer(), b"12\n23\n34");
    assert_eq!(wr().wo2([0; 0].sep(", ")).as_writer(), b"");
    assert_eq!(wr().wo2([12].sep(", ")).as_writer(), b"12");
    assert_eq!(wr().wo2([12, 23].sep(", ")).as_writer(), b"12, 23");
    assert_eq!(wr().wo2([12, 23, 34].sep(", ")).as_writer(), b"12, 23, 34");
    assert_eq!(wr().wo2([0; 0].fmt("[", ", ", "]")).as_writer(), b"");
    assert_eq!(wr().wo2([12].fmt("[", ", ", "]")).as_writer(), b"12");
    assert_eq!(
        wr().wo2([12, 23].fmt("[", ", ", "]")).as_writer(),
        b"12, 23"
    );
    assert_eq!(
        wr().wo2([12, 23, 34].fmt("[", ", ", "]")).as_writer(),
        b"12, 23, 34"
    );
}
