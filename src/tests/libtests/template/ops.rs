use core::iter::FusedIterator;
use std::hint::black_box;

use crate::tests::rdtsc_perf;
use crate::{
    btm, bts, div_rem_u128, gcd, hm, hs, lcm, wrap, DedupCount, IntoVec, Primes, Sortable,
};

#[test]
fn test_gcd() {
    assert_eq!(gcd(1, 1), 1);
    assert_eq!(gcd(2, 3), 1);
    assert_eq!(gcd(6, 15), 3);
    assert_eq!(gcd(30, 105), 15);
    assert_eq!(gcd(321, 321), 321);
}

#[test]
fn test_lcm() {
    assert_eq!(lcm(1, 1), 1);
    assert_eq!(lcm(2, 3), 6);
    assert_eq!(lcm(6, 15), 30);
    assert_eq!(lcm(30, 105), 210);
    assert_eq!(gcd(321, 321), 321);
}

#[test]
#[allow(clippy::bool_assert_comparison)]
fn test_is_prime() {
    let primes = Primes::new(1_000_000);
    assert_eq!(primes.is_prime(2), true);
    assert_eq!(primes.is_prime(3), true);
    assert_eq!(primes.is_prime(4), false);
    assert_eq!(primes.is_prime(5), true);
    assert_eq!(primes.is_prime(6), false);
    assert_eq!(primes.is_prime(7), true);
    assert_eq!(primes.is_prime(8), false);
    assert_eq!(primes.is_prime(9), false);
    assert_eq!(primes.is_prime(10), false);
    assert_eq!(primes.is_prime(11), true);
    assert_eq!(primes.is_prime(12), false);
    assert_eq!(primes.is_prime(13), true);
    assert_eq!(primes.is_prime(14), false);
    assert_eq!(primes.is_prime(15), false);
    assert_eq!(primes.is_prime(16), false);
    assert_eq!(primes.is_prime(17), true);
    assert_eq!(primes.is_prime(18), false);
    assert_eq!(primes.is_prime(19), true);
    assert_eq!(primes.is_prime(3567), false);
    assert_eq!(primes.is_prime(3569), false);
    assert_eq!(primes.is_prime(3571), true);
    assert_eq!(primes.is_prime(999_979), true);
    assert_eq!(primes.is_prime(999_981), false);
    assert_eq!(primes.is_prime(999_983), true);
}

#[test]
fn test_iter_primes() {
    let sieve = Primes::new(1_000_000);
    let primes = sieve.iter().take(25).into_vec();
    assert_eq!(
        primes,
        [
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97,
        ]
    );
}

#[test]
fn test_prime_sieve_iter_primes() {
    let sieve = Primes::new(1_000_000);
    let mut primes = sieve.iter();
    assert_eq!(primes.next(), Some(2));
    assert_eq!(primes.next(), Some(3));
    assert_eq!(primes.next(), Some(5));
    assert_eq!(primes.next(), Some(7));
    assert_eq!(primes.next(), Some(11));
    assert_eq!(primes.next(), Some(13));
    assert_eq!(primes.next(), Some(17));
    assert_eq!(primes.next(), Some(19));
    assert_eq!(primes.next(), Some(23));
    assert_eq!(primes.next(), Some(29));
    assert_eq!(primes.next(), Some(31));
    assert_eq!(primes.next(), Some(37));
    assert_eq!(primes.next(), Some(41));
    assert_eq!(primes.next(), Some(43));
    assert_eq!(primes.next(), Some(47));
    assert_eq!(primes.next(), Some(53));
    assert_eq!(primes.next(), Some(59));
    assert_eq!(primes.next(), Some(61));
    assert_eq!(primes.next(), Some(67));
    assert_eq!(primes.next(), Some(71));
    assert_eq!(primes.next(), Some(73));
    assert_eq!(primes.next(), Some(79));
    assert_eq!(primes.next(), Some(83));
    assert_eq!(primes.next(), Some(89));
    assert_eq!(primes.next(), Some(97));
}

