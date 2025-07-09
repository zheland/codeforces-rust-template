use crate::extensions::io_partial_word::PartialWord;
use crate::tests::re;
use crate::ReaderExt;

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
#[should_panic(expected = "source word length exceeds target word length")]
fn test_partial_array_word_bigger() {
    let mut input = re(b" \n ab bc cd ");
    let _ = input.re::<PartialWord<1>>();
}
