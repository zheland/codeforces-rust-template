use core::iter::repeat;

use crate::extensions::collection_bit_vec::BitVec;
use crate::into_vec::IntoVec;

#[test]
fn test_bitvec() {
    let as_vec = |bitvec: &BitVec| -> Vec<i8> {
        (0..bitvec.len()).map(|j| i8::from(bitvec.get(j))).collect()
    };

    let bitvec = BitVec::new();
    assert_eq!(as_vec(&bitvec), []);

    let mut bitvec: BitVec = repeat(false).take(80).collect();
    assert_eq!(
        as_vec(&bitvec),
        [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
        ]
    );
    bitvec.extend(repeat(true).take(7));
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

    let mut bitvec: BitVec = repeat(false).take(145).collect();
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
    use core::iter::repeat;

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
            assert!(!bitvec.get(j));
            bitvec.set(j, true);
            assert!(bitvec.get(j));
        }
        let mut bitvec: BitVec = (0..len).map(|j| j & 1 == 1).collect();
        for j1 in 0..len {
            #[allow(clippy::cast_possible_truncation)]
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
