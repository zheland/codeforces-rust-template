use crate::extensions::{
    BitVec, ConstValue, Dijkstra, Each, EachCombination, EachPermutation, Modular, NdArray, NonMax,
    PrimeValue, ValueWithEulersPhi, M1_000_000_007,
};
use crate::{combinations, k_permutations, permutations, wrap, IntoVec, Primes};

#[test]
fn test_bitvec() {
    let as_vec =
        |bitvec: &BitVec| -> Vec<i8> { (0..bitvec.len()).map(|j| bitvec.get(j) as i8).collect() };

    let bitvec = BitVec::new();
    assert_eq!(as_vec(&bitvec), []);

    let mut bitvec: BitVec = (0..80).map(|_| false).collect();
    assert_eq!(
        as_vec(&bitvec),
        [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
        ]
    );
    bitvec.extend((0..7).map(|_| true));
    assert_eq!(
        as_vec(&bitvec),
        [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1,
        ]
    );
    bitvec.set(30, true);
    bitvec.set(31, true);
    bitvec.set(32, true);
    bitvec.set(33, true);
    bitvec.set(86, false);
    bitvec.set(85, false);
    assert_eq!(
        as_vec(&bitvec),
        [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0,
        ]
    );
    bitvec.set_range(2..4, true);
    bitvec.set_range(32..34, false);
    bitvec.set_range(62..66, true);
    assert_eq!(
        as_vec(&bitvec),
        [
            0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0,
        ]
    );
    bitvec.set_range(0..29, false);
    bitvec.set_range(29..58, true);
    bitvec.set_range(58..87, false);
    assert_eq!(
        as_vec(&bitvec),
        [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]
    );

    let mut bitvec: BitVec = (0..145).map(|_| false).collect();
    bitvec.set_range(29..116, true);
    assert_eq!(
        as_vec(&bitvec),
        [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]
    );
    bitvec.set_range(31..114, false);
    assert_eq!(
        as_vec(&bitvec),
        [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]
    );
    bitvec.set_range(33..112, true);
    assert_eq!(
        as_vec(&bitvec),
        [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]
    );
}

