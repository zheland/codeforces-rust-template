use crate::extensions::modular::{
    ConstValue, Modular, PrimeValue, ValueWithEulersPhi, M1_000_000_007,
};
use crate::primes::Primes;

#[test]
fn test_mod_ops() {
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    struct N103;

    impl ConstValue for N103 {
        type Output = u32;
        fn get() -> Self::Output {
            103
        }
    }

    impl PrimeValue for N103 {}

    fn vmod(value: u32) -> Modular<u32, N103> {
        Modular::from(value)
    }

    assert_eq!(vmod(10), vmod(10));
    assert_eq!(-vmod(10), vmod(93));

    assert_eq!(vmod(50) + vmod(10), vmod(60));
    assert_eq!(vmod(50) + vmod(40), vmod(90));
    assert_eq!(vmod(50) + vmod(80), vmod(27));
    assert_eq!(vmod(50) + vmod(102), vmod(49));
    assert_eq!(vmod(102) + vmod(102), vmod(101));

    assert_eq!(vmod(50) - vmod(10), vmod(40));
    assert_eq!(vmod(50) - vmod(40), vmod(10));
    assert_eq!(vmod(50) - vmod(80), vmod(73));
    assert_eq!(vmod(50) - vmod(102), vmod(51));
    assert_eq!(vmod(102) - vmod(102), vmod(0));

    assert_eq!(vmod(11) * vmod(22), vmod(36));
}

#[test]
fn test_rem_example() {
    let mut v = M1_000_000_007::from(0);
    v += 1_000_000_006;
    assert_eq!(v.value(), 1_000_000_006);
    v += 1;
    assert_eq!(v.value(), 0);
}

#[test]
fn test_mod_pow() {
    let v = M1_000_000_007::from(123);
    assert_eq!(v.pow(3).value(), 1_860_867);
    assert_eq!(v.pow(10).value(), 26_898_250);
}

#[test]
#[allow(clippy::similar_names)]
fn test_mod_inv() {
    let v = M1_000_000_007::from(123);

    let a1 = v.pow(1);
    let b1 = a1.inv().unwrap();
    assert_eq!(a1.value(), 123);
    assert_eq!(b1.value(), 886_178_868);
    assert_eq!((a1 * b1).value(), 1);

    let a3 = v.pow(3);
    let b3 = a3.inv().unwrap();
    assert_eq!(a3.value(), 1_860_867);
    assert_eq!(b3.value(), 939_777_003);
    assert_eq!((a3 * b3).value(), 1);

    let a10 = v.pow(10);
    let b10 = a10.inv().unwrap();
    assert_eq!(a10.value(), 26_898_250);
    assert_eq!(b10.value(), 408_060_267);
    assert_eq!((a10 * b10).value(), 1);
}

#[test]
fn test_modular_inv_with_eulers_phi() {
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    struct N103;

    impl ConstValue for N103 {
        type Output = u32;
        fn get() -> Self::Output {
            103
        }
    }

    impl PrimeValue for N103 {}

    let primes = Primes::new(1000);

    for n in 0..N103::get() {
        let v = Modular::from_raw(n, N103);
        assert_eq!(v.inv(), v.inv_with_eulers_phi());
    }

    for m in 2_u32..100 {
        for n in 0..m {
            #[allow(clippy::cast_possible_truncation)]
            let m = ValueWithEulersPhi::new(m, primes.eulers_phi(m as usize) as u32);
            let v1 = Modular::from_raw(n, m);
            let v2 = v1.inv_with_eulers_phi();
            if let Some(v2) = v2 {
                if n == 0 {
                    assert_eq!(v2, Modular::from_raw(0, m));
                } else {
                    assert_eq!(v1 * v2, Modular::from_raw(1, m));
                }
            }
        }
    }
}

#[test]
fn test_mod_div() {
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    struct N103;

    impl ConstValue for N103 {
        type Output = u32;
        fn get() -> Self::Output {
            103
        }
    }

    impl PrimeValue for N103 {}

    for a in 0..103 {
        for b in 1..103 {
            let a: Modular<_, N103> = Modular::from(a);
            let b = Modular::from(b);
            let c = a * b;
            let d = c / b;
            assert_eq!(a, d);
        }
    }
}
