use crate::extensions::math_binomial::{binomial, Binomial};

#[test]
fn test_binomial() {
    assert_eq!(binomial(0_u32, 0_u32), 1_u32);
    assert_eq!(binomial(1_u32, 0_u32), 1_u32);
    assert_eq!(binomial(1_u32, 1_u32), 1_u32);
    assert_eq!(binomial(2_u32, 0_u32), 1_u32);
    assert_eq!(binomial(2_u32, 1_u32), 2_u32);
    assert_eq!(binomial(2_u32, 2_u32), 1_u32);
    assert_eq!(binomial(3_u32, 0_u32), 1_u32);
    assert_eq!(binomial(3_u32, 1_u32), 3_u32);
    assert_eq!(binomial(3_u32, 2_u32), 3_u32);
    assert_eq!(binomial(3_u32, 3_u32), 1_u32);
    assert_eq!(binomial(5_u32, 0_u32), 1_u32);
    assert_eq!(binomial(5_u32, 1_u32), 5_u32);
    assert_eq!(binomial(5_u32, 2_u32), 10_u32);
    assert_eq!(binomial(5_u32, 3_u32), 10_u32);
    assert_eq!(binomial(5_u32, 4_u32), 5_u32);
    assert_eq!(binomial(5_u32, 5_u32), 1_u32);

    let coeff = Binomial::with(8, 3);
    assert_eq!(coeff.get(), &56);
    assert_eq!(coeff.dec_n().get(), &35);
    assert_eq!(coeff.inc_n().get(), &84);
    assert_eq!(coeff.dec_k().get(), &28);
    assert_eq!(coeff.inc_k().get(), &70);
    assert_eq!(coeff.dec_nk().get(), &21);
    assert_eq!(coeff.inc_nk().get(), &126);
}
