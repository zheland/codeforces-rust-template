use crate::{Chs, Dec, LineReader, ReaderExt, Word, WordWriter, WriterExt};

use std::io::BufReader;

fn re(value: &[u8]) -> LineReader<BufReader<&[u8]>> {
    LineReader::new(BufReader::new(value))
}

fn wr() -> WordWriter<Vec<u8>> {
    WordWriter::new(Vec::new())
}

#[test]
fn test_read() {
    assert_eq!(re(b" \n ab bc cd ").re::<&str>(), "ab");
    {
        let mut re = re(b" \n ab bc cd ");
        let _ = re.re::<&str>();
        assert_eq!(re.re::<&str>(), "bc");
    }
    assert_eq!(re(b" \n 123 bc cd ").re::<u32>(), 123);
    assert_eq!(re(b" \n ab bc cd ").re::<Word>(), Word(b"ab".to_vec()));
    assert_eq!(re(b" \n ab bc cd ").re::<Word<[u8; 2]>>(), Word(*b"ab"));
    assert_eq!(re(b" \n ab bc cd ").re::<Word<[u8; 3]>>(), Word(*b"ab "));
    assert_eq!(re(b" \n ab bc cd ").re::<Word<[u8; 4]>>(), Word(*b"ab b"));
    assert_eq!(re(b" \n ab bc cd ").re::<Word<[u8; 2]>>(), Word(*b"ab"));
    assert_eq!(re(b" \n ab bc cd ").re::<Word<[u8; 3]>>(), Word(*b"ab\0"));
    assert_eq!(re(b" \n ab bc cd ").re::<Word<[u8; 4]>>(), Word(*b"ab\0\0"));
    assert_eq!(re(b" \n 135 246 357 ").re::<Dec<[u8; 3]>>(), Dec([1, 3, 5]));
}

#[test]
#[should_panic]
fn test_read_ascii_word_smaller_size() {
    let _ = re(b" \n ab bc cd ").re::<Word<[u8; 1]>>();
}

#[test]
fn test_read_ascii_smaller_size() {
    {
        let mut re = re(b" \n ab bc cd ");
        assert_eq!(re.re::<Chs<[u8; 1]>>(), Chs(*b"a"));
        assert_eq!(re.re::<Chs<[u8; 1]>>(), Chs(*b"b"));
        assert_eq!(re.re::<Chs<[u8; 1]>>(), Chs(*b"b"));
        assert_eq!(re.re::<Chs<[u8; 1]>>(), Chs(*b"c"));
    }
}

#[test]
#[should_panic]
fn test_read_dec_smaller_size() {
    let _ = re(b" \n 135 246 357 ").re::<Dec<[u8; 2]>>();
}

#[test]
#[should_panic]
fn test_read_dec_greater_size() {
    let _ = re(b" \n 135 246 357 ").re::<Dec<[u8; 4]>>();
}

#[test]
#[should_panic]
fn test_read_dec_invalid_chars() {
    let _ = re(b" \n a35 246 357 ").re::<Dec<[u8; 3]>>();
}

#[test]
fn test_write() {
    assert_eq!(wr().wo2("ab").as_writer(), b"ab");
    assert_eq!(
        wr().wo2("ab").wo2("bc").ln2().wo2("cd").as_writer(),
        b"ab bc\ncd"
    );
    assert_eq!(wr().wo2(123).as_writer(), b"123");
    assert_eq!(wr().wo2(Chs(&b"abc"[..])).as_writer(), b"abc");
    assert_eq!(wr().wo2(Chs(*b"abc")).as_writer(), b"abc");
    assert_eq!(wr().wo2(Chs(b"abc".to_vec())).as_writer(), b"abc");
    assert_eq!(wr().wo2(Dec(&[1, 2, 3][..])).as_writer(), b"123");
    assert_eq!(wr().wo2(Dec([1, 2, 3])).as_writer(), b"123");
    assert_eq!(wr().wo2(Dec(vec![1, 2, 3])).as_writer(), b"123");
}

#[test]
#[should_panic]
fn test_write_dec_invalid_chars() {
    let _ = wr().wr(Dec([1, 2, 10]));
}