#[test]
fn test_prime_sieve_factorize() {
    let primes = Primes::new(1_000_000);
    assert_eq!(primes.factorize(1).into_vec(), []);
    assert_eq!(primes.factorize(2).into_vec(), [2]);
    assert_eq!(primes.factorize(3).into_vec(), [3]);
    assert_eq!(primes.factorize(4).into_vec(), [2, 2]);
    assert_eq!(primes.factorize(5).into_vec(), [5]);
    assert_eq!(primes.factorize(6).into_vec(), [2, 3]);
    assert_eq!(primes.factorize(7).into_vec(), [7]);
    assert_eq!(primes.factorize(8).into_vec(), [2, 2, 2]);
    assert_eq!(primes.factorize(9).into_vec(), [3, 3]);
    assert_eq!(primes.factorize(10).into_vec(), [2, 5]);
    assert_eq!(primes.factorize(11).into_vec(), [11]);
    assert_eq!(primes.factorize(12).into_vec(), [2, 2, 3]);
    assert_eq!(primes.factorize(13).into_vec(), [13]);
    assert_eq!(primes.factorize(14).into_vec(), [2, 7]);
    assert_eq!(primes.factorize(15).into_vec(), [3, 5]);
    assert_eq!(primes.factorize(16).into_vec(), [2, 2, 2, 2]);
    assert_eq!(primes.factorize(17).into_vec(), [17]);
    assert_eq!(primes.factorize(18).into_vec(), [2, 3, 3]);
    assert_eq!(primes.factorize(19).into_vec(), [19]);
    assert_eq!(primes.factorize(3567).into_vec(), [3, 29, 41]);
    assert_eq!(primes.factorize(3569).into_vec(), [43, 83]);
    assert_eq!(primes.factorize(3571).into_vec(), [3571]);
    assert_eq!(primes.factorize(999_979).into_vec(), [999_979]);
    assert_eq!(primes.factorize(999_981).into_vec(), [3, 3, 111_109]);
    assert_eq!(primes.factorize(999_983).into_vec(), [999_983]);
}

#[test]
fn test_prime_sieve_num_divisors() {
    let primes = Primes::new(1_000_000);
    assert_eq!(primes.num_divisors(1), 1);
    assert_eq!(primes.num_divisors(2), 2);
    assert_eq!(primes.num_divisors(3), 2);
    assert_eq!(primes.num_divisors(4), 3);
    assert_eq!(primes.num_divisors(5), 2);
    assert_eq!(primes.num_divisors(6), 4);
    assert_eq!(primes.num_divisors(7), 2);
    assert_eq!(primes.num_divisors(8), 4);
    assert_eq!(primes.num_divisors(9), 3);
    assert_eq!(primes.num_divisors(10), 4);
    assert_eq!(primes.num_divisors(11), 2);
    assert_eq!(primes.num_divisors(12), 6);
    assert_eq!(primes.num_divisors(13), 2);
    assert_eq!(primes.num_divisors(14), 4);
    assert_eq!(primes.num_divisors(15), 4);
    assert_eq!(primes.num_divisors(16), 5);
    assert_eq!(primes.num_divisors(17), 2);
    assert_eq!(primes.num_divisors(18), 6);
    assert_eq!(primes.num_divisors(19), 2);
    assert_eq!(primes.num_divisors(3567), 8);
    assert_eq!(primes.num_divisors(3569), 4);
    assert_eq!(primes.num_divisors(3571), 2);
    assert_eq!(primes.num_divisors(999_979), 2);
    assert_eq!(primes.num_divisors(999_981), 6);
    assert_eq!(primes.num_divisors(999_983), 2);
}

#[test]
fn test_prime_sieve_eulers_phi() {
    let primes = Primes::new(1_000_000);
    assert_eq!(primes.eulers_phi(1), 0);
    assert_eq!(primes.eulers_phi(2), 1);
    assert_eq!(primes.eulers_phi(3), 2);
    assert_eq!(primes.eulers_phi(4), 2);
    assert_eq!(primes.eulers_phi(5), 4);
    assert_eq!(primes.eulers_phi(6), 2);
    assert_eq!(primes.eulers_phi(7), 6);
    assert_eq!(primes.eulers_phi(8), 4);
    assert_eq!(primes.eulers_phi(9), 6);
    assert_eq!(primes.eulers_phi(10), 4);
    assert_eq!(primes.eulers_phi(11), 10);
    assert_eq!(primes.eulers_phi(12), 4);
    assert_eq!(primes.eulers_phi(13), 12);
    assert_eq!(primes.eulers_phi(14), 6);
    assert_eq!(primes.eulers_phi(15), 8);
    assert_eq!(primes.eulers_phi(16), 8);
    assert_eq!(primes.eulers_phi(17), 16);
    assert_eq!(primes.eulers_phi(18), 6);
    assert_eq!(primes.eulers_phi(19), 18);
    assert_eq!(primes.eulers_phi(3567), 2240);
    assert_eq!(primes.eulers_phi(3569), 3444);
    assert_eq!(primes.eulers_phi(3571), 3570);
    assert_eq!(primes.eulers_phi(999_979), 999_978);
    assert_eq!(primes.eulers_phi(999_981), 666_648);
    assert_eq!(primes.eulers_phi(999_983), 999_982);
}

