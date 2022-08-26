use crate::extensions::io_chs::Chs;
use crate::tests::re;
use crate::{LineStart, ReaderExt, WordStart};

#[test]
fn test_chs() {
    let mut input = re(b" \n ab cd \n \n ef ");
    assert_eq!(input.re::<Chs<1>>(), Chs(*b" "));
    assert_eq!(input.re::<(WordStart, Chs<3>)>().1, Chs(*b"ab "));
    assert_eq!(input.re::<(LineStart, LineStart, Chs<3>)>().2, Chs(*b" ef"));
    assert_eq!(format!("{}", Chs(*b" abc ")), " abc ");
    assert_eq!(format!("{:?}", Chs(*b" abc ")), " abc ");
}
