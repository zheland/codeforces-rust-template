use core::iter::once;
use std::collections::BTreeSet;

use crate::extensions::rooted_tree::{root_tree, rooted_tree_path};

#[test]
fn test_root_tree() {
    let uvs: BTreeSet<_> = [
        (0, 1),
        (0, 2),
        (1, 3),
        (1, 4),
        (2, 5),
        (2, 6),
        (5, 7),
        (5, 8),
        (5, 9),
        (8, 10),
        (8, 11),
    ]
    .into_iter()
    .flat_map(|(u, v)| [(u, v), (v, u)])
    .collect();

    let mut pars = vec![0; 12];
    let mut lvls = vec![0; 12];

    root_tree(0, 0, 0, &uvs, &mut pars, &mut lvls);
    assert_eq!(pars, [0, 0, 0, 1, 1, 2, 2, 5, 5, 5, 8, 8]);
    assert_eq!(lvls, [0, 1, 1, 2, 2, 2, 2, 3, 3, 3, 4, 4]);

    root_tree(11, 11, 0, &uvs, &mut pars, &mut lvls);
    assert_eq!(pars, [2, 0, 5, 1, 1, 8, 2, 5, 11, 5, 8, 11]);
    assert_eq!(lvls, [4, 5, 3, 6, 6, 2, 4, 3, 1, 3, 2, 0]);
}

#[allow(clippy::similar_names)]
#[test]
fn test_rooted_tree_path() {
    let uvs: BTreeSet<_> = [
        (0, 1),
        (0, 2),
        (1, 3),
        (1, 4),
        (2, 5),
        (2, 6),
        (5, 7),
        (5, 8),
        (5, 9),
        (8, 10),
        (8, 11),
    ]
    .into_iter()
    .flat_map(|(u, v)| [(u, v), (v, u)])
    .collect();

    for root in 0..12 {
        let mut pars = vec![0; 12];
        let mut lvls = vec![0; 12];
        root_tree(root, root, 0, &uvs, &mut pars, &mut lvls);

        for (u, v, expected) in [
            (0, 0, vec![0]),
            (5, 8, vec![5, 8]),
            (0, 4, vec![0, 1, 4]),
            (4, 0, vec![4, 1, 0]),
            (7, 6, vec![7, 5, 2, 6]),
            (1, 11, vec![1, 0, 2, 5, 8, 11]),
            (3, 10, vec![3, 1, 0, 2, 5, 8, 10]),
        ] {
            let mut upars = vec![];
            let mut vpars = vec![];
            let mid = rooted_tree_path(u, v, &pars, &lvls, &mut upars, &mut vpars);
            let path: Vec<_> = upars
                .iter()
                .copied()
                .chain(once(mid))
                .chain(vpars.iter().copied().rev())
                .collect();
            assert_eq!(
                path, expected,
                "with root {root}, {u} => {v}, [{upars:?}, {mid}, {vpars:?}]"
            );
        }
    }
}