#[test]
fn test_primes_perf() {
    for j in 0..1_000_000 {
        let _ = black_box(j);
    }
    let p1 = rdtsc_perf(|| (), |_| Primes::new(1000), 256);
    let p2 = rdtsc_perf(|| (), |_| FullPrimesSieve::new(1000), 256);
    assert!(p1 < p2 * 2);

    let p1 = rdtsc_perf(
        || Primes::new(100_000),
        |p| {
            for v in 100..200 {
                for f in p.factorize(v) {
                    let _f = black_box(f);
                }
            }
        },
        256,
    );
    let p2 = rdtsc_perf(
        || FullPrimesSieve::new(100_000),
        |p| {
            for v in 100..200 {
                for f in p.factorize(v) {
                    let _f = black_box(f);
                }
            }
        },
        256,
    );
    assert!(p1 < p2 * 2);
}

#[test]
fn test_primes_eq() {
    for len in vec![0..16, 256..292, 1000..1001].into_iter().flatten() {
        let primes = Primes::new(len);
        let primes_full = FullPrimesSieve::new(len);
        println!("{len}");
        assert_eq!(primes.iter().into_vec(), primes_full.iter().into_vec());
        for j in 0..len.min(8) {
            assert_eq!(
                primes.iter_from(j).into_vec(),
                primes_full.iter_from(j).into_vec()
            );
        }
        for j in 0..len {
            println!("{len} {j}");
            assert_eq!(primes.get_sieve_value(j), primes_full.sieve()[j]);
            assert_eq!(primes.is_prime(j), primes_full.is_prime(j));
            assert_eq!(
                primes.factorize(j).into_vec(),
                primes_full.factorize(j).into_vec()
            );
        }
    }
}

#[test]
fn test_count_dups() {
    type Lhs = [i32];
    type Rhs = [(&'static i32, usize)];
    let data: Vec<(&Lhs, &Rhs)> = vec![
        (&[], &[]),
        (&[1], &[(&1, 1)]),
        (&[1, 1], &[(&1, 2)]),
        (&[1, 1, 1], &[(&1, 3)]),
        (&[1, 1, 1, 1], &[(&1, 4)]),
        (&[1, 2], &[(&1, 1), (&2, 1)]),
        (&[1, 2, 2], &[(&1, 1), (&2, 2)]),
        (&[1, 1, 2, 2], &[(&1, 2), (&2, 2)]),
        (&[1, 1, 2, 3, 3], &[(&1, 2), (&2, 1), (&3, 2)]),
        (
            &[1, 1, 2, 3, 3, 2, 1, 1],
            &[(&1, 2), (&2, 1), (&3, 2), (&2, 1), (&1, 2)],
        ),
    ];
    for (lhs, rhs) in data {
        assert_eq!(lhs.dedup_count().into_vec(), rhs);
    }
}

#[test]
fn test_array_into_iter() {
    let a = [1, 2, 3];
    assert_eq!(a.into_iter().into_vec(), [1, 2, 3]);
}

#[test]
fn test_into_wrapped() {
    let mut a = wrap(1u32);
    assert_eq!(a.0, 1);
    a -= wrap(2);
    assert_eq!(a.0, 4_294_967_295);
    a += wrap(2);
    assert_eq!(a.0, 1);

    let mut a = wrap(127i8);
    assert_eq!(a.0, 127);
    a += wrap(2);
    assert_eq!(a.0, -127);
    a -= wrap(2);
    assert_eq!(a.0, 127);
}

#[test]
fn test_bts_macro() {
    let mut collection = bts![];
    assert_eq!(collection.iter().collect::<Vec<_>>(), [&0; 0]);
    let _ = collection.insert(1);
    assert_eq!(collection.iter().collect::<Vec<_>>(), [&1]);
    let collection = bts![2];
    assert_eq!(collection.iter().collect::<Vec<_>>(), [&2]);
    let collection = bts![3, 4];
    assert_eq!(collection.iter().collect::<Vec<_>>(), [&3, &4]);
}

#[test]
fn test_btm_macro() {
    let mut collection = btm![];
    assert_eq!(collection.iter().collect::<Vec<_>>(), [(&0, &0); 0]);
    let _ = collection.insert(1, 11);
    assert_eq!(collection.iter().collect::<Vec<_>>(), [(&1, &11)]);
    let collection = btm![(2, 22)];
    assert_eq!(collection.iter().collect::<Vec<_>>(), [(&2, &22)]);
    let collection = btm![(3, 33), (4, 44)];
    assert_eq!(
        collection.iter().collect::<Vec<_>>(),
        [(&3, &33), (&4, &44)]
    );
}

#[test]
fn test_hs_macro() {
    let mut collection = hs![];
    assert_eq!(collection.iter().collect::<Vec<_>>(), [&0; 0]);
    let _ = collection.insert(1);
    assert_eq!(collection.iter().collect::<Vec<_>>(), [&1]);
    let collection = hs![2];
    assert_eq!(collection.iter().collect::<Vec<_>>(), [&2]);
    let collection = hs![3, 4];
    let mut vec = collection.iter().collect::<Vec<_>>();
    vec.sort_unstable();
    assert_eq!(vec, [&3, &4]);
}

#[test]
fn test_hm_macro() {
    let mut collection = hm![];
    assert_eq!(collection.iter().collect::<Vec<_>>(), [(&0, &0); 0]);
    let _ = collection.insert(1, 11);
    assert_eq!(collection.iter().collect::<Vec<_>>(), [(&1, &11)]);
    let collection = hm![(2, 22)];
    assert_eq!(collection.iter().collect::<Vec<_>>(), [(&2, &22)]);
    let collection = hm![(3, 33), (4, 44)];
    let mut vec = collection.iter().collect::<Vec<_>>();
    vec.sort_unstable();
    assert_eq!(vec, [(&3, &33), (&4, &44)]);
}

#[test]
fn test_sort_rev() {
    let mut a = [100, 2, 50, 3, 600, 9, 2, 29];
    let result = [600, 100, 50, 29, 9, 3, 2, 2];
    let mut b = a;
    a.sort_rev();
    b.sort_unstable_rev();
    assert_eq!(a, result);
    assert_eq!(b, result);
}

#[test]
fn test_div_rem_u128() {
    let values = [
        0_u128,
        1,
        2,
        3,
        5,
        (1 << 12) + 123,
        (1 << 20) + 12_345,
        (1 << 32) + 1_234_567,
        u128::from(u64::MAX),
        (1 << 64) + 123_456_789,
        u128::MAX,
    ];
    for divident in values {
        for divisor in values {
            if divisor == 0 || divisor > u128::from(u64::MAX) {
                continue;
            }
            let quot = divident / divisor;
            let rem = divident % divisor;
            #[allow(clippy::cast_possible_truncation)]
            if quot < u128::from(u64::MAX) {
                assert_eq!(
                    div_rem_u128(divident, divisor as u64),
                    (quot as u64, rem as u64),
                    "{divident}/{divisor}",
                );
            }
        }
    }
}

// ========

#[derive(Clone, Debug)]
pub struct FullPrimesSieve(Vec<usize>);

impl FullPrimesSieve {
    pub fn new(len: usize) -> Self {
        let mut data = vec![0; len];
        #[allow(
            clippy::cast_precision_loss,
            clippy::cast_possible_truncation,
            clippy::cast_sign_loss
        )]
        let half = (len as f64).sqrt().ceil() as usize;
        for j in 2..half {
            if data[j] == 0 {
                for k in (j * j..len).step_by(j) {
                    if data[k] == 0 {
                        data[k] = j;
                    }
                }
            }
        }
        Self(data)
    }

    pub fn sieve(&self) -> &[usize] {
        &self.0
    }

    pub fn is_prime(&self, value: usize) -> bool {
        value > 1 && self.0[value] == 0
    }

    pub fn iter(&self) -> FullPrimesSieveIter<'_> {
        FullPrimesSieveIter::new(self, 0)
    }

    pub fn iter_from(&self, from: usize) -> FullPrimesSieveIter<'_> {
        FullPrimesSieveIter::new(self, from)
    }

    pub fn factorize(&self, value: usize) -> FullPrimesSieveFactorizeIter<'_> {
        assert!(value < self.0.len());
        FullPrimesSieveFactorizeIter::new(self, value)
    }
}

