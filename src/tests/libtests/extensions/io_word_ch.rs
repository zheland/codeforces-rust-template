use crate::extensions::io_word_ch::WordCh;
use crate::tests::re;
use crate::ReaderExt;

#[test]
fn test_word_ch() {
    let mut input = re(b" \n ab bc \n \n cd ");
    assert_eq!(input.re::<WordCh>(), WordCh(b'a'));
    assert_eq!(input.re::<WordCh>(), WordCh(b'b'));
    assert_eq!(input.re::<WordCh>(), WordCh(b'b'));
    assert_eq!(input.re::<WordCh>(), WordCh(b'c'));
    assert_eq!(format!("{}", WordCh(b'a')), "a");
    assert_eq!(format!("{:?}", WordCh(b'a')), "a");
}
