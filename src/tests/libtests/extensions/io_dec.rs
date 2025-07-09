use crate::extensions::io_dec::{ArrayDecBe, ArrayDecLe, DecBe, DecLe, VecDecBe, VecDecLe};
use crate::tests::re;
use crate::ReaderExt;

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
#[should_panic(expected = "source word length exceeds target word length")]
fn test_array_dec_be_smaller() {
    let mut input = re(b" \n 0123 1234 \n \n 2345 ");
    let _ = input.re::<ArrayDecBe<3>>();
}

#[test]
#[should_panic(expected = "assertion failed: *value >= b'0' && *value <= b'9'")]
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
#[should_panic(expected = "source word length exceeds target word length")]
fn test_array_dec_le_smaller() {
    let mut input = re(b" \n 0123 1234 \n \n 2345 ");
    let _ = input.re::<ArrayDecLe<3>>();
}

#[test]
#[should_panic(expected = "assertion failed: *value >= b'0' && *value <= b'9'")]
fn test_array_dec_le_invalid_chars() {
    let mut input = re(b" \n ab ");
    let _ = input.re::<ArrayDecLe<4>>();
}