#[test]
fn test_bitvec_eq_vec() {
    use std::iter::repeat;

    for tmpl in &[
        vec![false],
        vec![true],
        vec![false, true, true],
        vec![true, false, false, false, false],
    ] {
        let vec: Vec<bool> = repeat(tmpl.clone()).flatten().take(80).collect();
        let bitvec: BitVec = repeat(tmpl.clone()).flatten().take(80).collect();
        for &j1 in &[0, 1, 2, 30, 31, 32, 33, 34, 62, 63, 64, 65, 66] {
            for &v1 in &[false, true] {
                let mut vec = vec.clone();
                let mut bitvec = bitvec.clone();
                vec.resize(j1, v1);
                bitvec.resize(j1, v1);

                for &j2 in &[0, 1, 2, 30, 31, 32, 33, 34, 62, 63, 64, 65, 66] {
                    for &v2 in &[false, true] {
                        let mut vec = vec.clone();
                        let mut bitvec = bitvec.clone();
                        vec.resize(j2, v2);
                        bitvec.resize(j2, v2);
                        assert_eq!(vec, bitvec.into_vec());
                    }
                }
                for &v2 in &[false, true] {
                    let mut vec = vec.clone();
                    let mut bitvec = bitvec.clone();
                    vec.push(v2);
                    bitvec.push(v2);
                    assert_eq!(vec, bitvec.into_vec());
                }
                {
                    let mut vec = vec.clone();
                    let mut bitvec = bitvec.clone();
                    assert_eq!(vec.pop(), bitvec.pop());
                    assert_eq!(vec, bitvec.into_vec());
                }

                for &j2 in &[0, 1, 2, 30, 31, 32, 33, 34, 62, 63, 64, 65, 66] {
                    for &v2 in &[false, true] {
                        for &j3 in &[0, 1, 2, 30, 31, 32, 33, 34, 62, 63, 64, 65, 66] {
                            let mut vec = vec.clone();
                            let mut bitvec = bitvec.clone();
                            if j2 < vec.len() {
                                vec[j2] = v2;
                            }
                            assert_eq!(bitvec.try_set(j2, v2).is_ok(), j2 < vec.len());
                            assert_eq!(vec.get(j3).copied(), bitvec.try_get(j3));
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn test_bitvec_count_ones() {
    let mut bitvec: BitVec = (0..300).map(|j| j & 1 == 1).collect();
    assert_eq!(bitvec.count_ones(100..104), 2);
    assert_eq!(bitvec.count_ones(100..105), 2);
    assert_eq!(bitvec.count_ones(100..106), 3);
    assert_eq!(bitvec.count_ones(100..107), 3);
    assert_eq!(bitvec.count_ones(101..107), 3);
    assert_eq!(bitvec.count_ones(102..107), 2);
    assert_eq!(bitvec.count_ones(126..130), 2);
    assert_eq!(bitvec.count_ones(127..130), 2);
    assert_eq!(bitvec.count_ones(128..130), 1);
    assert_eq!(bitvec.count_ones(129..130), 1);
    assert_eq!(bitvec.count_ones(130..130), 0);
    assert_eq!(bitvec.count_ones(126..130), 2);
    assert_eq!(bitvec.count_ones(126..129), 1);
    assert_eq!(bitvec.count_ones(126..128), 1);
    assert_eq!(bitvec.count_ones(126..127), 0);
    assert_eq!(bitvec.count_ones(126..126), 0);
    assert_eq!(bitvec.count_ones(126..255), 64);
    assert_eq!(bitvec.count_ones(126..256), 65);
    assert_eq!(bitvec.count_ones(126..257), 65);
    assert_eq!(bitvec.count_ones(126..258), 66);
    assert_eq!(bitvec.count_ones(127..255), 64);
    assert_eq!(bitvec.count_ones(127..256), 65);
    assert_eq!(bitvec.count_ones(127..257), 65);
    assert_eq!(bitvec.count_ones(127..258), 66);
    assert_eq!(bitvec.count_ones(128..255), 63);
    assert_eq!(bitvec.count_ones(128..256), 64);
    assert_eq!(bitvec.count_ones(128..257), 64);
    assert_eq!(bitvec.count_ones(128..258), 65);
    assert_eq!(bitvec.count_ones(129..255), 63);
    assert_eq!(bitvec.count_ones(129..256), 64);
    assert_eq!(bitvec.count_ones(129..257), 64);
    assert_eq!(bitvec.count_ones(129..258), 65);
}

#[test]
fn test_bitvec_ops() {
    for &len in &[0, 1, 10, 31, 32, 33, 60, 61, 62, 63, 64, 100, 1000] {
        let mut bitvec: BitVec = (0..len).map(|j| j & 1 == 1).collect();
        for j in 0..len {
            bitvec.set(j, false);
            assert_eq!(bitvec.get(j), false);
            bitvec.set(j, true);
            assert_eq!(bitvec.get(j), true);
        }
        let mut bitvec: BitVec = (0..len).map(|j| j & 1 == 1).collect();
        for j1 in 0..len {
            for j2 in 0..=len {
                bitvec.set_range(j1..j2, false);
                assert_eq!(bitvec.count_ones(j1..j2), 0);
                bitvec.set_range(j1..j2, true);
                assert_eq!(bitvec.count_ones(j1..j2), j2.saturating_sub(j1) as u32);
            }
        }
        for _ in 0..len {
            let _ = bitvec.pop();
        }
        assert!(bitvec.is_empty());
    }
}

#[test]
fn test_each() {
    let vec: Vec<_> = [1..=2, 1..=3, 1..=4].each().collect();
    assert_eq!(
        vec,
        [
            [1, 1, 1],
            [1, 1, 2],
            [1, 1, 3],
            [1, 1, 4],
            [1, 2, 1],
            [1, 2, 2],
            [1, 2, 3],
            [1, 2, 4],
            [1, 3, 1],
            [1, 3, 2],
            [1, 3, 3],
            [1, 3, 4],
            [2, 1, 1],
            [2, 1, 2],
            [2, 1, 3],
            [2, 1, 4],
            [2, 2, 1],
            [2, 2, 2],
            [2, 2, 3],
            [2, 2, 4],
            [2, 3, 1],
            [2, 3, 2],
            [2, 3, 3],
            [2, 3, 4],
        ]
    );
}

#[test]
fn test_each_vec() {
    let vec: Vec<_> = vec![1..=2; 3].each().collect();
    assert_eq!(
        vec,
        [
            [1, 1, 1],
            [1, 1, 2],
            [1, 2, 1],
            [1, 2, 2],
            [2, 1, 1],
            [2, 1, 2],
            [2, 2, 1],
            [2, 2, 2],
        ]
    );
}

#[test]
fn test_each_combination() {
    let test = |vals: &[i32], perms: &[&[i32]]| {
        assert_eq!(
            vals.each_combination()
                .map(|v| v.copied().into_vec())
                .into_vec(),
            perms
        );
    };

    test(&[], &[&[]]);
    test(&[1], &[&[], &[1]]);
    test(&[1, 2], &[&[], &[1], &[2], &[1, 2]]);
    test(
        &[1, 2, 3],
        &[&[], &[1], &[2], &[1, 2], &[3], &[1, 3], &[2, 3], &[1, 2, 3]],
    );
}

#[test]
fn test_each_k_combination() {
    let test = |vals: &[i32], k: usize, perms: &[&[i32]]| {
        assert_eq!(
            vals.each_k_combination(k)
                .map(|v| v.copied().into_vec())
                .into_vec(),
            perms
        );
    };

    test(&[], 0, &[&[]]);
    test(&[], 3, &[]);
    test(&[], 10, &[]);

    test(&[1, 2, 3, 4, 5], 0, &[&[]]);
    test(
        &[1, 2, 3, 4, 5],
        3,
        &[
            &[1, 2, 3],
            &[1, 2, 4],
            &[1, 3, 4],
            &[2, 3, 4],
            &[1, 2, 5],
            &[1, 3, 5],
            &[2, 3, 5],
            &[1, 4, 5],
            &[2, 4, 5],
            &[3, 4, 5],
        ],
    );
    test(&[1, 2, 3, 4, 5], 10, &[]);

    for n in 0..8 {
        for k in 0..8 {
            let vals = (0..n).into_vec();
            let num_combs = vals.each_k_combination(k).count();
            if k <= n {
                assert_eq!(num_combs, combinations::<u64>(k, n) as usize);
            } else {
                assert_eq!(num_combs, 0);
            }
        }
    }
}

#[test]
fn test_each_ks_combination() {
    let test = |vals: &[i32], k: (usize, usize), perms: &[&[i32]]| {
        assert_eq!(
            vals.each_ks_combination(k.0, k.1)
                .map(|v| v.copied().into_vec())
                .into_vec(),
            perms
        );
    };

    test(
        &[1, 2, 3, 4],
        (2, 3),
        &[
            &[1, 2],
            &[1, 3],
            &[2, 3],
            &[1, 2, 3],
            &[1, 4],
            &[2, 4],
            &[1, 2, 4],
            &[3, 4],
            &[1, 3, 4],
            &[2, 3, 4],
        ],
    );
}

#[test]
fn test_each_permutation() {
    let test = |vals: &[i32], perms: &[&[i32]]| {
        assert_eq!(
            vals.each_permutation()
                .map(|v| v.get().iter().copied().copied().into_vec())
                .into_vec(),
            perms
        );
    };

    test(
        &[1, 2, 3],
        &[
            &[1, 2, 3],
            &[1, 3, 2],
            &[2, 1, 3],
            &[2, 3, 1],
            &[3, 1, 2],
            &[3, 2, 1],
        ],
    );

    test(&[], &[&[]]);

    for n in 0..8 {
        let vals = (0..n).into_vec();
        let num_combs = vals.each_permutation().count();
        assert_eq!(num_combs, permutations::<u64>(n) as usize);
    }
}

#[test]
fn test_each_k_permutation() {
    let test = |vals: &[i32], k: usize, perms: &[&[i32]]| {
        assert_eq!(
            vals.each_k_permutation(k)
                .map(|v| v.get().iter().copied().copied().into_vec())
                .into_vec(),
            perms
        );
    };

    test(
        &[1, 2, 3],
        2,
        &[&[1, 2], &[1, 3], &[2, 1], &[2, 3], &[3, 1], &[3, 2]],
    );

    test(&[1, 2, 3], 0, &[&[]]);
    test(&[], 0, &[&[]]);
    test(&[1, 2, 3], 10, &[]);

    for n in 0..8 {
        for k in 0..8 {
            let vals = (0..n).into_vec();
            let num_combs = vals.each_k_permutation(k).count();
            if k <= n {
                assert_eq!(num_combs, k_permutations::<u64>(k, n) as usize);
            } else {
                assert_eq!(num_combs, 0);
            }
        }
    }
}

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
    v += 1000000006;
    assert_eq!(v.value(), 1000000006);
    v += 1;
    assert_eq!(v.value(), 0);
}

#[test]
fn test_mod_pow() {
    let v = M1_000_000_007::from(123);
    assert_eq!(v.pow(3).value(), 1860867);
    assert_eq!(v.pow(10).value(), 26898250);
}

#[test]
fn test_mod_inv() {
    let v = M1_000_000_007::from(123);

    let a1 = v.pow(1);
    let b1 = a1.inv().unwrap();
    assert_eq!(a1.value(), 123);
    assert_eq!(b1.value(), 886178868);
    assert_eq!((a1 * b1).value(), 1);

    let a3 = v.pow(3);
    let b3 = a3.inv().unwrap();
    assert_eq!(a3.value(), 1860867);
    assert_eq!(b3.value(), 939777003);
    assert_eq!((a3 * b3).value(), 1);

    let a10 = v.pow(10);
    let b10 = a10.inv().unwrap();
    assert_eq!(a10.value(), 26898250);
    assert_eq!(b10.value(), 408060267);
    assert_eq!((a10 * b10).value(), 1);
}

#[test]
fn test_dimview2() {
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    let view = NdArray::from(&arr[..], [3, 4]);

    assert_eq!(view.at(0)[&[0]], 1);
    assert_eq!(view.at(0)[&[1]], 2);
    assert_eq!(view.at(0)[&[2]], 3);
    assert_eq!(view.at(0)[&[3]], 4);
    assert_eq!(view.at(1)[&[0]], 5);
    assert_eq!(view.at(1)[&[1]], 6);
    assert_eq!(view.at(1)[&[2]], 7);
    assert_eq!(view.at(1)[&[3]], 8);
    assert_eq!(view.at(2)[&[0]], 9);
    assert_eq!(view.at(2)[&[1]], 10);
    assert_eq!(view.at(2)[&[2]], 11);
    assert_eq!(view.at(2)[&[3]], 12);

    assert_eq!(view[[0, 0]], 1);
    assert_eq!(view[[0, 1]], 2);
    assert_eq!(view[[0, 2]], 3);
    assert_eq!(view[[0, 3]], 4);
    assert_eq!(view[[1, 0]], 5);
    assert_eq!(view[[1, 1]], 6);
    assert_eq!(view[[1, 2]], 7);
    assert_eq!(view[[1, 3]], 8);
    assert_eq!(view[[2, 0]], 9);
    assert_eq!(view[[2, 1]], 10);
    assert_eq!(view[[2, 2]], 11);
    assert_eq!(view[[2, 3]], 12);

    assert_eq!(view.get([0, 0]), Some(&1));
    assert_eq!(view.get([(wrap(0usize) - wrap(1)).0, 0]), None);
    assert_eq!(view.get([0, (wrap(0usize) - wrap(1)).0]), None);
    assert_eq!(
        view.get([(wrap(0usize) - wrap(1)).0, (wrap(0usize) - wrap(1)).0]),
        None
    );
}

#[test]
fn test_dimview3_at() {
    let arr = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
    ];
    let view = NdArray::from(&arr[..], [4, 3, 2]);

    assert_eq!(view.at(0).at(0)[&[0]], 1);
    assert_eq!(view.at(0).at(0)[&[1]], 2);
    assert_eq!(view.at(0).at(1)[&[0]], 3);
    assert_eq!(view.at(0).at(1)[&[1]], 4);
    assert_eq!(view.at(0).at(2)[&[0]], 5);
    assert_eq!(view.at(0).at(2)[&[1]], 6);
    assert_eq!(view.at(1).at(0)[&[0]], 7);
    assert_eq!(view.at(1).at(0)[&[1]], 8);
    assert_eq!(view.at(1).at(1)[&[0]], 9);
    assert_eq!(view.at(1).at(1)[&[1]], 10);
    assert_eq!(view.at(1).at(2)[&[0]], 11);
    assert_eq!(view.at(1).at(2)[&[1]], 12);
    assert_eq!(view.at(2).at(0)[&[0]], 13);
    assert_eq!(view.at(2).at(0)[&[1]], 14);
    assert_eq!(view.at(2).at(1)[&[0]], 15);
    assert_eq!(view.at(2).at(1)[&[1]], 16);
    assert_eq!(view.at(2).at(2)[&[0]], 17);
    assert_eq!(view.at(2).at(2)[&[1]], 18);
    assert_eq!(view.at(3).at(0)[&[0]], 19);
    assert_eq!(view.at(3).at(0)[&[1]], 20);
    assert_eq!(view.at(3).at(1)[&[0]], 21);
    assert_eq!(view.at(3).at(1)[&[1]], 22);
    assert_eq!(view.at(3).at(2)[&[0]], 23);
    assert_eq!(view.at(3).at(2)[&[1]], 24);
}

#[test]
fn test_dimview3_index() {
    let arr = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
    ];
    let view = NdArray::from(&arr[..], [4, 3, 2]);

    assert_eq!(view[[0, 0, 0]], 1);
    assert_eq!(view[[0, 0, 1]], 2);
    assert_eq!(view[[0, 1, 0]], 3);
    assert_eq!(view[[0, 1, 1]], 4);
    assert_eq!(view[[0, 2, 0]], 5);
    assert_eq!(view[[0, 2, 1]], 6);
    assert_eq!(view[[1, 0, 0]], 7);
    assert_eq!(view[[1, 0, 1]], 8);
    assert_eq!(view[[1, 1, 0]], 9);
    assert_eq!(view[[1, 1, 1]], 10);
    assert_eq!(view[[1, 2, 0]], 11);
    assert_eq!(view[[1, 2, 1]], 12);
    assert_eq!(view[[2, 0, 0]], 13);
    assert_eq!(view[[2, 0, 1]], 14);
    assert_eq!(view[[2, 1, 0]], 15);
    assert_eq!(view[[2, 1, 1]], 16);
    assert_eq!(view[[2, 2, 0]], 17);
    assert_eq!(view[[2, 2, 1]], 18);
    assert_eq!(view[[3, 0, 0]], 19);
    assert_eq!(view[[3, 0, 1]], 20);
    assert_eq!(view[[3, 1, 0]], 21);
    assert_eq!(view[[3, 1, 1]], 22);
    assert_eq!(view[[3, 2, 0]], 23);
    assert_eq!(view[[3, 2, 1]], 24);
}

/*#[test]
fn test_dimview3_range() {
    let arr = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
    ];
    let view = NdArray::from(&arr[..], [4, 3, 2]);

    assert_eq!(view.range(1..3)[[0, 0, 0]], 7);
    assert_eq!(view.range(1..3)[[0, 0, 1]], 8);
    assert_eq!(view.range(1..3)[[0, 1, 0]], 9);
    assert_eq!(view.range(1..3)[[0, 1, 1]], 10);
    assert_eq!(view.range(1..3)[[0, 2, 0]], 11);
    assert_eq!(view.range(1..3)[[0, 2, 1]], 12);
    assert_eq!(view.range(1..3)[[1, 0, 0]], 13);
    assert_eq!(view.range(1..3)[[1, 0, 1]], 14);
    assert_eq!(view.range(1..3)[[1, 1, 0]], 15);
    assert_eq!(view.range(1..3)[[1, 1, 1]], 16);
    assert_eq!(view.range(1..3)[[1, 2, 0]], 17);
    assert_eq!(view.range(1..3)[[1, 2, 1]], 18);
}*/

#[test]
fn test_dimview3_mut() {
    let mut arr = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
    ];

    let mut view = NdArray::from(&mut arr[..], [4, 3, 2]);

    view[[1, 1, 1]] = 100;
    view.at_mut(3).at_mut(2)[&[1]] = 200;

    assert_eq!(view.at(1).at(1)[&[1]], 100);
    assert_eq!(view[[3, 2, 1]], 200);
}

#[test]
#[should_panic]
fn test_dimview2_size_check() {
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    let _ = NdArray::from(&arr[..], [4, 4]);
}

#[test]
#[should_panic]
fn test_dimview3_size_check() {
    let arr = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
    ];
    let _ = NdArray::from(&arr[..], [4, 3, 3]);
}

#[test]
#[should_panic]
fn test_dimview3_index1_check() {
    let arr = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
    ];
    let view = NdArray::from(&arr[..], [4, 3, 2]);
    let _ = view.at(4);
}

#[test]
#[should_panic]
fn test_dimview3_index2_check() {
    let arr = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
    ];
    let view = NdArray::from(&arr[..], [4, 3, 2]);
    let _ = view.at(3).at(3);
}