#[derive(Clone, Debug)]
pub struct FullPrimesSieveIter<'a>(&'a FullPrimesSieve, usize);

impl<'a> FullPrimesSieveIter<'a> {
    pub fn new(sieve: &'a FullPrimesSieve, from: usize) -> Self {
        Self(sieve, from)
    }
}

impl Iterator for FullPrimesSieveIter<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.1 = self.1.max(2);
        while self.1 < self.0.sieve().len() {
            if self.0.sieve()[self.1] == 0 {
                let item = self.1;
                self.1 += 1;
                return Some(item);
            }
            self.1 += 1;
        }
        None
    }
}

impl FusedIterator for FullPrimesSieveIter<'_> {}

#[derive(Clone, Debug)]
pub struct FullPrimesSieveFactorizeIter<'a>(&'a FullPrimesSieve, usize);

impl<'a> FullPrimesSieveFactorizeIter<'a> {
    pub fn new(sieve: &'a FullPrimesSieve, value: usize) -> Self {
        Self(sieve, value)
    }
}

impl Iterator for FullPrimesSieveFactorizeIter<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.1 > 1 {
            let divisor = self.0.sieve()[self.1];
            if divisor == 0 {
                let item = self.1;
                self.1 = 1;
                Some(item)
            } else {
                self.1 /= divisor;
                Some(divisor)
            }
        } else {
            None
        }
    }
}

impl FusedIterator for FullPrimesSieveFactorizeIter<'_> {}
