use crate::extensions::math_permutations::{k_permutations, permutations};
use crate::extensions::util_each_permutation::EachPermutation;
use crate::into_vec::IntoVec;

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

    #[allow(clippy::cast_possible_truncation)]
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
            #[allow(clippy::cast_possible_truncation)]
            if k <= n {
                assert_eq!(num_combs, k_permutations::<u64>(k, n) as usize);
            } else {
                assert_eq!(num_combs, 0);
            }
        }
    }
}
