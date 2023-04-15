use crate::extensions::math_combinations::combinations;
use crate::extensions::util_each_combination::EachCombination;
use crate::into_vec::IntoVec;

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
            #[allow(clippy::cast_possible_truncation)]
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
