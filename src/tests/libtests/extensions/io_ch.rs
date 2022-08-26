use crate::extensions::io_ch::Ch;
use crate::tests::re;
use crate::{LineStart, ReaderExt, WordStart};

#[test]
fn test_ch() {
    let mut input = re(b" \n ab cd \n \n ef ");

    assert_eq!(input.re::<Ch>(), Ch(b' '));
    assert_eq!(input.re::<Ch>(), Ch(b'\n'));
    assert_eq!(input.re::<Ch>(), Ch(b' '));
    assert_eq!(input.re::<Ch>(), Ch(b'a'));
    assert_eq!(input.re::<Ch>(), Ch(b'b'));
    assert_eq!(input.re::<(WordStart, Ch)>().1, Ch(b'c'));
    assert_eq!(input.re::<(LineStart, LineStart, Ch)>().2, Ch(b' '));
    assert_eq!(input.re::<Ch>(), Ch(b'e'));
    assert_eq!(format!("{}", Ch(b'a')), "a");
    assert_eq!(format!("{:?}", Ch(b'a')), "a");
}
