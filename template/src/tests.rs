use core::fmt::Debug;
use std::io::BufReader;

use crate::problem::*;

fn re(value: &[u8]) -> Reader<BufReader<&[u8]>> {
    Reader::new(BufReader::new(value))
}

fn wr() -> Writer<Vec<u8>> {
    Writer::new(Vec::new())
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
    assert_eq!(re(b" \n ab bc cd ").re::<Ascii>(), Ascii(b"ab".to_vec()));
    assert_eq!(
        re(b" \n ab bc cd ").re::<Ascii<[u8; 2]>>(),
        Ascii([b'a', b'b'])
    );
    assert_eq!(
        re(b" \n ab bc cd ").re::<Ascii<[u8; 3]>>(),
        Ascii([b'a', b'b', b'\0'])
    );
    assert_eq!(re(b" \n 135 246 357 ").re::<Dec<[u8; 3]>>(), Dec([1, 3, 5]));
}

#[test]
#[should_panic]
fn test_read_ascii_smaller_size() {
    let _ = re(b" \n ab bc cd ").re::<Ascii<[u8; 1]>>();
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
    assert_eq!(wr().wr2("ab").to_write(), b"ab");
    assert_eq!(
        wr().wr2("ab").wr2("bc").ln2().wr2("cd").to_write(),
        b"ab bc\ncd"
    );
    assert_eq!(wr().wr2(123).to_write(), b"123");
    assert_eq!(wr().wr2(Ascii(&b"abc"[..])).to_write(), b"abc");
    assert_eq!(wr().wr2(Ascii(*b"abc")).to_write(), b"abc");
    assert_eq!(wr().wr2(Ascii(b"abc".to_vec())).to_write(), b"abc");
    assert_eq!(wr().wr2(Dec(&[1, 2, 3][..])).to_write(), b"123");
    assert_eq!(wr().wr2(Dec([1, 2, 3])).to_write(), b"123");
    assert_eq!(wr().wr2(Dec(vec![1, 2, 3])).to_write(), b"123");
}

#[test]
#[should_panic]
fn test_write_dec_invalid_chars() {
    let _ = wr().wr(Dec([1, 2, 10]));
}