#[test]
#[should_panic]
fn test_dimview3_index3_check() {
    let arr = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
    ];
    let view = NdArray::from(&arr[..], [4, 3, 2]);
    let _ = view.at(2).at(2)[&[2]];
}

#[test]
fn test_btreemap_dijkstra() {
    use core::iter::once;
    use std::collections::BTreeMap;

    let g: BTreeMap<_, _> = vec![
        ((0, 1), 9),
        ((0, 2), 2),
        ((0, 4), 14),
        ((1, 3), 6),
        ((2, 3), 11),
        ((2, 4), 9),
        ((2, 5), 10),
        ((3, 5), 15),
        ((4, 5), 7),
        //
        ((1, 0), 9),
        ((2, 0), 2),
        ((4, 0), 14),
        ((3, 1), 6),
        ((3, 2), 11),
        ((4, 2), 9),
        ((5, 2), 10),
        ((5, 3), 15),
        ((5, 4), 7),
    ]
    .into_iter()
    .collect();

    let _ = Dijkstra::new(6, once((4, 0)), |n, l| {
        Some(g.range((n, 0)..(n + 1, 0)).map(move |n| (n.0 .1, l + n.1)))
    });
    let d = Dijkstra::new(6, once((4, 0)), |n, l| {
        Some(g.range((n, 0)..(n + 1, 0)).map(move |n| (n.0 .1, l + n.1)))
    });
    let _ = g.into_iter().count();

    assert_eq!(d.nodes, &[4, 5, 2, 0, 1, 3]);
    assert_eq!(
        d.dists,
        &[Some(11), Some(20), Some(9), Some(20), Some(0), Some(7)]
    );
    assert_eq!(
        d.prevs,
        &[
            NonMax::from(2),
            NonMax::from(0),
            NonMax::from(4),
            NonMax::from(2),
            NonMax::none(),
            NonMax::from(4)
        ]
    );
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
            let m = ValueWithEulersPhi::new(m, primes.eulers_phi(m as usize) as u32);
            let v1 = Modular::from_raw(n, m);
            let v2 = v1.inv_with_eulers_phi();
            if let Some(v2) = v2 {
                if n != 0 {
                    assert_eq!(v1 * v2, Modular::from_raw(1, m));
                } else {
                    assert_eq!(v2, Modular::from_raw(0, m));
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
