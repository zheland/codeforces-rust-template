use std::collections::BTreeSet;

use crate::extensions::rooted_tree::{root_tree, rooted_tree_path};
use crate::extensions::rooted_tree_with_binary_lifting::{
    root_tree_with_binary_lifting, rooted_tree_with_binary_lifting_lca,
};

#[test]
fn test_root_tree_with_binary_lifting() {
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
        (11, 12),
        (12, 13),
        (13, 14),
        (14, 15),
    ]
    .into_iter()
    .flat_map(|(u, v)| [(u, v), (v, u)])
    .collect();

    let mut pars = vec![[0; 4]; 16];
    let mut lvls = vec![0; 16];

    root_tree_with_binary_lifting([0; 4], 0, 0, &uvs, &mut pars, &mut lvls);
    assert_eq!(
        pars,
        [
            [0, 0, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
            [1, 0, 0, 0],
            [1, 0, 0, 0],
            [2, 0, 0, 0],
            [2, 0, 0, 0],
            [5, 5, 0, 0],
            [5, 5, 0, 0],
            [5, 5, 0, 0],
            [8, 5, 0, 0],
            [8, 5, 0, 0],
            [11, 11, 11, 0],
            [12, 11, 11, 0],
            [13, 13, 11, 0],
            [14, 13, 11, 0],
        ]
    );
    assert_eq!(lvls, [0, 1, 1, 2, 2, 2, 2, 3, 3, 3, 4, 4, 5, 6, 7, 8]);

    root_tree_with_binary_lifting([11; 4], 11, 0, &uvs, &mut pars, &mut lvls);
    assert_eq!(
        pars,
        [
            [2, 5, 11, 11],
            [0, 0, 0, 11],
            [5, 5, 11, 11],
            [1, 0, 0, 11],
            [1, 0, 0, 11],
            [8, 11, 11, 11],
            [2, 5, 11, 11],
            [5, 5, 11, 11],
            [11, 11, 11, 11],
            [5, 5, 11, 11],
            [8, 11, 11, 11],
            [11, 11, 11, 11],
            [11, 11, 11, 11],
            [12, 11, 11, 11],
            [13, 13, 11, 11],
            [14, 13, 11, 11],
        ]
    );
    assert_eq!(lvls, [4, 5, 3, 6, 6, 2, 4, 3, 1, 3, 2, 0, 1, 2, 3, 4]);
}

#[allow(clippy::similar_names)]
#[test]
fn test_rooted_tree_with_binary_lifting_lca() {
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
        (11, 12),
        (12, 13),
        (13, 14),
        (14, 15),
    ]
    .into_iter()
    .flat_map(|(u, v)| [(u, v), (v, u)])
    .collect();

    for root in 0..16 {
        let mut pars = vec![[0; 4]; 16];
        let mut lvls = vec![0; 16];
        root_tree_with_binary_lifting([root; 4], root, 0, &uvs, &mut pars, &mut lvls);

        let mut pars2 = vec![0; 16];
        let mut lvls2 = vec![0; 16];
        root_tree(root, root, 0, &uvs, &mut pars2, &mut lvls2);

        for (u, v) in [(0, 0), (5, 8), (0, 4), (4, 0), (7, 6), (1, 11), (3, 10)] {
            let mid = rooted_tree_with_binary_lifting_lca(u, v, &pars, &lvls);

            let mut upars = vec![];
            let mut vpars = vec![];
            let mid2 = rooted_tree_path(u, v, &pars2, &lvls, &mut upars, &mut vpars);

            assert_eq!(mid, mid2);
        }
    }
}
