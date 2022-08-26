use crate::extensions::io_word_chs::WordChs;
use crate::tests::re;
use crate::ReaderExt;

#[test]
fn test_word_chs() {
    let mut input = re(b" \n ab cd \n \n ef ");
    assert_eq!(input.re::<WordChs<1>>(), WordChs(*b"a"));
    assert_eq!(input.re::<WordChs<3>>(), WordChs(*b"b c"));
    assert_eq!(input.re::<WordChs<1>>(), WordChs(*b"d"));
    assert_eq!(format!("{}", WordChs(*b" abc ")), " abc ");
    assert_eq!(format!("{:?}", WordChs(*b" abc ")), " abc ");
}
