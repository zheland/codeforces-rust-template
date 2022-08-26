use core::iter::FusedIterator;

use crate::tests::{black_box, rdtsc_perf};
use crate::{btm, bts, gcd, hm, hs, lcm, wrap, DedupCount, IntoVec, Primes, Sortable};

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
fn test_is_prime() {
    let primes = Primes::new(1000000);
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
    assert_eq!(primes.is_prime(999979), true);
    assert_eq!(primes.is_prime(999981), false);
    assert_eq!(primes.is_prime(999983), true);
}

#[test]
fn test_iter_primes() {
    let sieve = Primes::new(1000000);
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
    let sieve = Primes::new(1000000);
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
    let primes = Primes::new(1000000);
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
    assert_eq!(primes.factorize(999979).into_vec(), [999979]);
    assert_eq!(primes.factorize(999981).into_vec(), [3, 3, 111109]);
    assert_eq!(primes.factorize(999983).into_vec(), [999983]);
}

#[test]
fn test_prime_sieve_num_divisors() {
    let primes = Primes::new(1000000);
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
    assert_eq!(primes.num_divisors(999979), 2);
    assert_eq!(primes.num_divisors(999981), 6);
    assert_eq!(primes.num_divisors(999983), 2);
}

#[test]
fn test_prime_sieve_eulers_phi() {
    let primes = Primes::new(1000000);
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
    assert_eq!(primes.eulers_phi(999979), 999978);
    assert_eq!(primes.eulers_phi(999981), 666648);
    assert_eq!(primes.eulers_phi(999983), 999982);
}

#[test]
fn test_primes_perf() {
    for j in 0..1000000 {
        let _ = black_box(j);
    }
    let p1 = rdtsc_perf(|| (), |_| Primes::new(1000), 256);
    let p2 = rdtsc_perf(|| (), |_| FullPrimesSieve::new(1000), 256);
    assert!(p1 * 2 < p2);

    let p1 = rdtsc_perf(
        || Primes::new(100000),
        |p| {
            for v in 100..200 {
                let _ = p.factorize(v).map(|v| black_box(v)).count();
            }
        },
        256,
    );
    let p2 = rdtsc_perf(
        || FullPrimesSieve::new(100000),
        |p| {
            for v in 100..200 {
                let _ = p.factorize(v).map(|v| black_box(v)).count();
            }
        },
        256,
    );
    assert!(p1 / 2 < p2);
}

#[test]
fn test_primes_eq() {
    for len in vec![0..16, 256..292, 1000..1001].into_iter().flatten() {
        let primes = Primes::new(len);
        let primes_full = FullPrimesSieve::new(len);
        println!("{}", len);
        assert_eq!(primes.iter().into_vec(), primes_full.iter().into_vec());
        for j in 0..len.min(8) {
            assert_eq!(
                primes.iter_from(j).into_vec(),
                primes_full.iter_from(j).into_vec()
            );
        }
        for j in 0..len {
            println!("{} {}", len, j);
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
    assert_eq!(a.0, 4294967295);
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
    let mut c = a;
    a.sort_rev();
    b.sort_unstable_rev();
    c.insertion_sort_rev();
    assert_eq!(a, result);
    assert_eq!(b, result);
    assert_eq!(c, result);
}

#[test]
fn test_insertion_sort() {
    use rand::Rng;

    let mut rng = rand::thread_rng();
    for _ in 0..256 {
        let n = rng.gen_range(0..256);
        let mut a: Vec<u64> = (0..n).map(|_| rng.gen_range(0..65_536)).collect();
        let mut c = a.clone();
        a.insertion_sort();
        c.sort_unstable();
        assert_eq!(a, c);
    }
}

#[test]
fn test_insertion_sort_faster_on_small_arrays() {
    use rand::Rng;

    const SAMPLES: usize = 256;
    const MIN_LENGTH: usize = 2;
    const MAX_LENGTH: usize = 8;
    const NUM_ITERATIONS: usize = 64;
    const MIN_SUCCESS_RATE: f64 = 0.6;

    let mut rng = rand::thread_rng();
    let mut oks = 0;
    let mut fails = 0;
    for n in MIN_LENGTH..MAX_LENGTH {
        for _ in 0..NUM_ITERATIONS {
            let array: Vec<i64> = (0..n).map(|_| rng.gen_range(0..65_536)).collect();
            let sort_time = measure_sort_time(&array[..], SAMPLES);
            if sort_time.insertion_sort_time < sort_time.sort_time
                && sort_time.insertion_sort_time < sort_time.sort_unstable_time
            {
                oks += 1;
            } else {
                fails += 1;
            }
        }
    }
    let success_rate = oks as f64 / (oks + fails) as f64;
    assert!(
        success_rate >= MIN_SUCCESS_RATE,
        "Success rate {} excees minimal success rate {}",
        success_rate,
        MIN_SUCCESS_RATE
    );
}

#[test]
fn test_insertion_sort_longer_on_large_arrays() {
    use rand::Rng;

    const SAMPLES: usize = 16;
    const MIN_LENGTH: usize = 256;
    const MAX_LENGTH: usize = 288;
    const NUM_ITERATIONS: usize = 4;
    const MIN_SUCCESS_RATE: f64 = 1.0;

    let mut rng = rand::thread_rng();
    let mut oks = 0;
    let mut fails = 0;
    for n in MIN_LENGTH..MAX_LENGTH {
        for _ in 0..NUM_ITERATIONS {
            let array: Vec<i64> = (0..n).map(|_| rng.gen_range(0..65_536)).collect();
            let sort_time = measure_sort_time(&array[..], SAMPLES);
            if sort_time.insertion_sort_time > sort_time.sort_time
                && sort_time.insertion_sort_time > sort_time.sort_unstable_time
            {
                oks += 1;
            } else {
                fails += 1;
            }
        }
    }
    let success_rate = oks as f64 / (oks + fails) as f64;
    assert!(
        success_rate >= MIN_SUCCESS_RATE,
        "Success rate {} excees minimal success rate {}",
        success_rate,
        MIN_SUCCESS_RATE
    );
}

struct SortTime {
    sort_time: u64,
    sort_unstable_time: u64,
    insertion_sort_time: u64,
}

fn measure_sort_time(slice: &[i64], samples: usize) -> SortTime {
    SortTime {
        sort_time: rdtsc_perf(
            || slice.to_vec(),
            |mut input| {
                #[allow(clippy::stable_sort_primitive)]
                input.sort();
                input
            },
            samples,
        ),
        sort_unstable_time: rdtsc_perf(
            || slice.to_vec(),
            |mut input| {
                input.sort_unstable();
                input
            },
            samples,
        ),
        insertion_sort_time: rdtsc_perf(
            || slice.to_vec(),
            |mut input| {
                input.insertion_sort();
                input
            },
            samples,
        ),
    }
}

// ========

#[derive(Clone, Debug)]
pub struct FullPrimesSieve(Vec<usize>);

impl FullPrimesSieve {
    pub fn new(len: usize) -> Self {
        let mut data = vec![0; len];
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
            if self.0.sieve()[self.1] != 0 {
                self.1 += 1;
            } else {
                let item = self.1;
                self.1 += 1;
                return Some(item);
            }
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
            if divisor != 0 {
                self.1 /= divisor;
                Some(divisor)
            } else {
                let item = self.1;
                self.1 = 1;
                Some(item)
            }
        } else {
            None
        }
    }
}

impl FusedIterator for FullPrimesSieveFactorizeIter<'_> {}
